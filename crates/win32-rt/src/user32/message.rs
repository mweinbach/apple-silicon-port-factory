//! Per-thread message queues: Get/Peek/Post/Send/Dispatch, timers,
//! `MsgWaitForMultipleObjectsEx`.
//!
//! Queue state lives under the global kernel wait lock so a queue is just
//! another waitable for the generic wait machinery. Lock order: window data
//! locks may be held while taking the wait lock, never the reverse; no lock
//! is ever held while guest code (a window procedure) runs.
//!
//! Wake semantics follow Windows: `wake` collects the QS_* bits of
//! everything queued since the owning thread last scanned its queue, and a
//! blocking `GetMessage`/`WaitMessage`/`MsgWaitForMultipleObjects` sleeps
//! until a new bit arrives — messages that are already queued but filtered
//! out do not cause busy loops.

use super::*;
use crate::handles::{self, Guarded, KObject, WaitGuard, WaitResult, Waitable};
use crate::types::*;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, OnceLock};

pub const QS_KEY: u32 = 0x0001;
pub const QS_MOUSEMOVE: u32 = 0x0002;
pub const QS_MOUSEBUTTON: u32 = 0x0004;
pub const QS_POSTMESSAGE: u32 = 0x0008;
pub const QS_TIMER: u32 = 0x0010;
pub const QS_PAINT: u32 = 0x0020;
pub const QS_SENDMESSAGE: u32 = 0x0040;
pub const QS_HOTKEY: u32 = 0x0080;
pub const QS_ALLPOSTMESSAGE: u32 = 0x0100;
pub const QS_RAWINPUT: u32 = 0x0400;
pub const QS_MOUSE: u32 = QS_MOUSEMOVE | QS_MOUSEBUTTON;
pub const QS_INPUT: u32 = QS_MOUSE | QS_KEY | QS_RAWINPUT;
pub const QS_ALLINPUT: u32 = QS_INPUT | QS_POSTMESSAGE | QS_TIMER | QS_PAINT | QS_HOTKEY | QS_SENDMESSAGE;

pub const PM_NOREMOVE: u32 = 0;
pub const PM_REMOVE: u32 = 1;

/// One queued hardware-input message (+ the characters AppKit produced for
/// a key press, consumed by `TranslateMessage`).
#[derive(Clone, Debug, Default)]
pub struct InputMsg {
    pub msg: MSG,
    pub chars: Vec<u16>,
    /// True once low-level hooks (`WH_KEYBOARD_LL` / `WH_MOUSE_LL`) have
    /// already been evaluated for this hardware event.
    pub ll_checked: bool,
    /// Monotonic queue sequence assigned by [`Queue::post_input`].
    pub(crate) seq: u64,
}

#[derive(Clone, Copy, Debug)]
struct PostedMsg {
    msg: MSG,
    seq: u64,
}

static NEXT_MSG_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// A cross-thread `SendMessage`, or an internal closure the owning thread
/// must run (native window events).
pub enum Sent {
    Msg { hwnd: usize, msg: u32, wparam: WPARAM, lparam: LPARAM, reply: Option<Arc<Reply>> },
    Call(Box<dyn FnOnce() + Send>),
}

pub struct Reply {
    result: Guarded<Option<LRESULT>>,
}

impl KObject for Reply {
    fn type_name(&self) -> &'static str {
        "SendMessageReply"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn waitable(&self) -> Option<&dyn Waitable> {
        Some(self)
    }
}
impl Waitable for Reply {
    fn is_signaled(&self, g: &WaitGuard, _tid: u32) -> bool {
        self.result.get(g).is_some()
    }
    fn acquire(&self, _g: &WaitGuard, _tid: u32) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug)]
struct Timer {
    hwnd: usize,
    id: usize,
    interval: u64,
    due: u64,
    proc_: usize,
}

#[derive(Default)]
struct QState {
    sent: VecDeque<Sent>,
    posted: VecDeque<PostedMsg>,
    input: VecDeque<InputMsg>,
    quit: Option<i32>,
    wake: u32,
    timers: Vec<Timer>,
    paint: Vec<usize>,
}

pub struct Queue {
    pub tid: u32,
    st: Guarded<QState>,
}

fn queues() -> &'static Mutex<HashMap<u32, Arc<Queue>>> {
    static Q: OnceLock<Mutex<HashMap<u32, Arc<Queue>>>> = OnceLock::new();
    Q.get_or_init(|| Mutex::new(HashMap::new()))
}

thread_local! {
    static CURRENT: RefCell<Option<Arc<Queue>>> = const { RefCell::new(None) };
    /// Characters of the key message most recently retrieved on this thread.
    static LAST_KEY: RefCell<Option<(MSG, Vec<u16>)>> = const { RefCell::new(None) };
    /// (time, pos) of the message most recently retrieved (GetMessageTime/Pos).
    static LAST_MSG: std::cell::Cell<(u32, POINT)> = const { std::cell::Cell::new((0, POINT { x: 0, y: 0 })) };
    /// Active stack of incoming inter-thread sent messages being processed on this thread.
    /// Each slot holds `Some(reply)` until answered early via `ReplyMessage` (which takes it
    /// to `None` and wakes the sender immediately) or when the window procedure returns.
    static ACTIVE_REPLIES: RefCell<Vec<Option<Arc<Reply>>>> = const { RefCell::new(Vec::new()) };
}

fn now_ms() -> u64 {
    crate::kernel32::time::interrupt_time_100ns() / 10_000
}

/// The calling Win32 thread's queue, created on first use (as Windows
/// creates it on the first user32 call that needs one).
pub fn current_queue() -> Arc<Queue> {
    let tid = crate::teb::thread_id();
    CURRENT.with(|c| {
        if let Some(q) = &*c.borrow() {
            if q.tid == tid {
                return q.clone();
            }
        }
        let q = queues()
            .lock()
            .unwrap()
            .entry(tid)
            .or_insert_with(|| Arc::new(Queue { tid, st: Guarded::new(QState::default()) }))
            .clone();
        *c.borrow_mut() = Some(q.clone());
        q
    })
}

/// Queue of thread `tid`, if that thread has one.
pub fn queue_of(tid: u32) -> Option<Arc<Queue>> {
    queues().lock().unwrap().get(&tid).cloned()
}

fn thread_alive(tid: u32) -> bool {
    match crate::thread::find_by_tid(tid) {
        Some(t) => t.exit_code().is_none(),
        // Adopted / initial threads may not be in the thread table; assume alive.
        None => true,
    }
}

