//! `--stats` and `--dump`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use d3d12_metal::trace::{Call, Reader, Record};

#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub records: u64,
    pub per_op: BTreeMap<&'static str, u64>,
    pub frames: u64,
    pub threads: BTreeSet<u16>,
    pub duration_ns: u64,
    pub draws: u64,
    pub dispatches: u64,
    pub indirect: u64,
    pub created: u64,
    pub released: u64,
    pub upload_bytes: u64,
    pub decode_errors: Vec<(u64, String)>,
    pub truncated: bool,
    pub bytes: usize,
}

fn is_creation(c: &Call) -> bool {
    matches!(
        c,
        Call::CreateDevice { .. }
            | Call::CreateCommandQueue { .. }
            | Call::CreateCommandAllocator { .. }
            | Call::CreateCommandList { .. }
            | Call::CreateDescriptorHeap { .. }
            | Call::CreateHeap { .. }
            | Call::CreateCommittedResource { .. }
            | Call::CreatePlacedResource { .. }
            | Call::CreateReservedResource { .. }
            | Call::CreateRootSignature { .. }
            | Call::CreateGraphicsPipelineState { .. }
            | Call::CreateComputePipelineState { .. }
            | Call::CreateMeshPipelineState { .. }
            | Call::CreateStateObject { .. }
            | Call::AddToStateObject { .. }
            | Call::CreateCommandSignature { .. }
            | Call::CreateFence { .. }
            | Call::CreateQueryHeap { .. }
            | Call::CreateSwapChain { .. }
    )
}

pub fn stats(data: &[u8], until: Option<u64>) -> anyhow::Result<Stats> {
    let mut reader = Reader::new(data).map_err(|e| anyhow::anyhow!("{}", e.0))?;
    let mut s = Stats { bytes: data.len(), ..Default::default() };
    let mut next = 0u64;
    for r in reader.by_ref() {
        let rec = match r {
            Ok(rec) => rec,
            Err(e) => {
                s.decode_errors.push((next, e.0));
                next += 1;
                continue;
            }
        };
        next = rec.index + 1;
        if until.is_some_and(|u| rec.index > u) {
            break;
        }
        s.records += 1;
        s.threads.insert(rec.thread);
        s.duration_ns = s.duration_ns.max(rec.t_ns);
        *s.per_op.entry(rec.call.name()).or_insert(0) += 1;
        match &rec.call {
            Call::Present { .. } => s.frames += 1,
            Call::DrawInstanced { .. } | Call::DrawIndexedInstanced { .. } | Call::DispatchMesh { .. } => s.draws += 1,
            Call::Dispatch { .. } | Call::DispatchRays { .. } => s.dispatches += 1,
            Call::ExecuteIndirect { .. } => s.indirect += 1,
            Call::Release { .. } => s.released += 1,
            Call::WriteResource { data, .. } => s.upload_bytes += data.len() as u64,
            c if is_creation(c) => s.created += 1,
            _ => {}
        }
    }
    s.truncated = reader.truncated();
    Ok(s)
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secs = self.duration_ns as f64 / 1e9;
        writeln!(f, "trace: {} bytes, {} records, {} threads, {:.3} s{}", self.bytes, self.records, self.threads.len(), secs, if self.truncated { " (TRUNCATED: torn final record)" } else { "" })?;
        let per_frame = |n: u64| if self.frames > 0 { format!(" ({:.1}/frame)", n as f64 / self.frames as f64) } else { String::new() };
        writeln!(f, "  frames {}{}", self.frames, if secs > 0.0 && self.frames > 0 { format!(" ({:.1} fps)", self.frames as f64 / secs) } else { String::new() })?;
        writeln!(f, "  draws {}{}, dispatches {}{}, indirect {}", self.draws, per_frame(self.draws), self.dispatches, per_frame(self.dispatches), self.indirect)?;
        writeln!(f, "  objects created {}, released {}; CPU upload writes {} bytes", self.created, self.released, self.upload_bytes)?;
        let mut ops: Vec<_> = self.per_op.iter().collect();
        ops.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        for (op, n) in ops {
            writeln!(f, "  {n:>10}  {op}")?;
        }
        if !self.decode_errors.is_empty() {
            writeln!(f, "  decode errors: {}", self.decode_errors.len())?;
            for (i, e) in self.decode_errors.iter().take(20) {
                writeln!(f, "    #{i}: {e}")?;
            }
        }
        Ok(())
    }
}

/// Collapses long numeric arrays in `Debug` output (blobs) to `<N items>`.
fn compact_debug(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = String::with_capacity(s.len().min(512));
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'[' {
            // Scan a run of digits / commas / spaces / '-' / '.' up to ']'.
            let mut j = i + 1;
            let mut items = 0usize;
            let mut in_num = false;
            while j < b.len() && matches!(b[j], b'0'..=b'9' | b',' | b' ' | b'-' | b'.' | b'e' | b'x' | b'a'..=b'd' | b'f') {
                let digit = b[j].is_ascii_alphanumeric() || b[j] == b'-' || b[j] == b'.';
                if digit && !in_num {
                    items += 1;
                }
                in_num = digit;
                j += 1;
            }
            if j < b.len() && b[j] == b']' && items > 8 {
                let _ = write!(out, "<{items} items>");
                i = j + 1;
                continue;
            }
        }
        let ch = s[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

/// One line per record.
pub fn dump_line(rec: &Record) -> String {
    let mut body = compact_debug(&format!("{:?}", rec.call));
    // Drop the variant name (printed separately) and hex-format ids would be nicer, but the
    // Debug form stays exhaustive without per-call code.
    if let Some(p) = body.find(" {") {
        body.drain(..p + 1);
    } else {
        body.clear();
    }
    if body.len() > 400 {
        let mut cut = 400;
        while !body.is_char_boundary(cut) {
            cut -= 1;
        }
        body.truncate(cut);
        body.push_str(" ...");
    }
    format!("{:>9} t{:<3} {:>12.3}ms {} {}", rec.index, rec.thread, rec.t_ns as f64 / 1e6, rec.call.name(), body)
}

pub fn dump(data: &[u8], until: Option<u64>, out: &mut dyn std::io::Write) -> anyhow::Result<()> {
    let mut reader = Reader::new(data).map_err(|e| anyhow::anyhow!("{}", e.0))?;
    let mut next = 0u64;
    for r in reader.by_ref() {
        match r {
            Ok(rec) => {
                next = rec.index + 1;
                if until.is_some_and(|u| rec.index > u) {
                    break;
                }
                writeln!(out, "{}", dump_line(&rec))?;
            }
            Err(e) => {
                writeln!(out, "{next:>9} DECODE ERROR: {}", e.0)?;
                next += 1;
            }
        }
    }
    if reader.truncated() {
        writeln!(out, "(trace truncated: torn final record)")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compacts_long_arrays_only() {
        assert_eq!(compact_debug("x: [1, 2, 3]"), "x: [1, 2, 3]");
        let long = format!("blob: {:?}, y: 5", (0..40u8).collect::<Vec<_>>());
        assert_eq!(compact_debug(&long), "blob: <40 items>, y: 5");
        assert_eq!(compact_debug("f: [0.25, 0.5, 0.75, 1.0] é"), "f: [0.25, 0.5, 0.75, 1.0] é");
    }
}
