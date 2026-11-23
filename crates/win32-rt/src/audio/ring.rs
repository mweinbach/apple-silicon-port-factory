//! Lock-free single-producer / single-consumer sample ring.
//!
//! Producer: the game's audio thread in `IAudioRenderClient::ReleaseBuffer`.
//! Consumer: the CoreAudio IO thread (render callback) or the null-device
//! clock thread. The consumer never blocks, allocates or takes locks.
//! `clear` is only called while the stream is stopped (`IAudioClient::Reset`).

use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

#[repr(align(128))]
struct CachePadded<T>(T);

pub struct Ring {
    buf: Box<[UnsafeCell<f32>]>,
    channels: usize,
    cap_frames: usize,
    /// Monotonic frame counters on separate 128-byte cache lines to avoid
    /// false sharing between the game audio thread and CoreAudio's RT thread.
    write: CachePadded<AtomicUsize>,
    read: CachePadded<AtomicUsize>,
}

unsafe impl Sync for Ring {}
unsafe impl Send for Ring {}

impl Ring {
    pub fn new(cap_frames: usize, channels: usize) -> Ring {
        let n = cap_frames.max(1) * channels.max(1);
        Ring {
            buf: (0..n).map(|_| UnsafeCell::new(0.0)).collect(),
            channels: channels.max(1),
            cap_frames: cap_frames.max(1),
            write: CachePadded(AtomicUsize::new(0)),
            read: CachePadded(AtomicUsize::new(0)),
        }
    }

    pub fn channels(&self) -> usize {
        self.channels
    }
    pub fn capacity(&self) -> usize {
        self.cap_frames
    }