fn input_bits(m: u32) -> u32 {
    match m {
        wm::INPUT => QS_RAWINPUT,
        wm::MOUSEMOVE => QS_MOUSEMOVE,
        m if (wm::MOUSEFIRST..=wm::MOUSELAST).contains(&m) => QS_MOUSEBUTTON,
        _ => QS_KEY,
    }
}

impl Queue {
    fn push(&self, f: impl FnOnce(&mut QState) -> u32) {
        let g = handles::wait_lock();
        let st = self.st.get(&g);
        let bits = f(st);
        st.wake |= bits;
        drop(g);
        handles::notify_object_key(self as *const Queue as usize, usize::MAX);
    }

    pub fn post(&self, m: MSG) {
        let seq = NEXT_MSG_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.push(|st| {
            st.posted.push_back(PostedMsg { msg: m, seq });
            QS_POSTMESSAGE | QS_ALLPOSTMESSAGE
        });
    }

    /// Queue a hardware input message. Consecutive `WM_MOUSEMOVE`s for the
    /// same window and button state are coalesced, as on Windows.
    pub fn post_input(&self, mut m: InputMsg) {
        m.seq = NEXT_MSG_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.push(|st| {
            let bits = input_bits(m.msg.message);
            if m.msg.message == wm::MOUSEMOVE {
                if let Some(last) = st.input.back_mut() {
                    if last.msg.message == wm::MOUSEMOVE && last.msg.hwnd == m.msg.hwnd && last.msg.wparam == m.msg.wparam {
                        *last = m;
                        return bits;
                    }
                }
            }
            st.input.push_back(m);
            bits
        });
    }

    pub fn send_item(&self, s: Sent) {
        self.push(|st| {
            st.sent.push_back(s);
            QS_SENDMESSAGE
        });
    }

    /// Run `f` on the owning thread the next time it looks at its queue.
    pub fn call(&self, f: impl FnOnce() + Send + 'static) {
        self.send_item(Sent::Call(Box::new(f)));
    }

    pub fn post_quit(&self, code: i32) {
        self.push(|st| {
            st.quit = Some(code);
            QS_POSTMESSAGE | QS_ALLPOSTMESSAGE
        });
    }

    pub fn invalidate(&self, hwnd: usize) {
        self.push(|st| {
            if !st.paint.contains(&hwnd) {
                st.paint.push(hwnd);
            }
            QS_PAINT
        });
    }

    pub fn validate(&self, hwnd: usize) {
        let g = handles::wait_lock();
        self.st.get(&g).paint.retain(|&h| h != hwnd);
    }

    /// Forget everything queued for a destroyed window.
    pub fn purge_window(&self, hwnd: usize) {
        let g = handles::wait_lock();
        let st = self.st.get(&g);
        st.posted.retain(|m| m.msg.hwnd != hwnd);
        st.input.retain(|m| m.msg.hwnd != hwnd);
        st.paint.retain(|&h| h != hwnd);
        st.timers.retain(|t| t.hwnd != hwnd);
    }
}

/// `MsgWaitForMultipleObjects` participant: signaled when new messages of
/// the given kinds arrived since the last scan (or, with
/// `MWMO_INPUTAVAILABLE`, when any such message is queued).
struct QueueWaiter {
    q: Arc<Queue>,
    mask: u32,
    any_available: bool,
}

impl KObject for QueueWaiter {
    fn type_name(&self) -> &'static str {
        "MessageQueue"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn waitable(&self) -> Option<&dyn Waitable> {
        Some(self)
    }
}

impl Waitable for QueueWaiter {
    fn is_signaled(&self, g: &WaitGuard, _tid: u32) -> bool {
        let st = self.q.st.get(g);
        if !st.sent.is_empty() && self.mask & QS_SENDMESSAGE != 0 {
            return true;
        }
        if st.wake & self.mask != 0 {
            return true;
        }
        if self.any_available {
            return status_bits(st) & self.mask != 0;
        }
        false
    }
    fn acquire(&self, _g: &WaitGuard, _tid: u32) -> bool {
        false
    }
    fn wait_key(&self) -> usize {
        Arc::as_ptr(&self.q) as usize
    }
}

fn status_bits(st: &QState) -> u32 {
    let mut b = 0;
    if !st.sent.is_empty() {
        b |= QS_SENDMESSAGE;
    }
    if !st.posted.is_empty() || st.quit.is_some() {
        b |= QS_POSTMESSAGE | QS_ALLPOSTMESSAGE;
    }
    for m in &st.input {
        b |= input_bits(m.msg.message);
    }
    if !st.paint.is_empty() {
        b |= QS_PAINT;
    }
    let now = now_ms();
    if st.timers.iter().any(|t| t.due <= now) {
        b |= QS_TIMER;
    }
    b
}

/// Get/PeekMessage filter.
#[derive(Clone, Copy, Debug)]
pub struct Filter {
    pub hwnd: usize,
    pub min: u32,
    pub max: u32,
    /// QS_* categories (from PM_QS_*); QS_ALLINPUT when unrestricted.
    pub kinds: u32,
}

impl Filter {
    fn msg_ok(&self, m: u32) -> bool {
        (self.min == 0 && self.max == 0) || (m >= self.min && m <= self.max)
    }
    fn hwnd_ok(&self, h: usize) -> bool {
        match self.hwnd {
            0 => true,
            usize::MAX => h == 0, // (HWND)-1: thread messages only
            f => h == f || super::window::is_child_of(h, f),
        }
    }
    fn ok(&self, m: &MSG) -> bool {
        self.msg_ok(m.message) && self.hwnd_ok(m.hwnd)
    }
}

fn complete_reply(rep: Arc<Reply>, r: LRESULT) {
    let g = handles::wait_lock();
    *rep.result.get(&g) = Some(r);
    drop(g);
    handles::notify_object_key(&*rep as *const Reply as usize, usize::MAX);
}

fn run_sent(s: Sent) {
    match s {
        Sent::Call(f) => f(),
        Sent::Msg { hwnd, msg, wparam, lparam, reply } => {
            struct ReplyPopGuard;
            impl Drop for ReplyPopGuard {
                fn drop(&mut self) {
                    if let Some(Some(rep)) = ACTIVE_REPLIES.with(|v| v.borrow_mut().pop()) {
                        complete_reply(rep, 0);
                    }
                }
            }
            ACTIVE_REPLIES.with(|v| v.borrow_mut().push(reply));
            let guard = ReplyPopGuard;
            super::misc::dispatch_callwndproc_hook(hwnd, msg, wparam, lparam, false);
            let r = super::window::call_window_proc(hwnd, msg, wparam, lparam);
            let pending = ACTIVE_REPLIES.with(|v| v.borrow_mut().pop()).flatten();
            std::mem::forget(guard);
            if let Some(rep) = pending {
                complete_reply(rep, r);
            }
        }
    }
}

