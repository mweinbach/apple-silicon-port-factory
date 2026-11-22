//! Runtime object registry over several loaded banks (the A3 subset of AK's bank manager).

use crate::bnk::hirc::{HircBody, HircObject, Source};
use crate::bnk::{Bank, GlobalSettings, MediaEntry};
use crate::data::AudioDataSource;
use std::collections::HashMap;
use std::sync::Arc;

pub struct LoadedBank {
    pub id: u32,
    pub name: String,
    pub bytes: Vec<u8>,
    pub bank: Bank,
    pub active: bool,
    pub ref_count: u32,
}

/// AK keeps one index per object family (`CAkIndexItem` tables), so the same ShortID may name
/// e.g. a bus and an audio device (Init.bnk has such pairs) [V: data].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Family {
    Node,
    Bus,
    Action,
    Event,
    State,
    Attenuation,
    Fx,
    Modulator,
    Device,
    Dialogue,
}

pub fn family(hirc_type: u8) -> Family {
    match hirc_type {
        0x01 => Family::State,
        0x03 => Family::Action,
        0x04 => Family::Event,
        0x08 | 0x12 => Family::Bus,
        0x0e => Family::Attenuation,
        0x0f => Family::Dialogue,
        0x10 | 0x11 => Family::Fx,
        0x13 | 0x14 | 0x16 => Family::Modulator,
        0x15 => Family::Device,
        _ => Family::Node,
    }
}

/// Objects from all loaded banks. As in AK, an ID that is already registered keeps the first
/// loaded definition (later banks only add a reference) [A: matches AK's `CAkIndexable` reuse].
pub struct Project {
    pub banks: Vec<LoadedBank>,
    index: HashMap<(Family, u32), (usize, usize)>,
    media: HashMap<u32, (usize, MediaEntry)>,
    /// Standalone / synthetic media blobs keyed by media ID (e.g. plugin IR media registered
    /// programmatically or in unit tests without constructing a full `.bnk` DIDX chunk).
    extra_media: HashMap<u32, Vec<u8>>,
    pub game: Option<Arc<dyn AudioDataSource>>,
    /// Game parameter id -> default value (STMG `RTPCRamping` defaults from Init.bnk).
    pub param_defaults: HashMap<u32, f32>,
    pub volume_threshold_db: f32,
}

#[derive(Debug, Clone)]
pub enum MediaSource {
    Bank { bank: usize, entry: MediaEntry },
    Stream(String),
}

impl Project {
    pub fn new(game: Option<Arc<dyn AudioDataSource>>) -> Self {
        Project {
            banks: vec![],
            index: HashMap::new(),
            media: HashMap::new(),
            extra_media: HashMap::new(),
            game,
            param_defaults: HashMap::new(),
            volume_threshold_db: -96.3,
        }
    }