    /// Frames queued and not yet consumed (= WASAPI padding).
    pub fn len(&self) -> usize {
        let w = self.write.0.load(Ordering::Acquire);
        let r = self.read.0.load(Ordering::Acquire);
        w.wrapping_sub(r)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn free(&self) -> usize {
        self.cap_frames - self.len().min(self.cap_frames)
    }
    /// Total frames ever consumed (device position).
    pub fn consumed(&self) -> usize {
        self.read.0.load(Ordering::Acquire)
    }

    /// Producer: append interleaved frames. Returns frames written (all, unless full).
    pub fn push(&self, samples: &[f32]) -> usize {
        let ch = self.channels;
        let frames = samples.len() / ch;
        let w = self.write.0.load(Ordering::Relaxed);
        let r = self.read.0.load(Ordering::Acquire);
        let free = self.cap_frames - w.wrapping_sub(r);
        let n = frames.min(free);
        if n > 0 {
            let start = w % self.cap_frames;
            let first = n.min(self.cap_frames - start);
            let second = n - first;
            let dst = self.buf.as_ptr() as *mut f32;
            unsafe {
                std::ptr::copy_nonoverlapping(samples.as_ptr(), dst.add(start * ch), first * ch);
                if second > 0 {
                    std::ptr::copy_nonoverlapping(samples.as_ptr().add(first * ch), dst, second * ch);
                }
            }
            self.write.0.store(w.wrapping_add(n), Ordering::Release);
        }
        n
    }

    /// Producer: append `frames` frames of silence.
    pub fn push_silence(&self, frames: usize) -> usize {
        let ch = self.channels;
        let w = self.write.0.load(Ordering::Relaxed);
        let r = self.read.0.load(Ordering::Acquire);
        let n = frames.min(self.cap_frames - w.wrapping_sub(r));
        if n > 0 {
            let start = w % self.cap_frames;
            let first = n.min(self.cap_frames - start);
            let second = n - first;
            let dst = self.buf.as_ptr() as *mut f32;
            unsafe {
                // IEEE 754 +0.0f32 is all-zero bytes.
                std::ptr::write_bytes(dst.add(start * ch), 0, first * ch);
                if second > 0 {
                    std::ptr::write_bytes(dst, 0, second * ch);
                }
            }
            self.write.0.store(w.wrapping_add(n), Ordering::Release);
        }
        n
    }

    /// Consumer: fill `out` (interleaved, `out.len() / channels` frames).
    /// Missing frames are zero-filled. Returns frames actually taken.
    pub fn pop_into(&self, out: &mut [f32]) -> usize {
        let ch = self.channels;
        let frames = out.len() / ch;
        let r = self.read.0.load(Ordering::Relaxed);
        let w = self.write.0.load(Ordering::Acquire);
        let n = frames.min(w.wrapping_sub(r));
        if n > 0 {
            let start = r % self.cap_frames;
            let first = n.min(self.cap_frames - start);
            let second = n - first;
            let src = self.buf.as_ptr() as *const f32;
            unsafe {
                std::ptr::copy_nonoverlapping(src.add(start * ch), out.as_mut_ptr(), first * ch);
                if second > 0 {
                    std::ptr::copy_nonoverlapping(src, out.as_mut_ptr().add(first * ch), second * ch);
                }
            }
            self.read.0.store(r.wrapping_add(n), Ordering::Release);
        }
        out[n * ch..].fill(0.0);
        n
    }

    /// Consumer: drop up to `frames` frames (null device). Returns frames dropped.
    pub fn discard(&self, frames: usize) -> usize {
        let r = self.read.0.load(Ordering::Relaxed);
        let w = self.write.0.load(Ordering::Acquire);
        let n = frames.min(w.wrapping_sub(r));
        self.read.0.store(r.wrapping_add(n), Ordering::Release);
        n
    }

    /// Drop all queued frames (stream must be stopped). Keeps `consumed()` monotonic.
    pub fn clear(&self) {
        let w = self.write.0.load(Ordering::Acquire);
        self.read.0.store(w, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_and_zero_fills() {
        let r = Ring::new(4, 2);
        assert_eq!(r.push(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]), 3);
        assert_eq!(r.len(), 3);
        let mut o = [9.0; 4];
        assert_eq!(r.pop_into(&mut o), 2);
        assert_eq!(o, [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(r.push(&[7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0]), 3); // 1 queued + 3 = full
        assert_eq!(r.free(), 0);
        let mut o = [9.0; 10];
        assert_eq!(r.pop_into(&mut o), 4);
        assert_eq!(o, [5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 0.0, 0.0]);
        assert_eq!(r.consumed(), 6);
        r.push_silence(2);
        r.clear();
        assert!(r.is_empty());
    }

    #[test]
    fn spsc_threads_preserve_order() {
        let r = std::sync::Arc::new(Ring::new(256, 1));
        let p = r.clone();
        let t = std::thread::spawn(move || {
            let mut i = 0u32;
            while i < 100_000 {
                if p.push(&[i as f32]) == 1 {
                    i += 1;
                } else {
                    std::thread::yield_now();
                }
            }
        });
        let mut next = 0u32;
        let mut o = [0.0f32; 64];
        while next < 100_000 {
            let n = r.pop_into(&mut o);
            for &v in &o[..n] {
                assert_eq!(v, next as f32);
                next += 1;
            }
        }
        t.join().unwrap();
    }

    #[test]
    fn write_and_read_are_on_separate_cache_lines_and_silence_wraps() {
        let r = Ring::new(4, 2);
        let w_addr = &r.write as *const _ as usize;
        let r_addr = &r.read as *const _ as usize;
        assert!(
            w_addr.abs_diff(r_addr) >= 128,
            "write ({w_addr:#x}) and read ({r_addr:#x}) share a 128-byte cache line"
        );
        // Exercise push_silence wrap-around across the capacity boundary.
        assert_eq!(r.push(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]), 3);
        let mut o = [9.0; 4];
        assert_eq!(r.pop_into(&mut o), 2);
        assert_eq!(o, [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(r.push_silence(3), 3); // 1 frame at tail + 2 frames wrapped at head
        let mut o2 = [9.0; 8];
        assert_eq!(r.pop_into(&mut o2), 4);
        assert_eq!(o2, [5.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }
}