/// Dispatch every pending incoming sent message / internal call.
pub fn process_sent(q: &Queue) -> bool {
    let mut any = false;
    loop {
        let item = {
            let g = handles::wait_lock();
            q.st.get(&g).sent.pop_front()
        };
        match item {
            Some(s) => {
                any = true;
                run_sent(s)
            }
            None => return any,
        }
    }
}

enum Found {
    Posted(PostedMsg),
    Synthetic(MSG),
    Input(InputMsg),
}

/// Core of GetMessage/PeekMessage. Returns `None` if nothing matched and
/// `wait` is false.
pub fn retrieve(f: Filter, remove: bool, wait: bool) -> Option<MSG> {
    let q = current_queue();
    loop {
        // Sent messages are always processed by Get/Peek, whatever the filter.
        process_sent(&q);
        let found = {
            let g = handles::wait_lock();
            let st = q.st.get(&g);
            st.wake = 0;
            scan(st, &f, remove)
        };
        match found {
            Some(Found::Posted(p)) => {
                if super::input::filter_posted_msg(&p.msg, remove) {
                    if !remove {
                        let g = handles::wait_lock();
                        q.st.get(&g).posted.retain(|x| x.seq != p.seq);
                    }
                    continue;
                }
                note_retrieved(&p.msg);
                return Some(p.msg);
            }
            Some(Found::Synthetic(m)) => {
                note_retrieved(&m);
                return Some(m);
            }
            Some(Found::Input(i)) => {
                let m = i.msg;
                if super::input::filter_input_msg(&m, remove, i.ll_checked) {
                    if !remove {
                        let g = handles::wait_lock();
                        q.st.get(&g).input.retain(|x| x.seq != i.seq);
                    }
                    continue;
                }
                if !remove && !i.ll_checked {
                    let g = handles::wait_lock();
                    if let Some(entry) = q.st.get(&g).input.iter_mut().find(|x| x.seq == i.seq) {
                        entry.ll_checked = true;
                    }
                }
                note_retrieved(&m);
                if remove {
                    super::input::on_input_retrieved(&m);
                    if matches!(m.message, wm::KEYDOWN | wm::SYSKEYDOWN) {
                        LAST_KEY.with(|k| *k.borrow_mut() = Some((m, i.chars)));
                    }
                }
                return Some(m);
            }
            None => {}
        }
        if !wait {
            return None;
        }
        let timeout = {
            let g = handles::wait_lock();
            let st = q.st.get(&g);
            if !st.sent.is_empty() {
                continue;
            }
            let now = now_ms();
            st.timers.iter().map(|t| t.due.saturating_sub(now)).min().map(|d| d.min(u32::MAX as u64 - 1) as u32).unwrap_or(INFINITE)
        };
        let w: Arc<dyn KObject> = Arc::new(QueueWaiter { q: q.clone(), mask: QS_ALLINPUT, any_available: false });
        let _ = handles::wait_for(&[w], false, timeout, false);
    }
}

fn scan(st: &mut QState, f: &Filter, remove: bool) -> Option<Found> {
    if f.kinds & (QS_POSTMESSAGE | QS_ALLPOSTMESSAGE) != 0 {
        if let Some(i) = st.posted.iter().position(|m| f.ok(&m.msg)) {
            let m = if remove { st.posted.remove(i).unwrap() } else { st.posted[i] };
            return Some(Found::Posted(m));
        }
    }
    if f.kinds & QS_INPUT != 0 {
        if let Some(i) = st.input.iter().position(|m| f.ok(&m.msg) && f.kinds & input_bits(m.msg.message) != 0) {
            let m = if remove { st.input.remove(i).unwrap() } else { st.input[i].clone() };
            return Some(Found::Input(m));
        }
    }
    if let Some(code) = st.quit {
        // WM_QUIT is returned regardless of the hwnd/message-range filter.
        if f.kinds & QS_POSTMESSAGE != 0 {
            if remove {
                st.quit = None;
            }
            let m = MSG { message: wm::QUIT, wparam: code as isize as usize, time: tick_ms(), ..Default::default() };
            return Some(Found::Synthetic(m));
        }
    }
    if f.kinds & QS_PAINT != 0 && f.msg_ok(wm::PAINT) {
        if let Some(&h) = st.paint.iter().find(|&&h| f.hwnd_ok(h)) {
            // WM_PAINT is never removed; it is generated until validated.
            return Some(Found::Synthetic(MSG { hwnd: h, message: wm::PAINT, time: tick_ms(), ..Default::default() }));
        }
    }
    if f.kinds & QS_TIMER != 0 && f.msg_ok(wm::TIMER) {
        let now = now_ms();
        if let Some(t) = st.timers.iter_mut().find(|t| t.due <= now && f.hwnd_ok(t.hwnd)) {
            let m = MSG {
                hwnd: t.hwnd,
                message: wm::TIMER,
                wparam: t.id,
                lparam: t.proc_ as isize,
                time: tick_ms(),
                ..Default::default()
            };
            if remove {
                t.due = now + t.interval;
            }
            return Some(Found::Synthetic(m));
        }
    }
    None
}

fn note_retrieved(m: &MSG) {
    LAST_MSG.with(|c| c.set((m.time, m.pt)));
}

fn qs_kinds(flags: u32) -> u32 {
    let k = flags >> 16;
    if k == 0 {
        QS_ALLINPUT
    } else {
        k | QS_SENDMESSAGE
    }
}

unsafe fn get_message(out: *mut MSG, hwnd: usize, min: u32, max: u32) -> BOOL {
    if out.is_null() {
        set_error(e::INVALID_PARAMETER);
        return -1;
    }
    if hwnd != 0 && hwnd != usize::MAX && !super::window::exists(hwnd) {
        set_error(e::INVALID_WINDOW_HANDLE);
        return -1;
    }
    let mut m = retrieve(Filter { hwnd, min, max, kinds: QS_ALLINPUT }, true, true).unwrap();
    crate::hitch::heartbeat(crate::hitch::HeartbeatKind::MessagePump);
    super::misc::dispatch_getmessage_hook(&mut m, true);
    std::ptr::write_unaligned(out, m);
    (m.message != wm::QUIT) as BOOL
}

