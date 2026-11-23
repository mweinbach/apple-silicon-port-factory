//! `CURLM` multi handle runtime for `libcurl.dll`.

use super::easy::{
    apply_head_meta, apply_outcome, invoke_header_callback, invoke_write_callback,
    prepare_transfer, write_error_buffer, CallbackConfig, CurlEasy, CurlMsg, HeadMeta,
    TransferOutcome, CURLE_ABORTED_BY_CALLBACK, CURLE_FAILED_INIT, EASY_HANDLES,
};
use super::http::{execute_http_transfer, TransferSink};
use crate::handles::HandleTable;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

// CURLMcode
pub(super) const CURLM_OK: i32 = 0;
pub(super) const CURLM_BAD_HANDLE: i32 = 1;
pub(super) const CURLM_BAD_EASY_HANDLE: i32 = 2;
pub(super) const CURLM_OUT_OF_MEMORY: i32 = 3;
pub(super) const CURLM_INTERNAL_ERROR: i32 = 4;
pub(super) const CURLM_BAD_SOCKET: i32 = 5;
pub(super) const CURLM_UNKNOWN_OPTION: i32 = 6;
pub(super) const CURLM_ADDED_ALREADY: i32 = 7;
pub(super) const CURLM_BAD_FUNCTION_ARGUMENT: i32 = 10;

// CURLMSG
pub(super) const CURLMSG_DONE: i32 = 1;

static MULTI_HANDLES: HandleTable<Arc<CurlMulti>> = HandleTable::new(0xC081_0002, 4);

enum TransferEvent {
    Head(HeadMeta),
    HeaderLine(Vec<u8>),
    BodyChunk(Vec<u8>),
}

#[derive(Default)]
struct TransferChannel {
    events: VecDeque<TransferEvent>,
    finished: Option<TransferOutcome>,
}

struct MultiNotify {
    woken: bool,
    seq: u64,
}

struct CurlMulti {
    inner: Mutex<CurlMultiInner>,
    notify_lock: Mutex<MultiNotify>,
    notify_cv: Condvar,
}

impl CurlMulti {
    fn new() -> Self {
        Self {
            inner: Mutex::new(CurlMultiInner {
                entries: HashMap::new(),
                done_queue: VecDeque::new(),
                delivered_msgs: Vec::new(),
            }),
            notify_lock: Mutex::new(MultiNotify {
                woken: false,
                seq: 0,
            }),
            notify_cv: Condvar::new(),
        }
    }

    fn signal_activity(&self) {
        let mut n = self.notify_lock.lock().unwrap();
        n.seq = n.seq.wrapping_add(1);
        self.notify_cv.notify_all();
    }

    fn wakeup(&self) {
        let mut n = self.notify_lock.lock().unwrap();
        n.woken = true;
        n.seq = n.seq.wrapping_add(1);
        self.notify_cv.notify_all();
    }
}

enum MultiEntryState {
    Pending,
    Starting,
    Running {
        channel: Arc<Mutex<TransferChannel>>,
        cancel: Arc<AtomicBool>,
        cb: CallbackConfig,
    },
    Done,
}

struct MultiEntry {
    easy: Arc<Mutex<CurlEasy>>,
    state: MultiEntryState,
}

struct CurlMultiInner {
    entries: HashMap<usize, MultiEntry>,
    done_queue: VecDeque<Box<CurlMsg>>,
    delivered_msgs: Vec<Box<CurlMsg>>,
}

struct MultiWorkerSink {
    channel: Arc<Mutex<TransferChannel>>,
    cancel: Arc<AtomicBool>,
    multi: Arc<CurlMulti>,
}

impl TransferSink for MultiWorkerSink {
    fn is_cancelled(&self) -> bool {
        self.cancel.load(Ordering::Acquire)
    }

    fn cancel_token(&self) -> Option<Arc<AtomicBool>> {
        Some(Arc::clone(&self.cancel))
    }

    fn on_head(&mut self, meta: HeadMeta) {
        {
            let mut ch = self.channel.lock().unwrap();
            ch.events.push_back(TransferEvent::Head(meta));
        }
        self.multi.signal_activity();
    }

