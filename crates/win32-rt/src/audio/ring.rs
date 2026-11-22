//! Lock-free single-producer / single-consumer sample ring.
//!
//! Producer: the game's audio thread in `IAudioRenderClient::ReleaseBuffer`.
//! Consumer: the CoreAudio IO thread (render callback) or the null-device
//! clock thread. The consumer never blocks, allocates or takes locks.
//! `clear` is only called while the stream is stopped (`IAudioClient::Reset`).

use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Ring {
    buf: Box<[UnsafeCell<f32>]>,
    channels: usize,
    cap_frames: usize,
    /// Monotonic frame counters (wrap at usize::MAX; differences are what matter).
    write: AtomicUsize,
    read: AtomicUsize,
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
            write: AtomicUsize::new(0),
            read: AtomicUsize::new(0),
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
        let w = self.write.load(Ordering::Acquire);
        let r = self.read.load(Ordering::Acquire);
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
        self.read.load(Ordering::Acquire)
    }

    /// Producer: append interleaved frames. Returns frames written (all, unless full).
    pub fn push(&self, samples: &[f32]) -> usize {
        let frames = samples.len() / self.channels;
        let w = self.write.load(Ordering::Relaxed);
        let r = self.read.load(Ordering::Acquire);
        let free = self.cap_frames - w.wrapping_sub(r);
        let n = frames.min(free);
        for f in 0..n {
            let slot = (w.wrapping_add(f) % self.cap_frames) * self.channels;
            for c in 0..self.channels {
                unsafe { *self.buf[slot + c].get() = samples[f * self.channels + c] };
            }
        }
        self.write.store(w.wrapping_add(n), Ordering::Release);
        n
    }

    /// Producer: append `frames` frames of silence.
    pub fn push_silence(&self, frames: usize) -> usize {
        let w = self.write.load(Ordering::Relaxed);
        let r = self.read.load(Ordering::Acquire);
        let n = frames.min(self.cap_frames - w.wrapping_sub(r));
        for f in 0..n {
            let slot = (w.wrapping_add(f) % self.cap_frames) * self.channels;
            for c in 0..self.channels {
                unsafe { *self.buf[slot + c].get() = 0.0 };
            }
        }
        self.write.store(w.wrapping_add(n), Ordering::Release);
        n
    }

    /// Consumer: fill `out` (interleaved, `out.len() / channels` frames).
    /// Missing frames are zero-filled. Returns frames actually taken.
    pub fn pop_into(&self, out: &mut [f32]) -> usize {
        let frames = out.len() / self.channels;
        let r = self.read.load(Ordering::Relaxed);
        let w = self.write.load(Ordering::Acquire);
        let n = frames.min(w.wrapping_sub(r));
        for f in 0..n {
            let slot = (r.wrapping_add(f) % self.cap_frames) * self.channels;
            for c in 0..self.channels {
                out[f * self.channels + c] = unsafe { *self.buf[slot + c].get() };
            }
        }
        out[n * self.channels..].fill(0.0);
        self.read.store(r.wrapping_add(n), Ordering::Release);
        n
    }

    /// Consumer: drop up to `frames` frames (null device). Returns frames dropped.
    pub fn discard(&self, frames: usize) -> usize {
        let r = self.read.load(Ordering::Relaxed);
        let w = self.write.load(Ordering::Acquire);
        let n = frames.min(w.wrapping_sub(r));
        self.read.store(r.wrapping_add(n), Ordering::Release);
        n
    }

    /// Drop all queued frames (stream must be stopped). Keeps `consumed()` monotonic.
    pub fn clear(&self) {
        let w = self.write.load(Ordering::Acquire);
        self.read.store(w, Ordering::Release);
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
}
