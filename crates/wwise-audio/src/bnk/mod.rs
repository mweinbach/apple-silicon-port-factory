//! Wwise SoundBank (.bnk) parser for bank generator version 145 (Wwise 2022.1.x).
//!
//! Chunk layouts from `third_party/wwiser/wwiser/parser/wparser.py` (`CAkBankMgr::Process*`),
//! v145 branches only. Unknown chunks are kept raw and reported.

pub mod hirc;
pub mod patch;

use crate::reader::{ParseError, Reader, Result};
pub use hirc::{HircBody, HircObject};

pub const SUPPORTED_VERSION: u32 = 145;

#[derive(Debug, Clone, Default)]
pub struct BankHeader {
    pub version: u32,
    pub bank_id: u32,
    pub language_id: u32,
    /// bits 0-15 uAlignment, 16-31 bDeviceAllocated
    pub alt_values: u32,
    pub project_id: u32,
    /// AkBankTypeEnum: 0 User, 1 Event (auto-defined), 2 Bus, ...
    pub bank_type: u32,
    pub hash: [u8; 16],
}

#[derive(Debug, Clone)]
pub struct ChunkInfo {
    pub fourcc: [u8; 4],
    pub offset: usize,
    pub size: usize,
}

impl ChunkInfo {
    pub fn name(&self) -> String {
        String::from_utf8_lossy(&self.fourcc).into_owned()
    }
}

/// DIDX entry: media embedded in the DATA chunk.
#[derive(Debug, Clone, Copy)]
pub struct MediaEntry {
    pub id: u32,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct StateGroupDef {
    pub id: u32,
    pub default_transition_time: u32,
    /// (from, to, time ms)
    pub transitions: Vec<(u32, u32, u32)>,
}

#[derive(Debug, Clone)]
pub struct SwitchGroupDef {
    pub id: u32,
    pub rtpc_id: u32,
    pub rtpc_type: u8,
    pub points: Vec<hirc::GraphPoint>,
}

#[derive(Debug, Clone)]
pub struct RtpcRamping {
    pub rtpc_id: u32,
    pub default_value: f32,
    /// AkTransitionRampingType: 0 None, 1 SlewRate, 2 FilteringOverTime
    pub ramp_type: u32,
    pub ramp_up: f32,
    pub ramp_down: f32,
    /// AkBuiltInParam: 0 None, 1 Distance, 2 Azimuth, 3 Elevation, 4 EmitterCone, 5 Obsruction,
    /// 6 Occlusion, 7 ListenerCone, 8 Diffraction, ...
    pub bind_to_builtin: u8,
}

#[derive(Debug, Clone)]
pub struct AcousticTexture {
    pub id: u32,
    pub absorption_offset: f32,
    pub absorption: [f32; 4],
    pub scattering: f32,
}

/// STMG (Init.bnk only): global settings.
#[derive(Debug, Clone)]
pub struct GlobalSettings {
    pub filter_behavior: u16,
    pub volume_threshold: f32,
    pub max_num_voices: u16,
    pub max_num_dangerous_virt_voices: u16,
    pub state_groups: Vec<StateGroupDef>,
    pub switch_groups: Vec<SwitchGroupDef>,
    pub rtpc_ramping: Vec<RtpcRamping>,
    pub acoustic_textures: Vec<AcousticTexture>,
}

/// ENVS: obstruction/occlusion conversion curves [x: obstruction, occlusion][y: volume, LPF, HPF].
#[derive(Debug, Clone)]
pub struct EnvCurve {
    pub enabled: bool,
    pub scaling: u8,
    pub points: Vec<hirc::GraphPoint>,
}

#[derive(Debug, Clone, Default)]
pub struct Bank {
    pub header: BankHeader,
    pub chunks: Vec<ChunkInfo>,
    pub media_index: Vec<MediaEntry>,
    /// Absolute (offset, size) of the DATA chunk payload.
    pub data: Option<(usize, usize)>,
    pub hirc: Vec<HircObject>,
    /// HIRC objects that failed to parse: (type, id, offset, error).
    pub hirc_errors: Vec<(u8, u32, usize, String)>,
    /// STID: bank id -> file name.
    pub bank_names: Vec<(u32, String)>,
    /// INIT: plugins registered by the project (id, dll name).
    pub plugins: Vec<(u32, String)>,
    pub global_settings: Option<GlobalSettings>,
    pub env_curves: Vec<EnvCurve>,
    pub custom_platform: Option<String>,
}

impl Bank {
    pub fn parse(buf: &[u8]) -> Result<Bank> {
        let mut r = Reader::new(buf);
        let mut bank = Bank::default();
        while !r.is_empty() {
            let fourcc = r.fourcc()?;
            let size = r.u32()? as usize;
            let offset = r.pos();
            let mut c = r.sub(size)?;
            bank.chunks.push(ChunkInfo { fourcc, offset, size });
            match &fourcc {
                b"BKHD" => bank.header = parse_bkhd(&mut c)?,
                b"DIDX" => {
                    for _ in 0..size / 12 {
                        bank.media_index.push(MediaEntry { id: c.u32()?, offset: c.u32()?, size: c.u32()? });
                    }
                }
                b"DATA" => bank.data = Some((offset, size)),
                b"HIRC" => parse_hirc(&mut c, &mut bank)?,
                b"STID" => {
                    let _type = c.u32()?;
                    let n = c.u32()?;
                    for _ in 0..n {
                        let id = c.u32()?;
                        let len = c.u8()? as usize;
                        bank.bank_names.push((id, c.string(len)?));
                    }
                }
                b"INIT" => {
                    let n = c.u32()?;
                    for _ in 0..n {
                        let id = c.u32()?;
                        bank.plugins.push((id, c.stz()?));
                    }
                }
                b"STMG" => bank.global_settings = Some(parse_stmg(&mut c)?),
                b"ENVS" => {
                    for _ in 0..2 * 3 {
                        let enabled = c.u8()? != 0;
                        let scaling = c.u8()?;
                        let n = c.u16()? as usize;
                        let mut points = Vec::with_capacity(n);
                        for _ in 0..n {
                            points.push(hirc::GraphPoint { from: c.f32()?, to: c.f32()?, interp: c.u32()? });
                        }
                        bank.env_curves.push(EnvCurve { enabled, scaling, points });
                    }
                }
                b"PLAT" => bank.custom_platform = Some(c.stz()?),
                _ => {}
            }
            if bank.header.version != 0 && bank.header.version != SUPPORTED_VERSION {
                return Err(ParseError {
                    offset: 8,
                    msg: format!("bank version {} unsupported (only {})", bank.header.version, SUPPORTED_VERSION),
                });
            }
        }
        Ok(bank)
    }