    fn on_header_line(&mut self, line: Vec<u8>) -> Result<(), i32> {
        if self.is_cancelled() {
            return Err(CURLE_ABORTED_BY_CALLBACK);
        }
        {
            let mut ch = self.channel.lock().unwrap();
            ch.events.push_back(TransferEvent::HeaderLine(line));
        }
        self.multi.signal_activity();
        Ok(())
    }

    fn on_body_chunk(&mut self, data: Vec<u8>) -> Result<(), i32> {
        if self.is_cancelled() {
            return Err(CURLE_ABORTED_BY_CALLBACK);
        }
        {
            let mut ch = self.channel.lock().unwrap();
            ch.events.push_back(TransferEvent::BodyChunk(data));
        }
        self.multi.signal_activity();
        Ok(())
    }
}

pub(super) fn detach_easy_from_multi(multi_id: usize, easy: usize) {
    if let Some(multi) = MULTI_HANDLES.get_cloned(multi_id) {
        let mut inner = multi.inner.lock().unwrap();
        if let Some(entry) = inner.entries.remove(&easy) {
            if let MultiEntryState::Running { cancel, .. } = entry.state {
                cancel.store(true, Ordering::Release);
            }
        }
        inner.done_queue.retain(|m| m.easy_handle != easy);
        inner.delivered_msgs.retain(|m| m.easy_handle != easy);
    }
}

pub(super) fn multi_init() -> usize {
    MULTI_HANDLES.insert(Arc::new(CurlMulti::new()))
}

pub(super) fn multi_cleanup(multi: usize) -> i32 {
    let Some(m) = MULTI_HANDLES.remove(multi) else {
        return CURLM_BAD_HANDLE;
    };
    let entries: Vec<MultiEntry> = {
        let mut inner = m.inner.lock().unwrap();
        inner.done_queue.clear();
        inner.delivered_msgs.clear();
        inner.entries.drain().map(|(_, e)| e).collect()
    };
    for entry in entries {
        if let MultiEntryState::Running { cancel, .. } = entry.state {
            cancel.store(true, Ordering::Release);
        }
        let mut e = entry.easy.lock().unwrap();
        if e.attached_multi == Some(multi) {
            e.attached_multi = None;
        }
    }
    m.wakeup();
    CURLM_OK
}

pub(super) fn multi_add_handle(multi: usize, easy: usize) -> i32 {
    let Some(m) = MULTI_HANDLES.get_cloned(multi) else {
        return CURLM_BAD_HANDLE;
    };
    let Some(e) = EASY_HANDLES.get_cloned(easy) else {
        return CURLM_BAD_EASY_HANDLE;
    };
    {
        let mut eg = e.lock().unwrap();
        if eg.attached_multi.is_some() {
            return CURLM_ADDED_ALREADY;
        }
        eg.attached_multi = Some(multi);
    }
    let mut inner = m.inner.lock().unwrap();
    if inner.entries.contains_key(&easy) {
        return CURLM_ADDED_ALREADY;
    }
    inner.entries.insert(
        easy,
        MultiEntry {
            easy: e,
            state: MultiEntryState::Pending,
        },
    );
    drop(inner);
    m.signal_activity();
    CURLM_OK
}

pub(super) fn multi_remove_handle(multi: usize, easy: usize) -> i32 {
    let Some(m) = MULTI_HANDLES.get_cloned(multi) else {
        return CURLM_BAD_HANDLE;
    };
    let Some(e) = EASY_HANDLES.get_cloned(easy) else {
        return CURLM_BAD_EASY_HANDLE;
    };
    {
        let mut inner = m.inner.lock().unwrap();
        if let Some(entry) = inner.entries.remove(&easy) {
            if let MultiEntryState::Running { cancel, .. } = entry.state {
                cancel.store(true, Ordering::Release);
            }
        }
        inner.done_queue.retain(|msg| msg.easy_handle != easy);
        inner.delivered_msgs.retain(|msg| msg.easy_handle != easy);
    }
    let mut eg = e.lock().unwrap();
    if eg.attached_multi == Some(multi) {
        eg.attached_multi = None;
    }
    CURLM_OK
}

