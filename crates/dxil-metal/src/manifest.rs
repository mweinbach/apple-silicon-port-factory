//! Shader manifest: a converter-independent record of the compile requests the game actually
//! made (DXIL, root signatures, entry point, stage, options), so a later `dxil-metal prewarm
//! --manifest` can warm exactly the cache keys the game will ask for, including material
//! shaders that only exist at PSO-creation time.
//!
//! Layout of a manifest directory:
//! - `records.aw2m`: append-only records, each `u32 len | payload | u64 xxh64(payload)`;
//!   a torn or corrupt tail is ignored on load.
//! - `dxil/<xx>/<hash128>.dxil`: DXIL containers, content-addressed.
//! - `rs/<hash128>.rts0`: root signatures (canonical RTS0), keyed by
//!   [`RootSignature::hash`].
//!
//! [`ShaderPipeline::global`](crate::ShaderPipeline::global) records every
//! [`compile`](crate::ShaderPipeline::compile) automatically (see [`ManifestWriter::from_env`]);
//! other pipelines can opt in with [`ShaderPipeline::set_manifest`](crate::ShaderPipeline::set_manifest).

use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::compiler::{CompileOptions, CompileRequest};
use crate::hash::{hash128, xxh64};
use crate::reflection::ShaderStage;
use crate::root_signature::RootSignature;
use crate::wire::{self, Reader, Writer};
use crate::Error;

const RECORDS: &str = "records.aw2m";
const RECORD_VERSION: u8 = 1;

/// One recorded compile request.
#[derive(Debug, Clone, PartialEq)]
pub struct ManifestEntry {
    /// Converter-independent request key (cache key without the converter fingerprint).
    pub key: u128,
    pub dxil_hash: u128,
    pub root_signature: Option<u128>,
    pub local_root_signature: Option<u128>,
    pub entry_point: Option<String>,
    pub expected_stage: Option<ShaderStage>,
    pub options: CompileOptions,
}

fn io(path: &Path, e: std::io::Error) -> Error {
    Error::Io(format!("{}: {e}", path.display()))
}

fn dxil_path(dir: &Path, hash: u128) -> PathBuf {
    let hex = format!("{hash:032x}");
    dir.join("dxil").join(&hex[..2]).join(format!("{hex}.dxil"))
}

fn rs_path(dir: &Path, hash: u128) -> PathBuf {
    dir.join("rs").join(format!("{hash:032x}.rts0"))
}

fn write_once(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    if path.exists() {
        return Ok(());
    }
    crate::cache::write_atomic(path, bytes).map_err(|e| io(path, e))
}

fn encode_entry(e: &ManifestEntry) -> Vec<u8> {
    let mut w = Writer::default();
    w.u8(RECORD_VERSION).u128(e.key).u128(e.dxil_hash);
    for h in [e.root_signature, e.local_root_signature] {
        match h {
            Some(h) => w.u8(1).u128(h),
            None => w.u8(0),
        };
    }
    w.opt_str(e.entry_point.as_deref()).opt_u32(e.expected_stage.map(ShaderStage::to_raw));
    wire::put_options(&mut w, &e.options);
    w.buf
}

fn decode_entry(b: &[u8]) -> Result<ManifestEntry, Error> {
    let mut r = Reader::new(b);
    if r.u8()? != RECORD_VERSION {
        return Err(Error::Io("manifest record: unknown version".into()));
    }
    let key = r.u128()?;
    let dxil_hash = r.u128()?;
    let opt_hash = |r: &mut Reader| -> Result<Option<u128>, Error> { Ok(if r.bool()? { Some(r.u128()?) } else { None }) };
    let root_signature = opt_hash(&mut r)?;
    let local_root_signature = opt_hash(&mut r)?;
    Ok(ManifestEntry {
        key,
        dxil_hash,
        root_signature,
        local_root_signature,
        entry_point: r.opt_string()?,
        expected_stage: r.opt_u32()?.map(ShaderStage::from_raw),
        options: wire::get_options(&mut r)?,
    })
}

