//! RanSeq container selection, ported from the decompile [V unless marked].
//!
//! Runtime container fields (CAkRanSeqCntr): `+0x128` transition time (ms), `+0x12c/+0x130`
//! transition modifier min/max, `+0x134` loop count, `+0x136/+0x138` loop modifier min/max,
//! `+0x13a` avoid-repeat count, `+0x13c` transition mode (bits 0-3) and random mode (bits 4-5:
//! 0 standard, 0x10 shuffle), `+0x13d` bits: 0-2 mode (1 = sequence), 3 using weight,
//! 5 restart backward, 6 continuous, 7 global. Children are indexed in playlist order, child
//! weight = playlist weight.
//!
//! * [`RandomInfo`] = `CAkRandomInfo` (ctor `FUN_1415ccd20`, alloc `FUN_1415cd430`, created by
//!   `FUN_1415f9b80`), selection `FUN_1415fb3a0`, avoid-repeat bookkeeping `FUN_1415fc1f0`.
//! * [`SeqInfo`] = sequence info (ctor `FUN_1415ccd60`: forward, index -1); step logic inline in
//!   `FUN_1415fa360`.
//! * [`LoopInfo`] = continuous-play loop counter (`FUN_1415f9720`).

use super::rng::AkRng;
use crate::bnk::hirc::RanSeqCntr;

/// Per-(container, game object) random state.
#[derive(Debug, Clone)]
pub struct RandomInfo {
    /// `+0x08`: total weight (`n * 50000`, or the playlist weight sum when using weights).
    pub total_w: u32,
    /// `+0x0c`: weight still selectable.
    pub rem_w: u32,
    /// `+0x10`: number of items still selectable.
    pub rem_cnt: u16,
    /// `+0x12`: selections left in the current cycle (reset to `n` when it reaches 0).
    pub cycle: u16,
    /// `+0x18`: avoid-repeat history (oldest first).
    pub hist: Vec<u16>,
    /// `+0x28` bitset: played in this cycle.
    pub played: Vec<bool>,
    /// `+0x30` bitset: blocked by the avoid-repeat history.
    pub blocked: Vec<bool>,
}

/// Loop counter of a continuous play (`FUN_1415f9720`; entry `+0x10` short count, `+0x12` flags:
/// bit 0 looping, bit 1 infinite).
#[derive(Debug, Clone, Copy)]
pub struct LoopInfo {
    pub count: i16,
    pub looping: bool,
    pub infinite: bool,
}

impl LoopInfo {
    /// Created when a continuous container starts; draws the loop modifier when the loop count is
    /// finite and `max - min != 0`.
    pub fn new(c: &RanSeqCntr, rng: &mut AkRng) -> Self {
        if c.loop_count != 0 {
            let lo = c.loop_mod_min as i16;
            let hi = c.loop_mod_max as i16;
            let off = rng.loop_offset(hi.wrapping_sub(lo));
            let n = lo.wrapping_add(c.loop_count as i16).wrapping_add(off);
            LoopInfo { count: n.max(1), looping: n > 1, infinite: false }
        } else {
            LoopInfo { count: 1, looping: true, infinite: true }
        }
    }

    /// End-of-cycle test from `FUN_1415fb3a0`: returns false when the play is over.
    pub fn next_cycle(&mut self) -> bool {
        if !self.looping {
            return false;
        }
        if !self.infinite {
            self.count -= 1;
            if self.count == 0 {
                return false;
            }
        }
        true
    }
}

fn weight(c: &RanSeqCntr, i: usize) -> u32 {
    c.playlist[i].1 as u32
}

fn using_weight(c: &RanSeqCntr) -> bool {
    c.bits & 0x01 != 0
}

fn shuffle(c: &RanSeqCntr) -> bool {
    c.random_mode == 1
}