pub(super) unsafe fn multi_perform(multi: usize, running_handles: *mut i32) -> i32 {
    let Some(m) = MULTI_HANDLES.get_cloned(multi) else {
        if !running_handles.is_null() {
            std::ptr::write_unaligned(running_handles, 0);
        }
        return CURLM_BAD_HANDLE;
    };

    // Phase 1: transition Pending handles -> Starting so we can prepare transfers
    // (including guest READFUNCTION) without holding multi.inner.
    let to_start: Vec<(usize, Arc<Mutex<CurlEasy>>)> = {
        let mut inner = m.inner.lock().unwrap();
        let mut v = Vec::new();
        for (&easy_id, entry) in inner.entries.iter_mut() {
            if matches!(entry.state, MultiEntryState::Pending) {
                entry.state = MultiEntryState::Starting;
                v.push((easy_id, Arc::clone(&entry.easy)));
            }
        }
        v
    };

    // Phase 2: prepare & launch background I/O threads for each newly started handle.
    for (easy_id, easy_arc) in to_start {
        match prepare_transfer(&easy_arc) {
            Ok((req, cb)) => {
                let channel = Arc::new(Mutex::new(TransferChannel::default()));
                let cancel = Arc::new(AtomicBool::new(false));

                let still_present = {
                    let mut inner = m.inner.lock().unwrap();
                    if let Some(entry) = inner.entries.get_mut(&easy_id) {
                        entry.state = MultiEntryState::Running {
                            channel: Arc::clone(&channel),
                            cancel: Arc::clone(&cancel),
                            cb,
                        };
                        true
                    } else {
                        false
                    }
                };

                if still_present {
                    let worker_multi = Arc::clone(&m);
                    let worker_channel = Arc::clone(&channel);
                    let worker_cancel = Arc::clone(&cancel);
                    let spawn_res = std::thread::Builder::new()
                        .name("libcurl-io".to_string())
                        .spawn(move || {
                            let mut sink = MultiWorkerSink {
                                channel: Arc::clone(&worker_channel),
                                cancel: worker_cancel,
                                multi: Arc::clone(&worker_multi),
                            };
                            let outcome = execute_http_transfer(req, &mut sink);
                            {
                                let mut ch = worker_channel.lock().unwrap();
                                ch.finished = Some(outcome);
                            }
                            worker_multi.signal_activity();
                        });
                    if let Err(err) = spawn_res {
                        let outcome = TransferOutcome {
                            code: CURLE_FAILED_INIT,
                            error_msg: Some(format!("Failed to spawn I/O thread: {err}")),
                            size_download: 0,
                            size_upload: 0,
                            content_length_upload: 0,
                            total_time: 0.0,
                            os_errno: err.raw_os_error().unwrap_or(0),
                            ssl_verifyresult: 0,
                        };
                        channel.lock().unwrap().finished = Some(outcome);
                        m.signal_activity();
                    }
                }
            }
            Err((code, msg, cb)) => {
                write_error_buffer(cb.error_buffer, &msg);
                let mut inner = m.inner.lock().unwrap();
                if let Some(entry) = inner.entries.get_mut(&easy_id) {
                    entry.state = MultiEntryState::Done;
                    inner.done_queue.push_back(Box::new(CurlMsg {
                        msg: CURLMSG_DONE,
                        _pad: 0,
                        easy_handle: easy_id,
                        data: (code as u32) as usize,
                    }));
                }
            }
        }
    }

    // Phase 3: snapshot active Running handles so we can invoke guest callbacks
    // strictly on the calling thread with zero locks held.
    let active: Vec<(
        usize,
        Arc<Mutex<CurlEasy>>,
        Arc<Mutex<TransferChannel>>,
        Arc<AtomicBool>,
        CallbackConfig,
    )> = {
        let inner = m.inner.lock().unwrap();
        let mut v = Vec::new();
        for (&easy_id, entry) in &inner.entries {
            if let MultiEntryState::Running {
                channel,
                cancel,
                cb,
            } = &entry.state
            {
                v.push((
                    easy_id,
                    Arc::clone(&entry.easy),
                    Arc::clone(channel),
                    Arc::clone(cancel),
                    *cb,
                ));
            }
        }
        v
    };

    for (easy_id, easy_arc, channel, cancel, cb) in active {
        let (events, finished) = {
            let mut ch = channel.lock().unwrap();
            let evs: Vec<TransferEvent> = ch.events.drain(..).collect();
            let fin = ch.finished.take();
            (evs, fin)
        };

        let mut callback_err: Option<i32> = None;
        for ev in events {
            match ev {
                TransferEvent::Head(meta) => apply_head_meta(&easy_arc, meta),
                TransferEvent::HeaderLine(line) => {
                    if callback_err.is_none() {
                        if let Err(code) = invoke_header_callback(cb, &line) {
                            callback_err = Some(code);
                            cancel.store(true, Ordering::Release);
                        }
                    }
                }
                TransferEvent::BodyChunk(data) => {
                    if callback_err.is_none() {
                        if let Err(code) = invoke_write_callback(cb, &data) {
                            callback_err = Some(code);
                            cancel.store(true, Ordering::Release);
                        }
                    }
                }
            }
        }

        if let Some(code) = callback_err {
            let outcome = finished.map_or_else(
                || TransferOutcome {
                    code,
                    error_msg: Some("Write/header callback aborted transfer".to_string()),
                    size_download: 0,
                    size_upload: 0,
                    content_length_upload: 0,
                    total_time: 0.0,
                    os_errno: 0,
                    ssl_verifyresult: 0,
                },
                |mut o| {
                    o.code = code;
                    o.error_msg = Some("Write/header callback aborted transfer".to_string());
                    o
                },
            );
            apply_outcome(&easy_arc, cb, &outcome);
            let mut inner = m.inner.lock().unwrap();
            if let Some(entry) = inner.entries.get_mut(&easy_id) {
                entry.state = MultiEntryState::Done;
                inner.done_queue.push_back(Box::new(CurlMsg {
                    msg: CURLMSG_DONE,
                    _pad: 0,
                    easy_handle: easy_id,
                    data: (code as u32) as usize,
                }));
            }
        } else if let Some(outcome) = finished {
            apply_outcome(&easy_arc, cb, &outcome);
            let mut inner = m.inner.lock().unwrap();
            if let Some(entry) = inner.entries.get_mut(&easy_id) {
                entry.state = MultiEntryState::Done;
                inner.done_queue.push_back(Box::new(CurlMsg {
                    msg: CURLMSG_DONE,
                    _pad: 0,
                    easy_handle: easy_id,
                    data: (outcome.code as u32) as usize,
                }));
            }
        }
    }

    let still_running = {
        let inner = m.inner.lock().unwrap();
        inner
            .entries
            .values()
            .filter(|e| !matches!(e.state, MultiEntryState::Done))
            .count() as i32
    };
    if !running_handles.is_null() {
        std::ptr::write_unaligned(running_handles, still_running);
    }
    CURLM_OK
}