pub unsafe extern "win64" fn GetMessageW(out: *mut MSG, hwnd: HWND, min: u32, max: u32) -> BOOL {
    get_message(out, hwnd as usize, min, max)
}

pub unsafe extern "win64" fn GetMessageA(out: *mut MSG, hwnd: HWND, min: u32, max: u32) -> BOOL {
    get_message(out, hwnd as usize, min, max)
}

unsafe fn peek_message(out: *mut MSG, hwnd: usize, min: u32, max: u32, flags: u32) -> BOOL {
    crate::hitch::heartbeat(crate::hitch::HeartbeatKind::MessagePump);
    let remove = flags & PM_REMOVE != 0;
    let f = Filter { hwnd, min, max, kinds: qs_kinds(flags) };
    match retrieve(f, remove, false) {
        Some(mut m) => {
            super::misc::dispatch_getmessage_hook(&mut m, remove);
            if !out.is_null() {
                std::ptr::write_unaligned(out, m);
            }
            TRUE
        }
        None => FALSE,
    }
}

pub unsafe extern "win64" fn PeekMessageW(out: *mut MSG, hwnd: HWND, min: u32, max: u32, flags: u32) -> BOOL {
    peek_message(out, hwnd as usize, min, max, flags)
}

pub unsafe extern "win64" fn PeekMessageA(out: *mut MSG, hwnd: HWND, min: u32, max: u32, flags: u32) -> BOOL {
    peek_message(out, hwnd as usize, min, max, flags)
}

pub unsafe extern "win64" fn WaitMessage() -> BOOL {
    let q = current_queue();
    loop {
        if process_sent(&q) {
            return TRUE;
        }
        let w: Arc<dyn KObject> = Arc::new(QueueWaiter { q: q.clone(), mask: QS_ALLINPUT, any_available: false });
        if let WaitResult::Object(_) = handles::wait_for(&[w], false, INFINITE, false) {
            return TRUE;
        }
    }
}

pub unsafe extern "win64" fn GetQueueStatus(flags: u32) -> u32 {
    let q = current_queue();
    let g = handles::wait_lock();
    let st = q.st.get(&g);
    let cur = status_bits(st) & flags;
    let new = st.wake & flags;
    st.wake &= !flags;
    (new << 16) | cur
}

pub unsafe extern "win64" fn GetInputState() -> BOOL {
    let q = current_queue();
    let g = handles::wait_lock();
    (status_bits(q.st.get(&g)) & (QS_MOUSEBUTTON | QS_KEY) != 0) as BOOL
}

pub unsafe extern "win64" fn GetMessageTime() -> i32 {
    LAST_MSG.with(|c| c.get().0 as i32)
}

pub unsafe extern "win64" fn GetMessagePos() -> u32 {
    let p = LAST_MSG.with(|c| c.get().1);
    make_long(p.x, p.y) as u32
}

pub unsafe extern "win64" fn GetMessageExtraInfo() -> LPARAM {
    0
}

pub unsafe extern "win64" fn InSendMessage() -> BOOL {
    ACTIVE_REPLIES.with(|v| !v.borrow().is_empty()) as BOOL
}

/// `TranslateMessage`: posts `WM_CHAR`/`WM_SYSCHAR` for the characters
/// AppKit produced with the key press (layout-correct), falling back to
/// the US-layout table for synthetic key messages.
pub unsafe extern "win64" fn TranslateMessage(m: *const MSG) -> BOOL {
    if m.is_null() {
        return FALSE;
    }
    let m = &*m;
    match m.message {
        wm::KEYDOWN | wm::SYSKEYDOWN => {}
        wm::KEYUP | wm::SYSKEYUP => return TRUE,
        _ => return FALSE,
    }
    let chars = LAST_KEY.with(|k| {
        let k = k.borrow();
        match &*k {
            Some((lm, c)) if lm.hwnd == m.hwnd && lm.wparam == m.wparam && lm.lparam == m.lparam && lm.time == m.time => Some(c.clone()),
            _ => None,
        }
    });
    let chars = chars.unwrap_or_else(|| {
        super::keymap::us_char(m.wparam as u32, super::input::shift_down(), super::input::caps_on()).map(|c| vec![c]).unwrap_or_default()
    });
    let cm = if m.message == wm::SYSKEYDOWN { wm::SYSCHAR } else { wm::CHAR };
    let q = current_queue();
    for c in chars {
        q.post(MSG { hwnd: m.hwnd, message: cm, wparam: c as usize, lparam: m.lparam, time: m.time, pt: m.pt, l_private: 0 });
    }
    TRUE
}

unsafe fn dispatch(m: *const MSG) -> LRESULT {
    crate::hitch::heartbeat(crate::hitch::HeartbeatKind::MessagePump);
    if m.is_null() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let m = &*m;
    if m.message == wm::TIMER && m.lparam != 0 {
        crate::call_guest8(m.lparam as usize, m.hwnd as u64, wm::TIMER as u64, m.wparam as u64, tick_ms() as u64, 0, 0, 0, 0);
        return 0;
    }
    if m.hwnd == 0 {
        return 0;
    }
    match super::window::get(m.hwnd) {
        None => {
            set_error(e::INVALID_WINDOW_HANDLE);
            0
        }
        Some(w) if w.tid != crate::teb::thread_id() => {
            set_error(e::WINDOW_OF_OTHER_THREAD);
            0
        }
        Some(_) => {
            let r = super::window::call_window_proc(m.hwnd, m.message, m.wparam, m.lparam);
            r
        }
    }
}

pub unsafe extern "win64" fn DispatchMessageW(m: *const MSG) -> LRESULT {
    dispatch(m)
}

pub unsafe extern "win64" fn DispatchMessageA(m: *const MSG) -> LRESULT {
    dispatch(m)
}

fn cursor_point() -> POINT {
    super::cursor::cursor_pos()
}