impl RandomInfo {
    pub fn new(c: &RanSeqCntr) -> Self {
        let n = c.playlist.len();
        let total = if using_weight(c) { (0..n).map(|i| weight(c, i)).fold(0u32, |a, w| a.wrapping_add(w)) } else { n as u32 * 50000 };
        RandomInfo {
            total_w: total,
            rem_w: total,
            rem_cnt: n as u16,
            cycle: n as u16,
            hist: Vec::new(),
            played: vec![false; n],
            blocked: vec![false; n],
        }
    }

    fn available(&self, c: &RanSeqCntr, i: usize) -> bool {
        if shuffle(c) {
            !self.played[i] && !self.blocked[i]
        } else if c.avoid_repeat_count == 0 {
            true
        } else {
            !self.blocked[i]
        }
    }

    /// `FUN_1415fb3a0`: pick the next index; `None` when the loop info ends the play.
    pub fn select(&mut self, c: &RanSeqCntr, rng: &mut AkRng, li: Option<&mut LoopInfo>) -> Option<usize> {
        let n = c.playlist.len();
        if self.cycle == 0 {
            if let Some(li) = li {
                if !li.next_cycle() {
                    return None;
                }
            }
            self.cycle = n as u16;
            self.played.iter_mut().for_each(|p| *p = false);
            let mut w = self.total_w;
            let mut cnt = n as u16;
            for &h in &self.hist {
                w = w.wrapping_sub(weight(c, h as usize));
                cnt = cnt.wrapping_sub(1);
                self.blocked[h as usize] = true;
            }
            if shuffle(c) {
                self.rem_w = w;
            }
            self.rem_cnt = cnt;
        }
        let mut idx = 0usize;
        if !using_weight(c) {
            let r = rng.next();
            let target = (r % self.rem_cnt.max(1) as u32) as u16 as i32;
            let mut acc = -1i32;
            loop {
                if idx >= n {
                    break; // AK reads past the end here; unreachable with consistent state
                }
                if self.available(c, idx) {
                    acc += 1;
                }
                idx += 1;
                if acc >= target {
                    break;
                }
            }
        } else {
            let r = rng.next();
            let target = (r as u64 % self.rem_w.max(1) as u64) as i32;
            let mut acc = -1i32;
            loop {
                if idx >= n {
                    break;
                }
                if self.available(c, idx) {
                    acc = acc.wrapping_add(weight(c, idx) as i32);
                }
                idx += 1;
                if acc >= target {
                    break;
                }
            }
        }
        let sel = idx.saturating_sub(1);
        if !shuffle(c) {
            if !self.played[sel] {
                self.played[sel] = true;
                self.cycle = self.cycle.wrapping_sub(1);
            }
            self.avoid_repeat_standard(c, sel);
        } else {
            let avoid = if c.avoid_repeat_count != 0 { c.avoid_repeat_count } else { 1 };
            self.rem_cnt = self.rem_cnt.wrapping_sub(1);
            self.rem_w = self.rem_w.wrapping_sub(weight(c, sel));
            self.cycle = self.cycle.wrapping_sub(1);
            self.played[sel] = true;
            self.hist.push(sel as u16);
            self.blocked[sel] = true;
            let maxh = avoid.min((n as u16).wrapping_sub(1));
            if self.hist.len() > maxh as usize {
                let h0 = self.hist.remove(0) as usize;
                self.blocked[h0] = false;
                if !self.played[h0] {
                    self.rem_w = self.rem_w.wrapping_add(weight(c, h0));
                    self.rem_cnt = self.rem_cnt.wrapping_add(1);
                }
            }
        }
        Some(sel)
    }

    /// `FUN_1415fc1f0` (standard mode).
    fn avoid_repeat_standard(&mut self, c: &RanSeqCntr, sel: usize) {
        if c.avoid_repeat_count == 0 {
            return;
        }
        let n = c.playlist.len() as u16;
        self.rem_cnt = self.rem_cnt.wrapping_sub(1);
        self.hist.push(sel as u16);
        self.blocked[sel] = true;
        self.rem_w = self.rem_w.wrapping_sub(weight(c, sel));
        let maxh = c.avoid_repeat_count.min(n.wrapping_sub(1));
        if (maxh as usize) < self.hist.len() {
            let h0 = self.hist.remove(0) as usize;
            self.blocked[h0] = false;
            self.rem_w = self.rem_w.wrapping_add(weight(c, h0));
            self.rem_cnt = self.rem_cnt.wrapping_add(1);
        }
    }
}