pub(super) fn multi_wait_impl(
    multi_id: usize,
    extra_nfds: u32,
    timeout_ms: i32,
    numfds: *mut i32,
    is_poll: bool,
) -> i32 {
    let Some(multi) = MULTI_HANDLES.get_cloned(multi_id) else {
        if !numfds.is_null() {
            unsafe { std::ptr::write_unaligned(numfds, 0) };
        }
        return CURLM_BAD_HANDLE;
    };

    let start_seq = multi.notify_lock.lock().unwrap().seq;

    let (has_active, has_ready) = {
        let inner = multi.inner.lock().unwrap();
        let mut active = false;
        // Only active transfer readiness (Pending start or undrained worker events/completion)
        // wakes curl_multi_poll/wait immediately; unread messages already in done_queue must NOT
        // busy-spin callers looping `while running > 0 { perform(); poll(); }`.
        let mut ready = false;
        for entry in inner.entries.values() {
            match &entry.state {
                MultiEntryState::Pending => {
                    active = true;
                    ready = true;
                }
                MultiEntryState::Starting => {
                    active = true;
                }
                MultiEntryState::Running { channel, .. } => {
                    active = true;
                    let ch = channel.lock().unwrap();
                    if !ch.events.is_empty() || ch.finished.is_some() {
                        ready = true;
                    }
                }
                MultiEntryState::Done => {}
            }
        }
        (active, ready)
    };

    if has_ready {
        if !numfds.is_null() {
            unsafe { std::ptr::write_unaligned(numfds, 1) };
        }
        return CURLM_OK;
    }

    // Documented curl_multi_wait behavior: returns immediately when no active transfers
    // and no extra_fds exist, whereas curl_multi_poll still sleeps up to timeout_ms.
    if !is_poll && !has_active && extra_nfds == 0 {
        if !numfds.is_null() {
            unsafe { std::ptr::write_unaligned(numfds, 0) };
        }
        return CURLM_OK;
    }

    let timeout = if timeout_ms <= 0 {
        Duration::ZERO
    } else {
        Duration::from_millis(timeout_ms as u64)
    };
    let deadline = Instant::now() + timeout;

    let mut guard = multi.notify_lock.lock().unwrap();
    let mut activity = false;
    loop {
        if guard.woken {
            guard.woken = false;
            activity = true;
            break;
        }
        if guard.seq != start_seq {
            activity = true;
            break;
        }
        let now = Instant::now();
        if now >= deadline {
            break;
        }
        let (new_guard, wait_res) = multi
            .notify_cv
            .wait_timeout(guard, deadline.saturating_duration_since(now))
            .unwrap();
        guard = new_guard;
        if wait_res.timed_out() && !guard.woken && guard.seq == start_seq {
            break;
        }
    }

    if !numfds.is_null() {
        unsafe { std::ptr::write_unaligned(numfds, i32::from(activity)) };
    }
    CURLM_OK
}