/// `PostMessage` semantics: `hwnd == 0` posts to the calling thread;
/// `HWND_BROADCAST` posts to every top-level window.
pub fn post_message(hwnd: usize, msg: u32, wparam: WPARAM, lparam: LPARAM) -> BOOL {
    let m = MSG { hwnd, message: msg, wparam, lparam, time: tick_ms(), pt: cursor_point(), l_private: 0 };
    if hwnd == 0 {
        current_queue().post(m);
        return TRUE;
    }
    if hwnd == 0xFFFF {
        for h in super::window::top_level_windows() {
            post_message(h, msg, wparam, lparam);
        }
        return TRUE;
    }
    let Some(w) = super::window::get(hwnd) else {
        set_error(e::INVALID_WINDOW_HANDLE);
        return FALSE;
    };
    match queue_of(w.tid) {
        Some(q) => {
            q.post(m);
            TRUE
        }
        None => {
            set_error(e::INVALID_WINDOW_HANDLE);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn PostMessageW(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> BOOL {
    post_message(hwnd as usize, msg, wparam, lparam)
}

pub unsafe extern "win64" fn PostMessageA(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> BOOL {
    post_message(hwnd as usize, msg, wparam, lparam)
}

fn post_thread(tid: u32, msg: u32, wparam: WPARAM, lparam: LPARAM) -> BOOL {
    match queue_of(tid) {
        Some(q) => {
            q.post(MSG { hwnd: 0, message: msg, wparam, lparam, time: tick_ms(), pt: cursor_point(), l_private: 0 });
            TRUE
        }
        None => {
            set_error(e::INVALID_THREAD_ID);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn PostThreadMessageW(tid: u32, msg: u32, wparam: WPARAM, lparam: LPARAM) -> BOOL {
    post_thread(tid, msg, wparam, lparam)
}

pub unsafe extern "win64" fn PostThreadMessageA(tid: u32, msg: u32, wparam: WPARAM, lparam: LPARAM) -> BOOL {
    post_thread(tid, msg, wparam, lparam)
}

pub unsafe extern "win64" fn PostQuitMessage(code: i32) {
    current_queue().post_quit(code);
}

/// `SendMessage`: direct call on the owning thread; otherwise queue it to
/// the owner and wait for the reply while still dispatching messages sent
/// *to us* (so two threads sending to each other cannot deadlock).
pub fn send_message(hwnd: usize, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    send_message_timeout(hwnd, msg, wparam, lparam, INFINITE).unwrap_or(0)
}

pub fn send_message_timeout(hwnd: usize, msg: u32, wparam: WPARAM, lparam: LPARAM, timeout_ms: u32) -> Option<LRESULT> {
    if hwnd == 0xFFFF {
        for h in super::window::top_level_windows() {
            send_message_timeout(h, msg, wparam, lparam, timeout_ms);
        }
        return Some(1);
    }
    let Some(w) = super::window::get(hwnd) else {
        set_error(e::INVALID_WINDOW_HANDLE);
        return None;
    };
    let me = crate::teb::thread_id();
    if w.tid == me {
        super::misc::dispatch_callwndproc_hook(hwnd, msg, wparam, lparam, true);
        return Some(super::window::call_window_proc(hwnd, msg, wparam, lparam));
    }
    let Some(target) = queue_of(w.tid) else {
        set_error(e::INVALID_WINDOW_HANDLE);
        return None;
    };
    let reply = Arc::new(Reply { result: Guarded::new(None) });
    target.send_item(Sent::Msg { hwnd, msg, wparam, lparam, reply: Some(reply.clone()) });
    let mine = current_queue();
    let deadline = (timeout_ms != INFINITE).then(|| now_ms() + timeout_ms as u64);
    loop {
        process_sent(&mine);
        {
            let g = handles::wait_lock();
            if let Some(r) = *reply.result.get(&g) {
                return Some(r);
            }
        }
        if !thread_alive(w.tid) {
            return Some(0);
        }
        let mut wait = 250u32; // re-check the target thread's liveness periodically
        if let Some(d) = deadline {
            let now = now_ms();
            if now >= d {
                set_error(e::TIMEOUT);
                return None;
            }
            wait = wait.min((d - now) as u32);
        }
        let objs: [Arc<dyn KObject>; 2] =
            [reply.clone(), Arc::new(QueueWaiter { q: mine.clone(), mask: QS_SENDMESSAGE, any_available: true })];
        let _ = handles::wait_for(&objs, false, wait, false);
    }
}

pub unsafe extern "win64" fn SendMessageW(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    send_message(hwnd as usize, msg, wparam, lparam)
}

/// ANSI variant. Our windows are Unicode internally; the text messages are
/// converted, everything else passes through.
pub unsafe extern "win64" fn SendMessageA(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        wm::SETTEXT if lparam != 0 => {
            let w = crate::strings::to_wide_z(&crate::strings::from_ansi(lparam as *const u8));
            send_message(hwnd as usize, msg, wparam, w.as_ptr() as LPARAM)
        }
        wm::GETTEXT if lparam != 0 && wparam > 0 => {
            let mut buf = vec![0u16; wparam];
            let n = send_message(hwnd as usize, msg, wparam, buf.as_mut_ptr() as LPARAM).max(0) as usize;
            let s = String::from_utf16_lossy(&buf[..n.min(buf.len())]);
            crate::strings::write_ansi_trunc(&s, lparam as *mut u8, wparam).1 as LRESULT
        }
        _ => send_message(hwnd as usize, msg, wparam, lparam),
    }
}

pub unsafe extern "win64" fn SendMessageTimeoutW(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    _flags: u32,
    timeout: u32,
    result: *mut usize,
) -> LRESULT {
    match send_message_timeout(hwnd as usize, msg, wparam, lparam, timeout) {
        Some(r) => {
            if !result.is_null() {
                *result = r as usize;
            }
            1
        }
        None => 0,
    }
}

/// `SendNotifyMessage`: synchronous on the owner thread, fire-and-forget
/// across threads.
pub fn send_notify(hwnd: usize, msg: u32, wparam: WPARAM, lparam: LPARAM) -> BOOL {
    let Some(w) = super::window::get(hwnd) else {
        set_error(e::INVALID_WINDOW_HANDLE);
        return FALSE;
    };
    if w.tid == crate::teb::thread_id() {
        super::misc::dispatch_callwndproc_hook(hwnd, msg, wparam, lparam, true);
        super::window::call_window_proc(hwnd, msg, wparam, lparam);
        return TRUE;
    }
    match queue_of(w.tid) {
        Some(q) => {
            q.send_item(Sent::Msg { hwnd, msg, wparam, lparam, reply: None });
            TRUE
        }
        None => FALSE,
    }
}

pub unsafe extern "win64" fn SendNotifyMessageW(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> BOOL {
    send_notify(hwnd as usize, msg, wparam, lparam)
}

pub unsafe extern "win64" fn ReplyMessage(r: LRESULT) -> BOOL {
    let entry = ACTIVE_REPLIES.with(|v| {
        let mut s = v.borrow_mut();
        s.last_mut().map(|slot| slot.take())
    });
    match entry {
        None => FALSE, // not processing an inter-thread sent message
        Some(Some(rep)) => {
            complete_reply(rep, r);
            TRUE
        }
        Some(None) => TRUE, // already replied or fire-and-forget SendNotifyMessage
    }
}

pub unsafe extern "win64" fn CallWindowProcW(p: usize, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    super::dispatch_wndproc(p, hwnd, msg, wparam, lparam)
}

pub unsafe extern "win64" fn CallWindowProcA(p: usize, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    super::dispatch_wndproc(p, hwnd, msg, wparam, lparam)
}

// ---------------------------------------------------------------------------
// Timers
// ---------------------------------------------------------------------------

static NEXT_TIMER_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0x7FF0);

pub unsafe extern "win64" fn SetTimer(hwnd: HWND, id: usize, elapse: u32, proc_: usize) -> usize {
    let hwnd = hwnd as usize;
    let q = if hwnd != 0 {
        match super::window::get(hwnd) {
            Some(w) if w.tid == crate::teb::thread_id() => current_queue(),
            Some(_) => {
                set_error(e::WINDOW_OF_OTHER_THREAD);
                return 0;
            }
            None => {
                set_error(e::INVALID_WINDOW_HANDLE);
                return 0;
            }
        }
    } else {
        current_queue()
    };
    // USER_TIMER_MINIMUM = 10 ms, USER_TIMER_MAXIMUM = 0x7FFFFFFF.
    let interval = (elapse.clamp(10, 0x7FFF_FFFF)) as u64;
    let id = if hwnd == 0 { NEXT_TIMER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed) } else { id };
    let t = Timer { hwnd, id, interval, due: now_ms() + interval, proc_ };
    q.push(|st| {
        st.timers.retain(|x| !(x.hwnd == hwnd && x.id == id));
        st.timers.push(t);
        0
    });
    if hwnd == 0 {
        id
    } else if id == 0 {
        1
    } else {
        id
    }
}

pub unsafe extern "win64" fn KillTimer(hwnd: HWND, id: usize) -> BOOL {
    let hwnd = hwnd as usize;
    let q = current_queue();
    let g = handles::wait_lock();
    let st = q.st.get(&g);
    let before = st.timers.len();
    st.timers.retain(|x| !(x.hwnd == hwnd && x.id == id));
    if st.timers.len() == before {
        return FALSE;
    }
    TRUE
}

// ---------------------------------------------------------------------------
// MsgWaitForMultipleObjects
// ---------------------------------------------------------------------------

const MWMO_WAITALL: u32 = 1;
const MWMO_ALERTABLE: u32 = 2;
const MWMO_INPUTAVAILABLE: u32 = 4;

pub unsafe extern "win64" fn MsgWaitForMultipleObjectsEx(n: u32, hs: *const HANDLE, ms: u32, mask: u32, flags: u32) -> u32 {
    if n > 63 || (n > 0 && hs.is_null()) {
        set_error(e::INVALID_PARAMETER);
        return WAIT_FAILED;
    }
    let mut objs = handles::WaitableList::with_capacity(n as usize + 1);
    for &h in std::slice::from_raw_parts(hs, n as usize) {
        match handles::get(h) {
            Some(o) if o.waitable().is_some() => objs.push(o),
            _ => {
                set_error(err::ERROR_INVALID_HANDLE);
                return WAIT_FAILED;
            }
        }
    }
    let q = current_queue();
    let wait_all = flags & MWMO_WAITALL != 0;
    let any_available = flags & MWMO_INPUTAVAILABLE != 0;
    if wait_all {
        if mask != 0 || n == 0 {
            objs.push(Arc::new(QueueWaiter { q, mask, any_available }));
        }
    } else {
        objs.push(Arc::new(QueueWaiter { q, mask: mask | QS_SENDMESSAGE, any_available }));
    }
    match handles::wait_for(objs.as_slice(), wait_all, ms, flags & MWMO_ALERTABLE != 0) {
        WaitResult::Object(i) => WAIT_OBJECT_0 + i as u32,
        WaitResult::Abandoned(i) => WAIT_ABANDONED_0 + i as u32,
        WaitResult::Timeout => WAIT_TIMEOUT,
        WaitResult::IoCompletion => WAIT_IO_COMPLETION,
        WaitResult::Failed(e) => {
            set_error(e);
            WAIT_FAILED
        }
    }
}

pub unsafe extern "win64" fn MsgWaitForMultipleObjects(n: u32, hs: *const HANDLE, wait_all: BOOL, ms: u32, mask: u32) -> u32 {
    MsgWaitForMultipleObjectsEx(n, hs, ms, mask, if wait_all != 0 { MWMO_WAITALL } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_peek_get_order_and_filters() {
        std::thread::spawn(|| unsafe {
            let mut m = MSG::default();
            assert_eq!(PeekMessageW(&mut m, std::ptr::null_mut(), 0, 0, PM_REMOVE), FALSE);
            PostMessageW(std::ptr::null_mut(), wm::USER + 1, 1, 0);
            PostMessageW(std::ptr::null_mut(), wm::USER + 2, 2, 0);
            // Range filter skips USER+1.
            assert_eq!(PeekMessageW(&mut m, std::ptr::null_mut(), wm::USER + 2, wm::USER + 2, PM_REMOVE), TRUE);
            assert_eq!(m.wparam, 2);
            assert_eq!(GetMessageW(&mut m, std::ptr::null_mut(), 0, 0), TRUE);
            assert_eq!(m.message, wm::USER + 1);
            PostQuitMessage(7);
            assert_eq!(GetMessageW(&mut m, std::ptr::null_mut(), 0, 0), FALSE);
            assert_eq!(m.message, wm::QUIT);
            assert_eq!(m.wparam, 7);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn cross_thread_post_wakes_getmessage() {
        let (tx, rx) = std::sync::mpsc::channel();
        let h = std::thread::spawn(move || unsafe {
            let mut m = MSG::default();
            PeekMessageW(&mut m, std::ptr::null_mut(), 0, 0, PM_NOREMOVE); // create queue
            tx.send(crate::teb::thread_id()).unwrap();
            assert_eq!(GetMessageW(&mut m, std::ptr::null_mut(), 0, 0), TRUE);
            m.message
        });
        let tid = rx.recv().unwrap();
        unsafe { assert_eq!(PostThreadMessageW(tid, wm::APP, 0, 0), TRUE) };
        assert_eq!(h.join().unwrap(), wm::APP);
        unsafe { assert_eq!(PostThreadMessageW(0xFFFF_FFF0, wm::APP, 0, 0), FALSE) };
    }

    #[test]
    fn timers_fire() {
        std::thread::spawn(|| unsafe {
            let id = SetTimer(std::ptr::null_mut(), 0, 10, 0);
            assert_ne!(id, 0);
            let mut m = MSG::default();
            assert_eq!(GetMessageW(&mut m, std::ptr::null_mut(), 0, 0), TRUE);
            assert_eq!(m.message, wm::TIMER);
            assert_eq!(m.wparam, id);
            assert_eq!(KillTimer(std::ptr::null_mut(), id), TRUE);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn msg_wait_all_requires_both_handle_and_message() {
        std::thread::spawn(|| unsafe {
            let mut m = MSG::default();
            PeekMessageW(&mut m, std::ptr::null_mut(), 0, 0, PM_NOREMOVE);
            let tid = crate::teb::thread_id();
            let ev = crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            assert!(!ev.is_null());

            // 1. Only the event is signaled -> MWMO_WAITALL must time out.
            assert_eq!(crate::kernel32::threads::SetEvent(ev), TRUE);
            let r1 = MsgWaitForMultipleObjectsEx(1, &ev, 20, QS_POSTMESSAGE, MWMO_WAITALL | MWMO_INPUTAVAILABLE);
            assert_eq!(r1, WAIT_TIMEOUT);

            // 2. Only a message is queued (event reset) -> MWMO_WAITALL must time out.
            assert_eq!(crate::kernel32::threads::ResetEvent(ev), TRUE);
            assert_eq!(PostThreadMessageW(tid, wm::USER + 10, 1, 2), TRUE);
            let r2 = MsgWaitForMultipleObjectsEx(1, &ev, 20, QS_POSTMESSAGE, MWMO_WAITALL | MWMO_INPUTAVAILABLE);
            assert_eq!(r2, WAIT_TIMEOUT);

            // 3. Both event and message satisfied -> MWMO_WAITALL succeeds.
            assert_eq!(crate::kernel32::threads::SetEvent(ev), TRUE);
            let r3 = MsgWaitForMultipleObjectsEx(1, &ev, 1_000, QS_POSTMESSAGE, MWMO_WAITALL | MWMO_INPUTAVAILABLE);
            assert_eq!(r3, WAIT_OBJECT_0);

            assert_eq!(crate::kernel32::threads::CloseHandle(ev), TRUE);
            crate::teb::uninstall();
        })
        .join()
        .unwrap();
    }

    #[test]
    fn call_window_proc_requires_typed_wndproc_or_validated_guest_target() {
        use crate::kernel32::memory::{
            VirtualAlloc, VirtualFree, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
            PAGE_READWRITE,
        };
        use std::sync::atomic::{AtomicUsize, Ordering};

        static UNREG_HITS: AtomicUsize = AtomicUsize::new(0);
        static TYPED_HITS: AtomicUsize = AtomicUsize::new(0);
        static GUEST_DISPATCH_TARGET: AtomicUsize = AtomicUsize::new(0);

        unsafe extern "win64" fn unregistered_host_fn(
            _hwnd: HWND,
            _msg: u32,
            _wp: WPARAM,
            _lp: LPARAM,
        ) -> LRESULT {
            UNREG_HITS.fetch_add(1, Ordering::SeqCst);
            0x7777
        }

        unsafe extern "win64" fn registered_typed_proc(
            _hwnd: HWND,
            msg: u32,
            wp: WPARAM,
            lp: LPARAM,
        ) -> LRESULT {
            TYPED_HITS.fetch_add(1, Ordering::SeqCst);
            (msg as isize) + (wp as isize) + lp
        }

        unsafe fn mock_guest_dispatcher(
            va: u64,
            _rcx: u64,
            rdx: u64,
            r8: u64,
            r9: u64,
            _a5: u64,
            _a6: u64,
            _a7: u64,
            _a8: u64,
        ) -> u64 {
            GUEST_DISPATCH_TARGET.store(va as usize, Ordering::SeqCst);
            0x1000 + rdx + r8 + r9
        }

        std::thread::spawn(|| unsafe {
            UNREG_HITS.store(0, Ordering::SeqCst);
            TYPED_HITS.store(0, Ordering::SeqCst);
            GUEST_DISPATCH_TARGET.store(0, Ordering::SeqCst);

            // 1. Unregistered raw host function pointer is rejected BEFORE invocation (both W & A).
            let raw_host = unregistered_host_fn as *const () as usize;
            crate::teb::set_last_error(0);
            assert_eq!(CallWindowProcW(raw_host, std::ptr::null_mut(), wm::USER, 1, 2), 0);
            assert_eq!(crate::teb::last_error(), e::INVALID_PARAMETER);
            assert_eq!(UNREG_HITS.load(Ordering::SeqCst), 0);

            crate::teb::set_last_error(0);
            assert_eq!(CallWindowProcA(raw_host, std::ptr::null_mut(), wm::USER, 1, 2), 0);
            assert_eq!(crate::teb::last_error(), e::INVALID_PARAMETER);
            assert_eq!(UNREG_HITS.load(Ordering::SeqCst), 0);

            // 2. Explicitly registered typed host WNDPROC works via its opaque 0xFFFFxxxx handle,
            //    and the raw host pointer itself remains rejected.
            let handle = super::super::register_typed_host_wndproc(registered_typed_proc);
            assert_eq!(handle >> 16, 0xFFFF);
            assert_ne!(handle, registered_typed_proc as *const () as usize);
            assert_eq!(
                CallWindowProcW(handle, std::ptr::null_mut(), wm::USER, 10, 20),
                (wm::USER as isize) + 30
            );
            assert_eq!(TYPED_HITS.load(Ordering::SeqCst), 1);
            assert_eq!(
                CallWindowProcW(registered_typed_proc as *const () as usize, std::ptr::null_mut(), wm::USER, 10, 20),
                0
            );
            assert_eq!(TYPED_HITS.load(Ordering::SeqCst), 1);

            // 3. Built-in opaque WNDPROC handles (e.g. DefWindowProcW, Edit) work directly.
            assert_eq!(
                CallWindowProcW(super::super::WNDPROC_HANDLE_EDIT, std::ptr::null_mut(), wm::GETDLGCODE, 0, 0),
                0x0001 | 0x0080 // DLGC_WANTARROWS | DLGC_WANTCHARS
            );

            // 4. Guest VirtualAlloc executable page is rejected when no guest dispatcher is active,
            //    non-executable RW guest page is always rejected, and executable guest page is
            //    dispatched when a guest dispatcher is active.
            let rw_page = VirtualAlloc(std::ptr::null_mut(), 0x1000, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
            let rwx_page = VirtualAlloc(std::ptr::null_mut(), 0x1000, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
            assert!(!rw_page.is_null() && !rwx_page.is_null());

            // No guest dispatcher installed -> even executable guest memory is rejected safely.
            assert_eq!(CallWindowProcW(rwx_page as usize, std::ptr::null_mut(), wm::USER, 3, 4), 0);
            assert_eq!(crate::teb::last_error(), e::INVALID_PARAMETER);

            {
                let _disp = crate::tests::install_test_guest_dispatcher(Some(mock_guest_dispatcher));
                // Non-executable guest memory still rejected even with guest dispatcher active.
                crate::teb::set_last_error(0);
                assert_eq!(CallWindowProcW(rw_page as usize, std::ptr::null_mut(), wm::USER, 3, 4), 0);
                assert_eq!(crate::teb::last_error(), e::INVALID_PARAMETER);
                assert_eq!(GUEST_DISPATCH_TARGET.load(Ordering::SeqCst), 0);

                // Unregistered raw host function still rejected even with guest dispatcher active.
                crate::teb::set_last_error(0);
                assert_eq!(CallWindowProcW(raw_host, std::ptr::null_mut(), wm::USER, 3, 4), 0);
                assert_eq!(crate::teb::last_error(), e::INVALID_PARAMETER);
                assert_eq!(UNREG_HITS.load(Ordering::SeqCst), 0);
                assert_eq!(GUEST_DISPATCH_TARGET.load(Ordering::SeqCst), 0);

                // Validated executable guest VM target dispatches cleanly through guest dispatcher.
                assert_eq!(
                    CallWindowProcW(rwx_page as usize, std::ptr::null_mut(), wm::USER, 3, 4),
                    (0x1000 + wm::USER as isize + 7)
                );
                assert_eq!(GUEST_DISPATCH_TARGET.load(Ordering::SeqCst), rwx_page as usize);
            }

            assert_eq!(VirtualFree(rw_page, 0, MEM_RELEASE), TRUE);
            assert_eq!(VirtualFree(rwx_page, 0, MEM_RELEASE), TRUE);
            crate::teb::uninstall();
        })
        .join()
        .unwrap();
    }

    #[test]
    fn reply_message_unblocks_cross_thread_sender_early() {
        use std::sync::atomic::{AtomicBool, Ordering};
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        const WM_EARLY_REPLY: u32 = wm::USER + 0x55;

        static SENDER_GOT_REPLY: AtomicBool = AtomicBool::new(false);
        static WNDPROC_OBSERVED_SENDER_UNBLOCKED: AtomicBool = AtomicBool::new(false);

        unsafe extern "win64" fn reply_wndproc(
            hwnd: HWND,
            msg: u32,
            wparam: WPARAM,
            lparam: LPARAM,
        ) -> LRESULT {
            if msg == WM_EARLY_REPLY {
                assert_eq!(InSendMessage(), TRUE);
                assert_eq!(ReplyMessage(0x1234), TRUE);
                // Second ReplyMessage on same send returns TRUE without overwriting.
                assert_eq!(ReplyMessage(0x9999), TRUE);
                // Wait up to 2s to prove sender unblocked BEFORE WndProc returned!
                let start = std::time::Instant::now();
                while !SENDER_GOT_REPLY.load(Ordering::Acquire) && start.elapsed().as_millis() < 2000 {
                    std::thread::sleep(std::time::Duration::from_millis(2));
                }
                WNDPROC_OBSERVED_SENDER_UNBLOCKED.store(
                    SENDER_GOT_REPLY.load(Ordering::Acquire),
                    Ordering::Release,
                );
                return 0x7777; // must NOT overwrite the early 0x1234 reply
            }
            super::defproc::DefWindowProcW(hwnd, msg, wparam, lparam)
        }

        SENDER_GOT_REPLY.store(false, Ordering::Release);
        WNDPROC_OBSERVED_SENDER_UNBLOCKED.store(false, Ordering::Release);

        // Outside an inter-thread SendMessage, ReplyMessage returns FALSE.
        unsafe {
            assert_eq!(InSendMessage(), FALSE);
            assert_eq!(ReplyMessage(1), FALSE);
        }

        let (tx_hwnd, rx_hwnd) = std::sync::mpsc::channel::<usize>();
        let (tx_done, rx_done) = std::sync::mpsc::channel::<()>();

        let owner = std::thread::spawn(move || unsafe {
            let cls = crate::strings::to_wide_z("EarlyReplyTestWnd");
            let wc = super::class::WNDCLASSEX {
                cb_size: std::mem::size_of::<super::class::WNDCLASSEX>() as u32,
                style: 0,
                wndproc: super::register_typed_host_wndproc(reply_wndproc),
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: cls.as_ptr() as usize,
                icon_sm: 0,
            };
            let _ = super::class::RegisterClassExW(&wc);
            let hwnd = super::window::CreateWindowExW(
                0, cls.as_ptr(), std::ptr::null(), ws::POPUP,
                0, 0, 50, 50, std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut(),
            );
            assert!(!hwnd.is_null());
            tx_hwnd.send(hwnd as usize).unwrap();

            // Pump until sender finishes.
            let mut m = MSG::default();
            while rx_done.try_recv().is_err() {
                let _ = PeekMessageW(&mut m, std::ptr::null_mut(), 0, 0, PM_REMOVE);
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
            let _ = super::window::DestroyWindow(hwnd);
        });

        let hwnd = rx_hwnd.recv().unwrap();
        let sender = std::thread::spawn(move || unsafe {
            let res = SendMessageW(hwnd as HWND, WM_EARLY_REPLY, 0, 0);
            assert_eq!(res, 0x1234, "sender must receive early ReplyMessage value");
            SENDER_GOT_REPLY.store(true, Ordering::Release);
        });
        sender.join().unwrap();
        tx_done.send(()).unwrap();
        owner.join().unwrap();

        assert!(
            WNDPROC_OBSERVED_SENDER_UNBLOCKED.load(Ordering::Acquire),
            "sender must unblock before receiver WndProc returns"
        );
    }
}
