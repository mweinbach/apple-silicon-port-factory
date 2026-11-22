//! Game-agnostic audio data source abstraction (plus optional Northlight VFS adapter).

use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Pluggable audio asset provider for streamed `.wem` media, SoundBanks (`.bnk`),
/// and external sources. Implement this trait per engine/archive format so the Wwise
/// runtime never depends directly on a specific game's VFS.
pub trait AudioDataSource: Send + Sync {
    /// Read a virtual or relative audio asset path.
    fn read(&self, vpath: &str) -> Result<Vec<u8>, String>;

    /// Resolve a bank spec (short name or relative path) to a virtual path.
    fn bank_vpath(&self, spec: &str) -> String {
        let mut v = spec.to_string();
        if !v.ends_with(".bnk") {
            v.push_str(".bnk");
        }
        v
    }

    /// Resolve a streamed Wwise `source_id` to a virtual path.
    fn media_vpath(&self, source_id: u32) -> String {
        format!("{source_id}.wem")
    }

    /// Candidate virtual paths for an external source name (already slash-normalized with `.wem`).
    fn external_candidates(&self, norm: &str) -> Vec<String> {
        vec![norm.to_string()]
    }
}

/// Simple filesystem-backed [`AudioDataSource`] rooted at a directory containing
/// `.bnk` and `.wem` files (works for any unpacked/loose-file Wwise game).
pub struct DirAudioSource {
    root: PathBuf,
}

impl DirAudioSource {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

impl AudioDataSource for DirAudioSource {
    fn read(&self, vpath: &str) -> Result<Vec<u8>, String> {
        let p = self.root.join(vpath);
        std::fs::read(&p).map_err(|e| format!("{}: {e}", p.display()))
    }
}

/// Default audio source when none is explicitly provided.
/// Under the `northlight` feature, auto-discovers the Northlight VFS if installed.
pub fn default_source() -> Option<Arc<dyn AudioDataSource>> {
    #[cfg(feature = "northlight")]
    {
        GameAudio::open().ok().map(|g| Arc::new(g) as Arc<dyn AudioDataSource>)
    }
    #[cfg(not(feature = "northlight"))]
    {
        None
    }
}

// ---------------------------------------------------------------------------
// Optional Remedy Northlight VFS adapter (`feature = "northlight"`)
// ---------------------------------------------------------------------------

pub const AUDIO_ROOT: &str = "data_pc/audio/";
pub const EVENTS_DIR: &str = "data_pc/audio/events/";
pub const MEDIA_DIR: &str = "data_pc/audio/media/";

#[derive(Debug, Clone)]
pub struct AudioFile {
    pub path: String,
    pub size: u32,
}

#[cfg(feature = "northlight")]
pub fn game_dir() -> Option<PathBuf> {
    std::env::var("AW2_GAME_DIR")
        .ok()
        .map(PathBuf::from)
        .filter(|p| p.is_dir())
        .or_else(aw2_data::discover_game_dir)
}

#[cfg(feature = "northlight")]
pub struct GameAudio {
    pub vfs: aw2_data::NorthlightVfs,
}

#[cfg(feature = "northlight")]
impl GameAudio {
    pub fn open() -> Result<Self, String> {
        let dir = game_dir().ok_or("Alan Wake 2 install not found (set AW2_GAME_DIR)")?;
        Ok(Self { vfs: aw2_data::NorthlightVfs::mount(&dir)? })
    }

    pub fn list(&self, prefix: &str) -> Vec<AudioFile> {
        let mut v: Vec<AudioFile> = self
            .vfs
            .archives
            .iter()
            .flat_map(|a| a.files.iter())
            .filter(|e| e.path.starts_with(prefix))
            .map(|e| AudioFile { path: e.path.clone(), size: e.file_size })
            .collect();
        v.sort_by(|a, b| a.path.cmp(&b.path));
        v.dedup_by(|a, b| a.path == b.path);
        v
    }

    pub fn banks(&self) -> Vec<AudioFile> {
        self.list(EVENTS_DIR).into_iter().filter(|f| f.path.ends_with(".bnk")).collect()
    }

    pub fn wems(&self) -> Vec<AudioFile> {
        self.list(AUDIO_ROOT).into_iter().filter(|f| f.path.ends_with(".wem")).collect()
    }

    pub fn read(&self, vpath: &str) -> Result<Vec<u8>, String> {
        self.vfs.read_virtual_file(vpath)
    }

    /// Streamed media lives at `data_pc/audio/media/<sourceID>.wem`.
    pub fn media_path(source_id: u32) -> String {
        format!("{}{}.wem", MEDIA_DIR, source_id)
    }
}

#[cfg(feature = "northlight")]
impl AudioDataSource for GameAudio {
    fn read(&self, vpath: &str) -> Result<Vec<u8>, String> {
        GameAudio::read(self, vpath)
    }

    fn bank_vpath(&self, spec: &str) -> String {
        let mut v = spec.to_string();
        if !v.ends_with(".bnk") {
            v.push_str(".bnk");
        }
        if !v.contains('/') {
            v = format!("{}{}", EVENTS_DIR, v);
        }
        v
    }

    fn media_vpath(&self, source_id: u32) -> String {
        GameAudio::media_path(source_id)
    }

    fn external_candidates(&self, norm: &str) -> Vec<String> {
        let mut candidates = Vec::with_capacity(4);
        if let Some(idx) = norm.find(AUDIO_ROOT) {
            candidates.push(norm[idx..].to_string());
        }
        if norm.starts_with("data_pc/") {
            candidates.push(norm.to_string());
        } else if norm.starts_with("audio/") {
            candidates.push(format!("data_pc/{norm}"));
        } else {
            candidates.push(format!("{AUDIO_ROOT}{norm}"));
            candidates.push(format!("data_pc/audio/dialogue/script/locale/en/{norm}"));
        }
        candidates
    }
}