pub(super) fn multi_wakeup(multi: usize) -> i32 {
    let Some(m) = MULTI_HANDLES.get_cloned(multi) else {
        return CURLM_BAD_HANDLE;
    };
    m.wakeup();
    CURLM_OK
}

pub(super) unsafe fn multi_info_read(multi: usize, msgs_in_queue: *mut i32) -> usize {
    let Some(m) = MULTI_HANDLES.get_cloned(multi) else {
        if !msgs_in_queue.is_null() {
            std::ptr::write_unaligned(msgs_in_queue, 0);
        }
        return 0;
    };
    let mut inner = m.inner.lock().unwrap();
    let msg = inner.done_queue.pop_front();
    let remaining = inner.done_queue.len() as i32;
    if !msgs_in_queue.is_null() {
        std::ptr::write_unaligned(msgs_in_queue, remaining);
    }
    if let Some(boxed) = msg {
        let ptr = (&*boxed as *const CurlMsg) as usize;
        inner.delivered_msgs.push(boxed);
        ptr
    } else {
        0
    }
}

pub(super) fn multi_setopt(multi: usize, option: i32, param: usize) -> i32 {
    if !MULTI_HANDLES.contains(multi) {
        return CURLM_BAD_HANDLE;
    }
    log::trace!("libcurl: curl_multi_setopt(option={option}, param={param:#x}) -> CURLM_OK");
    CURLM_OK
}

pub(super) fn multi_strerror(code: i32) -> *const u8 {
    match code {
        CURLM_OK => b"No error\0".as_ptr(),
        CURLM_BAD_HANDLE => b"Invalid multi handle\0".as_ptr(),
        CURLM_BAD_EASY_HANDLE => b"Invalid easy handle\0".as_ptr(),
        CURLM_OUT_OF_MEMORY => b"Out of memory\0".as_ptr(),
        CURLM_INTERNAL_ERROR => b"Internal error\0".as_ptr(),
        CURLM_BAD_SOCKET => b"Invalid socket argument\0".as_ptr(),
        CURLM_UNKNOWN_OPTION => b"Unknown option\0".as_ptr(),
        CURLM_ADDED_ALREADY => b"The easy handle is already added to a multi handle\0".as_ptr(),
        CURLM_BAD_FUNCTION_ARGUMENT => b"A libcurl function was given a bad argument\0".as_ptr(),
        _ => b"Unknown CURLM error\0".as_ptr(),
    }
}