    /// In-bank media bytes (a complete or prefetch-truncated .wem) for a DIDX id.
    pub fn media<'a>(&self, buf: &'a [u8], id: u32) -> Option<&'a [u8]> {
        let (data_off, data_size) = self.data?;
        let e = self.media_index.iter().find(|e| e.id == id)?;
        if e.offset as usize + e.size as usize > data_size {
            return None;
        }
        let s = data_off + e.offset as usize;
        buf.get(s..s + e.size as usize)
    }

    pub fn object(&self, id: u32) -> Option<&HircObject> {
        self.hirc.iter().find(|o| o.id == id)
    }
}

fn parse_bkhd(c: &mut Reader) -> Result<BankHeader> {
    let mut h = BankHeader { version: c.u32()?, ..Default::default() };
    if h.version != SUPPORTED_VERSION {
        return c.err(format!("bank version {} unsupported (only {})", h.version, SUPPORTED_VERSION));
    }
    h.bank_id = c.u32()?;
    h.language_id = c.u32()?;
    h.alt_values = c.u32()?;
    h.project_id = c.u32()?;
    h.bank_type = c.u32()?;
    h.hash.copy_from_slice(c.bytes(16)?);
    Ok(h) // remaining bytes are padding
}

fn parse_hirc(c: &mut Reader, bank: &mut Bank) -> Result<()> {
    let n = c.u32()?;
    for _ in 0..n {
        let hirc_type = c.u8()?;
        let size = c.u32()? as usize;
        let offset = c.abs_pos();
        let mut s = c.sub(size)?;
        let id_peek = s.peek_u32().unwrap_or(0);
        match hirc::parse_object(hirc_type, &mut s) {
            Ok((id, body)) if s.is_empty() => bank.hirc.push(HircObject { hirc_type, id, offset, size, body }),
            Ok((id, _)) => bank.hirc_errors.push((
                hirc_type,
                id,
                offset,
                format!("{} bytes left unparsed of {}", s.remaining(), size),
            )),
            Err(e) => bank.hirc_errors.push((hirc_type, id_peek, offset, e.to_string())),
        }
    }
    Ok(())
}