    /// Convenience constructor accepting any concrete [`AudioDataSource`].
    pub fn with_source<S: AudioDataSource + 'static>(source: Option<Arc<S>>) -> Self {
        Self::new(source.map(|s| s as Arc<dyn AudioDataSource>))
    }

    /// Load a bank by VFS path (`data_pc/audio/events/<name>`), short name (`Global.bnk` or
    /// `Global`), or a file on disk.
    pub fn load_bank(&mut self, spec: &str) -> Result<usize, String> {
        let (name, bytes) = self.read_bank(spec)?;
        self.load_bank_bytes(name, bytes)
    }

    /// Load a bank by name/path and return its Wwise `bank_id` (`BKHD.dwSoundBankID`).
    pub fn load_bank_with_id(&mut self, spec: &str) -> Result<u32, String> {
        let bi = self.load_bank(spec)?;
        Ok(self.banks[bi].id)
    }

    /// Raw bytes of a bank spec (file path, VFS path or short name) without loading it.
    pub fn read_bank(&self, spec: &str) -> Result<(String, Vec<u8>), String> {
        Ok(if std::path::Path::new(spec).is_file() {
            (spec.to_string(), std::fs::read(spec).map_err(|e| format!("{}: {}", spec, e))?)
        } else {
            let ga = self.game.as_ref().ok_or("game data not available")?;
            let v = ga.bank_vpath(spec);
            let b = ga.read(&v)?;
            (v, b)
        })
    }

    pub fn load_bank_bytes(&mut self, name: String, bytes: Vec<u8>) -> Result<usize, String> {
        if bytes.len() < 16 {
            return Err(format!("{}: placeholder/empty bank ({} bytes)", name, bytes.len()));
        }
        let bank = Bank::parse(&bytes).map_err(|e| format!("{}: {}", name, e))?;
        let bank_id = if bank.header.bank_id != 0 {
            bank.header.bank_id
        } else {
            crate::fnv1_32_lower(&name)
        };
        if bank_id != 0 {
            if let Some((bi, existing)) = self
                .banks
                .iter_mut()
                .enumerate()
                .find(|(_, b)| b.active && b.id == bank_id)
            {
                existing.ref_count = existing.ref_count.saturating_add(1);
                return Ok(bi);
            }
        }
        let bi = self.banks.len();
        for (oi, o) in bank.hirc.iter().enumerate() {
            self.index.entry((family(o.hirc_type), o.id)).or_insert((bi, oi));
        }
        for e in &bank.media_index {
            self.media.entry(e.id).or_insert((bi, *e));
        }
        if let Some(gs) = &bank.global_settings {
            self.apply_global_settings(gs);
        }
        self.banks.push(LoadedBank {
            id: bank_id,
            name,
            bytes,
            bank,
            active: true,
            ref_count: 1,
        });
        Ok(bi)
    }

    /// Unload a previously loaded bank by its `bank_id` (decrements its reference count; when 0,
    /// removes its HIRC and DIDX entries and exposes any shadowed definitions from still-loaded banks).
    pub fn unload_bank(&mut self, bank_id: u32) -> bool {
        let Some(bi) = self.banks.iter().position(|b| b.active && b.id == bank_id) else {
            return false;
        };
        let lb = &mut self.banks[bi];
        if lb.ref_count > 1 {
            lb.ref_count -= 1;
            return true;
        }
        lb.ref_count = 0;
        lb.active = false;
        lb.bytes.clear();
        lb.bank.hirc.clear();
        lb.bank.media_index.clear();
        self.rebuild_index();
        true
    }

    fn rebuild_index(&mut self) {
        self.index.clear();
        self.media.clear();
        for (bi, lb) in self.banks.iter().enumerate() {
            if !lb.active {
                continue;
            }
            for (oi, o) in lb.bank.hirc.iter().enumerate() {
                self.index.entry((family(o.hirc_type), o.id)).or_insert((bi, oi));
            }
            for e in &lb.bank.media_index {
                self.media.entry(e.id).or_insert((bi, *e));
            }
        }
    }

    /// Insert a single HIRC object directly into the project registry.
    pub fn insert_hirc(&mut self, obj: HircObject) {
        if self.banks.is_empty() {
            self.banks.push(LoadedBank {
                id: 0,
                name: "synthetic".into(),
                bytes: vec![],
                bank: Bank::default(),
                active: true,
                ref_count: 1,
            });
        }
        let bi = 0;
        self.banks[bi].active = true;
        let oi = self.banks[bi].bank.hirc.len();
        self.index.insert((family(obj.hirc_type), obj.id), (bi, oi));
        self.banks[bi].bank.hirc.push(obj);
    }

    fn apply_global_settings(&mut self, gs: &GlobalSettings) {
        self.volume_threshold_db = gs.volume_threshold;
        for r in &gs.rtpc_ramping {
            self.param_defaults.insert(r.rtpc_id, r.default_value);
        }
    }

    pub fn get(&self, fam: Family, id: u32) -> Option<&HircObject> {
        let (b, o) = *self.index.get(&(fam, id))?;
        Some(&self.banks[b].bank.hirc[o])
    }

    pub fn node(&self, id: u32) -> Option<&HircObject> {
        self.get(Family::Node, id)
    }
    pub fn bus(&self, id: u32) -> Option<&HircObject> {
        self.get(Family::Bus, id)
    }
    pub fn action(&self, id: u32) -> Option<&HircObject> {
        self.get(Family::Action, id)
    }
    pub fn event(&self, id: u32) -> Option<&HircObject> {
        self.get(Family::Event, id)
    }
    pub fn fx(&self, id: u32) -> Option<&HircObject> {
        self.get(Family::Fx, id)
    }
    pub fn attenuation(&self, id: u32) -> Option<&HircObject> {
        self.get(Family::Attenuation, id)
    }
    pub fn state(&self, id: u32) -> Option<&HircObject> {
        self.get(Family::State, id)
    }
    pub fn env_curves(&self) -> &[crate::bnk::EnvCurve] {
        self.banks.iter().find(|b| !b.bank.env_curves.is_empty()).map_or(&[], |b| &b.bank.env_curves)
    }

    /// Where the complete media of a source lives: in-memory sources from a loaded bank's DIDX,
    /// prefetch/streamed sources from `data_pc/audio/media/<id>.wem` (the bank copy of a prefetch
    /// source is only a prefix, verified in `wwise-audio validate`).
    pub fn locate_media(&self, src: &Source) -> Option<MediaSource> {
        if src.stream_type == 0 {
            if let Some((b, e)) = self.media.get(&src.source_id) {
                return Some(MediaSource::Bank { bank: *b, entry: *e });
            }
        }
        if let Some(ga) = self.game.as_ref() {
            return Some(MediaSource::Stream(ga.media_vpath(src.source_id)));
        }
        let (b, e) = self.media.get(&src.source_id)?;
        Some(MediaSource::Bank { bank: *b, entry: *e })
    }

    pub fn read_media(&self, m: &MediaSource) -> Result<Vec<u8>, String> {
        match m {
            MediaSource::Bank { bank, entry } => {
                let lb = &self.banks[*bank];
                lb.bank.media(&lb.bytes, entry.id).map(|s| s.to_vec()).ok_or_else(|| format!("media {} out of range", entry.id))
            }
            MediaSource::Stream(p) => self.game.as_ref().ok_or("no game data")?.read(p),
        }
    }

    /// Register raw binary media bytes directly by `media_id` (used for plugin media assets
    /// such as Convolution Reverb IRs in synthetic tests or loose-media loaders).
    pub fn insert_media(&mut self, media_id: u32, bytes: Vec<u8>) {
        self.extra_media.insert(media_id, bytes);
    }

    /// Read raw embedded (DIDX), registered (`extra_media`), or loose streamed media bytes by
    /// media ID. Used by FX plugins with binary media assets (e.g. Wwise Convolution Reverb IRs).
    pub fn read_media_by_id(&self, media_id: u32) -> Option<Vec<u8>> {
        if let Some((bank, entry)) = self.media.get(&media_id) {
            let lb = &self.banks[*bank];
            if let Some(bytes) = lb.bank.media(&lb.bytes, entry.id) {
                return Some(bytes.to_vec());
            }
        }
        if let Some(bytes) = self.extra_media.get(&media_id) {
            return Some(bytes.clone());
        }
        if let Some(ga) = self.game.as_ref() {
            let vpath = ga.media_vpath(media_id);
            if let Ok(bytes) = ga.read(&vpath) {
                return Some(bytes);
            }
        }
        None
    }

    pub fn events(&self) -> Vec<u32> {
        let mut v: Vec<u32> = self
            .banks
            .iter()
            .flat_map(|b| b.bank.hirc.iter())
            .filter(|o| matches!(o.body, HircBody::Event(_)))
            .map(|o| o.id)
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    }

    pub fn param_default(&self, id: u32) -> f32 {
        self.param_defaults.get(&id).copied().unwrap_or(0.0)
    }
}