/// Parses `records.aw2m` bytes; stops at the first torn/corrupt record.
fn parse_records(b: &[u8]) -> (Vec<ManifestEntry>, bool) {
    let mut out = Vec::new();
    let mut pos = 0;
    while pos < b.len() {
        let Some(len) = b.get(pos..pos + 4).map(|l| u32::from_le_bytes(l.try_into().unwrap()) as usize) else { return (out, false) };
        let Some(payload) = b.get(pos + 4..pos + 4 + len) else { return (out, false) };
        let Some(sum) = b.get(pos + 4 + len..pos + 12 + len) else { return (out, false) };
        if xxh64(payload, 0).to_le_bytes() != sum {
            return (out, false);
        }
        match decode_entry(payload) {
            Ok(e) => out.push(e),
            Err(_) => return (out, false),
        }
        pos += 12 + len;
    }
    (out, true)
}

/// Appends compile requests to a manifest directory. Thread-safe; requests already recorded
/// (in this or an earlier run) are skipped after an in-memory lookup.
pub struct ManifestWriter {
    dir: PathBuf,
    file: Mutex<File>,
    seen: Mutex<HashSet<u128>>,
}

impl ManifestWriter {
    /// Opens (creating if needed) the manifest at `dir`.
    pub fn open(dir: impl Into<PathBuf>) -> Result<Self, Error> {
        let dir = dir.into();
        std::fs::create_dir_all(&dir).map_err(|e| io(&dir, e))?;
        let path = dir.join(RECORDS);
        let existing = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(e) => return Err(io(&path, e)),
        };
        let (entries, clean) = parse_records(&existing);
        if !clean {
            log::warn!(target: "shaders", "{}: ignoring torn/corrupt tail after {} records", path.display(), entries.len());
        }
        let file = OpenOptions::new().create(true).append(true).open(&path).map_err(|e| io(&path, e))?;
        Ok(Self { dir, file: Mutex::new(file), seen: Mutex::new(entries.iter().map(|e| e.key).collect()) })
    }

    /// `PORT_SHADER_MANIFEST` / `AW2_SHADER_MANIFEST` = directory, or `off`/`0`/empty to disable.
    /// Default: `<cache root>/manifest` when the disk cache is enabled (see
    /// [`CacheConfig::from_env`](crate::CacheConfig::from_env)).
    pub fn from_env() -> Option<Self> {
        let dir = match std::env::var_os("PORT_SHADER_MANIFEST")
            .or_else(|| std::env::var_os("AW2_SHADER_MANIFEST"))
        {
            Some(v) if v.is_empty() || v == "0" || v == "off" => return None,
            Some(v) => PathBuf::from(v),
            None => crate::CacheConfig::from_env().disk_dir?.join("manifest"),
        };
        match Self::open(&dir) {
            Ok(m) => Some(m),
            Err(e) => {
                log::warn!(target: "shaders", "shader manifest disabled: {e}");
                None
            }
        }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Number of distinct requests recorded (including earlier runs).
    pub fn len(&self) -> usize {
        self.seen.lock().unwrap().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Records `req`. Returns `Ok(true)` if it was new.
    pub fn record(&self, req: &CompileRequest) -> Result<bool, Error> {
        let key = req.cache_key("");
        if self.seen.lock().unwrap().contains(&key) {
            return Ok(false);
        }
        let dxil_hash = hash128(req.dxil);
        write_once(&dxil_path(&self.dir, dxil_hash), req.dxil)?;
        let store_rs = |rs: Option<&RootSignature>| -> Result<Option<u128>, Error> {
            let Some(rs) = rs else { return Ok(None) };
            write_once(&rs_path(&self.dir, rs.hash()), &rs.desc().to_rts0())?;
            Ok(Some(rs.hash()))
        };
        let entry = ManifestEntry {
            key,
            dxil_hash,
            root_signature: store_rs(req.root_signature)?,
            local_root_signature: store_rs(req.local_root_signature)?,
            entry_point: req.entry_point.map(str::to_string),
            expected_stage: req.expected_stage,
            options: req.options.clone(),
        };
        let payload = encode_entry(&entry);
        let mut rec = Vec::with_capacity(payload.len() + 12);
        rec.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        rec.extend_from_slice(&payload);
        rec.extend_from_slice(&xxh64(&payload, 0).to_le_bytes());
        let mut seen = self.seen.lock().unwrap();
        if !seen.insert(key) {
            return Ok(false);
        }
        // One write per record on an O_APPEND file, so concurrent writers don't interleave.
        self.file.lock().unwrap().write_all(&rec).map_err(|e| io(&self.dir.join(RECORDS), e))?;
        Ok(true)
    }
}

/// A loaded manifest (for replay).
pub struct Manifest {
    dir: PathBuf,
    pub entries: Vec<ManifestEntry>,
}

impl Manifest {
    /// Discovers existing manifest directories in priority order: writable user manifest
    /// (`PORT_SHADER_MANIFEST` / `AW2_SHADER_MANIFEST` / `<shader cache>/manifest`) plus any
    /// read-only bundled fallback (`PORT_SHADER_MANIFEST_FALLBACK` / `Contents/Resources/ShaderCache/manifest`).
    pub fn discover_dirs() -> Vec<PathBuf> {
        let mut out = Vec::new();
        let mut push_if_valid = |d: PathBuf| {
            if d.join(RECORDS).is_file() && !out.contains(&d) {
                out.push(d);
            }
        };
        match std::env::var_os("PORT_SHADER_MANIFEST")
            .or_else(|| std::env::var_os("AW2_SHADER_MANIFEST"))
        {
            Some(v) if v.is_empty() || v == "0" || v == "off" => {}
            Some(v) => push_if_valid(PathBuf::from(v)),
            None => {
                if let Some(d) = crate::CacheConfig::from_env().disk_dir {
                    push_if_valid(d.join("manifest"));
                }
            }
        }
        if let Some(v) = std::env::var_os("PORT_SHADER_MANIFEST_FALLBACK").filter(|v| !v.is_empty()) {
            push_if_valid(PathBuf::from(v));
        }
        if let Some(v) = std::env::var_os("PORT_SHADER_CACHE_FALLBACK").filter(|v| !v.is_empty()) {
            push_if_valid(PathBuf::from(v).join("manifest"));
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                push_if_valid(parent.join("ShaderCache/manifest"));
                push_if_valid(parent.join("../Resources/ShaderCache/manifest"));
            }
        }
        out
    }

    pub fn load(dir: impl Into<PathBuf>) -> Result<Self, Error> {
        let dir = dir.into();
        let path = dir.join(RECORDS);
        let b = std::fs::read(&path).map_err(|e| io(&path, e))?;
        let (entries, clean) = parse_records(&b);
        if !clean {
            log::warn!(target: "shaders", "{}: ignoring torn/corrupt tail after {} records", path.display(), entries.len());
        }
        Ok(Self { dir, entries })
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// The DXIL container of `e` (verified against its hash).
    pub fn dxil(&self, e: &ManifestEntry) -> Result<Vec<u8>, Error> {
        let p = dxil_path(&self.dir, e.dxil_hash);
        let b = std::fs::read(&p).map_err(|err| io(&p, err))?;
        if hash128(&b) != e.dxil_hash {
            return Err(Error::Io(format!("{}: content hash mismatch", p.display())));
        }
        Ok(b)
    }

    /// RTS0 bytes of a recorded root signature (feed to
    /// [`ShaderPipeline::root_signature`](crate::ShaderPipeline::root_signature)).
    pub fn root_signature_blob(&self, hash: u128) -> Result<Vec<u8>, Error> {
        let p = rs_path(&self.dir, hash);
        std::fs::read(&p).map_err(|err| io(&p, err))
    }
}