/// Sequence state (`FUN_1415ccd60`: forward = 1, index = -1).
#[derive(Debug, Clone, Copy)]
pub struct SeqInfo {
    pub forward: bool,
    pub idx: i16,
}

impl Default for SeqInfo {
    fn default() -> Self {
        SeqInfo { forward: true, idx: -1 }
    }
}

impl SeqInfo {
    /// `FUN_1415fb720` (and the identical inline step of `FUN_1415fa360` with no loop info): the
    /// next index, or `None` when wrapping ends the continuous play.
    pub fn next(&mut self, c: &RanSeqCntr, li: Option<&mut LoopInfo>) -> Option<usize> {
        let n = c.playlist.len() as i32;
        if self.forward {
            if self.idx as i32 + 1 != n {
                self.idx += 1;
                return Some(self.idx as usize);
            }
            if c.bits & 0x04 != 0 {
                // restart backward
                self.forward = false;
                self.idx -= 1;
                return Some(self.idx.max(0) as usize);
            }
            self.idx = 0;
        } else {
            if self.idx != 0 {
                self.idx -= 1;
                return Some(self.idx as usize);
            }
            self.idx = 1;
            self.forward = true;
        }
        // wrapped: one loop done
        if let Some(li) = li {
            if !li.looping {
                return None;
            }
            if !li.infinite {
                li.count -= 1;
                if li.count == 0 {
                    return None;
                }
            }
        }
        Some(self.idx as usize)
    }
}

/// Single-item continuous container (`FUN_1415fa780`, `n == 1`): no draw; the item plays `count`
/// times (infinite: forever).
pub fn single_item_next(li: &mut LoopInfo) -> Option<usize> {
    if li.count < 1 {
        return None;
    }
    if !li.infinite {
        li.count -= 1;
    }
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bnk::hirc::NodeBase;

    fn cntr(n: usize, random_mode: u8, avoid: u16, bits: u8) -> RanSeqCntr {
        RanSeqCntr {
            node: NodeBase::default(),
            loop_count: 1,
            loop_mod_min: 0,
            loop_mod_max: 0,
            transition_time: 0.0,
            transition_time_mod_min: 0.0,
            transition_time_mod_max: 0.0,
            avoid_repeat_count: avoid,
            transition_mode: 0,
            random_mode,
            mode: 0,
            bits,
            children: (0..n as u32).collect(),
            playlist: (0..n as u32).map(|i| (i, 50000)).collect(),
        }
    }

    #[test]
    fn shuffle_plays_each_once_per_cycle() {
        let c = cntr(5, 1, 0, 0);
        let mut ri = RandomInfo::new(&c);
        let mut rng = AkRng::new(0x5eed);
        let mut seen: Vec<usize> = (0..5).map(|_| ri.select(&c, &mut rng, None).unwrap()).collect();
        seen.sort();
        assert_eq!(seen, vec![0, 1, 2, 3, 4]);
        assert_eq!(rng.draws, 5);
    }

    #[test]
    fn avoid_repeat_blocks_last() {
        let c = cntr(3, 0, 1, 0);
        let mut ri = RandomInfo::new(&c);
        let mut rng = AkRng::new(1);
        let mut last = ri.select(&c, &mut rng, None).unwrap();
        for _ in 0..50 {
            let s = ri.select(&c, &mut rng, None).unwrap();
            assert_ne!(s, last);
            last = s;
        }
    }

    #[test]
    fn sequence_restart_backward() {
        let c = cntr(3, 0, 0, 0x04);
        let mut s = SeqInfo::default();
        let v: Vec<usize> = (0..7).map(|_| s.next(&c, None).unwrap()).collect();
        assert_eq!(v, vec![0, 1, 2, 1, 0, 1, 2]);
    }
}