fn parse_stmg(c: &mut Reader) -> Result<GlobalSettings> {
    let filter_behavior = c.u16()?;
    let volume_threshold = c.f32()?;
    let max_num_voices = c.u16()?;
    let max_num_dangerous_virt_voices = c.u16()?;
    let n = c.u32()? as usize;
    let mut state_groups = Vec::with_capacity(n);
    for _ in 0..n {
        let id = c.u32()?;
        let default_transition_time = c.u32()?;
        let nt = c.u32()? as usize;
        let mut transitions = Vec::with_capacity(nt);
        for _ in 0..nt {
            transitions.push((c.u32()?, c.u32()?, c.u32()?));
        }
        state_groups.push(StateGroupDef { id, default_transition_time, transitions });
    }
    let n = c.u32()? as usize;
    let mut switch_groups = Vec::with_capacity(n);
    for _ in 0..n {
        let id = c.u32()?;
        let rtpc_id = c.u32()?;
        let rtpc_type = c.u8()?;
        let np = c.u32()? as usize;
        let mut points = Vec::with_capacity(np);
        for _ in 0..np {
            points.push(hirc::GraphPoint { from: c.f32()?, to: c.f32()?, interp: c.u32()? });
        }
        switch_groups.push(SwitchGroupDef { id, rtpc_id, rtpc_type, points });
    }
    let n = c.u32()? as usize;
    let mut rtpc_ramping = Vec::with_capacity(n);
    for _ in 0..n {
        rtpc_ramping.push(RtpcRamping {
            rtpc_id: c.u32()?,
            default_value: c.f32()?,
            ramp_type: c.u32()?,
            ramp_up: c.f32()?,
            ramp_down: c.f32()?,
            bind_to_builtin: c.u8()?,
        });
    }
    let n = c.u32()? as usize;
    let mut acoustic_textures = Vec::with_capacity(n);
    for _ in 0..n {
        acoustic_textures.push(AcousticTexture {
            id: c.u32()?,
            absorption_offset: c.f32()?,
            absorption: [c.f32()?, c.f32()?, c.f32()?, c.f32()?],
            scattering: c.f32()?,
        });
    }
    if !c.is_empty() {
        return c.err(format!("STMG: {} trailing bytes", c.remaining()));
    }
    Ok(GlobalSettings {
        filter_behavior,
        volume_threshold,
        max_num_voices,
        max_num_dangerous_virt_voices,
        state_groups,
        switch_groups,
        rtpc_ramping,
        acoustic_textures,
    })
}

/// Plugin id -> human name (wwiser `wdefs.AkPluginType_id`, Audiokinetic + partners).
pub fn plugin_name(id: u32) -> &'static str {
    match id {
        0x0000_0000 | 0xFFFF_FFFF => "None",
        0x0000_0001 => "BANK",
        0x0001_0001 => "PCM",
        0x0002_0001 => "ADPCM",
        0x0003_0001 => "XMA",
        0x0004_0001 => "VORBIS",
        0x0007_0001 => "PCMEX",
        0x0008_0001 => "EXTERNAL_SOURCE",
        0x000A_0001 => "AAC",
        0x0010_0001 => "MIDI",
        0x0011_0001 => "OPUSNX",
        0x0013_0001 => "OPUS",
        0x0014_0001 | 0x0015_0001 => "OPUS_WEM",
        0x0064_0002 => "Wwise Sine",
        0x0065_0002 => "Wwise Silence",
        0x0066_0002 => "Wwise Tone Generator",
        0x0069_0003 => "Wwise Parametric EQ",
        0x006A_0003 => "Wwise Delay",
        0x006C_0003 => "Wwise Compressor",
        0x006D_0003 => "Wwise Expander",
        0x006E_0003 => "Wwise Peak Limiter",
        0x0073_0003 => "Wwise Matrix Reverb",
        0x0074_0003 => "SoundSeed Impact",
        0x0076_0003 => "Wwise RoomVerb",
        0x0077_0002 => "SoundSeed Air Wind",
        0x0078_0002 => "SoundSeed Air Woosh",
        0x007D_0003 => "Wwise Flanger",
        0x007E_0003 => "Wwise Guitar Distortion",
        0x007F_0003 => "Wwise Convolution Reverb",
        0x0081_0003 => "Wwise Meter",
        0x0082_0003 => "Wwise Time Stretch",
        0x0083_0003 => "Wwise Tremolo",
        0x0084_0003 => "Wwise Recorder",
        0x0087_0003 => "Wwise Stereo Delay",
        0x0088_0003 => "Wwise Pitch Shifter",
        0x008A_0003 => "Wwise Harmonizer",
        0x008B_0003 => "Wwise Gain",
        0x0094_0002 => "Wwise Synth One",
        0x00AB_0003 => "Wwise Reflect",
        0x00AE_0007 => "System (audio device)",
        0x00B0_0007 => "Communication (audio device)",
        0x00B1_0007 => "Controller Headphones (audio device)",
        0x00B3_0007 => "Controller Speaker (audio device)",
        0x00B5_0007 => "No Output (audio device)",
        0x00B7_0002 => "SoundSeed Grain",
        0x00BA_0003 => "Mastering Suite",
        0x00BE_0003 => "Wwise 3D Audio Bed Mixer",
        0x00C8_0002 => "Wwise Audio Input",
        0x0199_0002 => "Wwise Motion Source",
        0x01FB_0007 => "Wwise Motion (audio device)",
        0x006E_1003 => "McDSP FutzBox",
        0x0067_1003 => "McDSP ML1",
        0x0003_1033 => "iZotope Trash Distortion",
        0x0002_1033 => "iZotope Hybrid Reverb",
        0x0000_0403 => "QAnalyzer (company 0x40, 'Plugin')",
        _ => "?",
    }
}
