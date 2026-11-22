//! Typed HIRC objects for SoundBank generator version 145 (Wwise 2022.1).
//!
//! The layout follows `third_party/wwiser/wwiser/parser/wparser.py` with every version branch
//! resolved for v145 (function names below are AK's, as used by wwiser). Each object must consume
//! exactly its `dwSectionSize`; `wwise-audio inventory` enforces that over all 5,009 game banks.

use crate::reader::{Reader, Result};

// ---------------------------------------------------------------------------------------------
// Common pieces

/// AkPropBundle<AkPropValue,unsigned char>: property id -> raw 32-bit union (f32 or int/ID).
#[derive(Debug, Clone, Default)]
pub struct Props(pub Vec<(u8, u32)>);

impl Props {
    pub fn get_raw(&self, id: u8) -> Option<u32> {
        self.0.iter().find(|(i, _)| *i == id).map(|(_, v)| *v)
    }
    pub fn get_f32(&self, id: u8) -> Option<f32> {
        self.get_raw(id).map(f32::from_bits)
    }
}

/// AkPropBundle<RANGED_MODIFIERS<AkPropValue>>: property id -> (min, max) raw unions.
#[derive(Debug, Clone, Default)]
pub struct RangedProps(pub Vec<(u8, u32, u32)>);

/// AkPropID for bank v128..v145 (wwiser `wdefs.AkPropID_128`).
pub mod prop {
    pub const VOLUME: u8 = 0x00;
    pub const LFE: u8 = 0x01;
    pub const PITCH: u8 = 0x02;
    pub const LPF: u8 = 0x03;
    pub const HPF: u8 = 0x04;
    pub const BUS_VOLUME: u8 = 0x05;
    pub const MAKE_UP_GAIN: u8 = 0x06;
    pub const PRIORITY: u8 = 0x07;
    pub const PRIORITY_DISTANCE_OFFSET: u8 = 0x08;
    pub const MUTE_RATIO: u8 = 0x0B;
    pub const PAN_LR: u8 = 0x0C;
    pub const PAN_FR: u8 = 0x0D;
    pub const CENTER_PCT: u8 = 0x0E;
    pub const DELAY_TIME: u8 = 0x0F;
    pub const TRANSITION_TIME: u8 = 0x10;
    pub const PROBABILITY: u8 = 0x11;
    pub const DIALOGUE_MODE: u8 = 0x12;
    pub const USER_AUX_SEND_VOLUME0: u8 = 0x13;
    pub const GAME_AUX_SEND_VOLUME: u8 = 0x17;
    pub const OUTPUT_BUS_VOLUME: u8 = 0x18;
    pub const OUTPUT_BUS_HPF: u8 = 0x19;
    pub const OUTPUT_BUS_LPF: u8 = 0x1A;
    pub const HDR_BUS_THRESHOLD: u8 = 0x1B;
    pub const HDR_BUS_RATIO: u8 = 0x1C;
    pub const HDR_BUS_RELEASE_TIME: u8 = 0x1D;
    pub const HDR_ACTIVE_RANGE: u8 = 0x21;
    pub const LOOP_START: u8 = 0x22;
    pub const LOOP_END: u8 = 0x23;
    pub const TRIM_IN_TIME: u8 = 0x24;
    pub const TRIM_OUT_TIME: u8 = 0x25;
    pub const FADE_IN_TIME: u8 = 0x26;
    pub const FADE_OUT_TIME: u8 = 0x27;
    pub const FADE_IN_CURVE: u8 = 0x28;
    pub const FADE_OUT_CURVE: u8 = 0x29;
    pub const LOOP_CROSSFADE_DURATION: u8 = 0x2A;
    pub const PLAYBACK_SPEED: u8 = 0x36;
    pub const ATTACHED_PLUGIN_FX_ID: u8 = 0x39;
    pub const LOOP: u8 = 0x3A;
    pub const INITIAL_DELAY: u8 = 0x3B;
    pub const ATTENUATION_ID: u8 = 0x46;
    pub const POSITIONING_TYPE_BLEND: u8 = 0x47;
    pub const REFLECTION_BUS_VOLUME: u8 = 0x48;
    pub const PAN_UD: u8 = 0x49;

    pub fn name(id: u8) -> &'static str {
        const N: [&str; 0x4A] = [
            "Volume", "LFE", "Pitch", "LPF", "HPF", "BusVolume", "MakeUpGain", "Priority",
            "PriorityDistanceOffset", "_FeedbackVolume", "_FeedbackLPF", "MuteRatio", "PAN_LR", "PAN_FR",
            "CenterPCT", "DelayTime", "TransitionTime", "Probability", "DialogueMode",
            "UserAuxSendVolume0", "UserAuxSendVolume1", "UserAuxSendVolume2", "UserAuxSendVolume3",
            "GameAuxSendVolume", "OutputBusVolume", "OutputBusHPF", "OutputBusLPF", "HDRBusThreshold",
            "HDRBusRatio", "HDRBusReleaseTime", "HDRBusGameParam", "HDRBusGameParamMin",
            "HDRBusGameParamMax", "HDRActiveRange", "LoopStart", "LoopEnd", "TrimInTime", "TrimOutTime",
            "FadeInTime", "FadeOutTime", "FadeInCurve", "FadeOutCurve", "LoopCrossfadeDuration",
            "CrossfadeUpCurve", "CrossfadeDownCurve", "MidiTrackingRootNote", "MidiPlayOnNoteType",
            "MidiTransposition", "MidiVelocityOffset", "MidiKeyRangeMin", "MidiKeyRangeMax",
            "MidiVelocityRangeMin", "MidiVelocityRangeMax", "MidiChannelMask", "PlaybackSpeed",
            "MidiTempoSource", "MidiTargetNode", "AttachedPluginFXID", "Loop", "InitialDelay",
            "UserAuxSendLPF0", "UserAuxSendLPF1", "UserAuxSendLPF2", "UserAuxSendLPF3",
            "UserAuxSendHPF0", "UserAuxSendHPF1", "UserAuxSendHPF2", "UserAuxSendHPF3", "GameAuxSendLPF",
            "GameAuxSendHPF", "AttenuationID", "PositioningTypeBlend", "ReflectionBusVolume", "PAN_UD",
        ];
        N.get(id as usize).copied().unwrap_or("?")
    }

    /// Properties whose union holds an object ID rather than a float (wwiser `AkPropID_tids`).
    /// Other integer-valued props (e.g. Loop) are not yet classified - verify against
    /// `AkPropValue` users in the exe before relying on a type.
    pub fn is_id(id: u8) -> bool {
        matches!(id, ATTACHED_PLUGIN_FX_ID | ATTENUATION_ID | 0x38)
    }
}

fn read_props(r: &mut Reader) -> Result<Props> {
    let n = r.u8()? as usize;
    let ids = r.bytes(n)?.to_vec();
    let mut v = Vec::with_capacity(n);
    for id in ids {
        v.push((id, r.u32()?));
    }
    Ok(Props(v))
}

fn read_ranged_props(r: &mut Reader) -> Result<RangedProps> {
    let n = r.u8()? as usize;
    let ids = r.bytes(n)?.to_vec();
    let mut v = Vec::with_capacity(n);
    for id in ids {
        v.push((id, r.u32()?, r.u32()?));
    }
    Ok(RangedProps(v))
}

#[derive(Debug, Clone, Copy)]
pub struct GraphPoint {
    pub from: f32,
    pub to: f32,
    /// AkCurveInterpolation (0 Log3, 1 Sine, 2 Log1, 3 InvSCurve, 4 Linear, 5 SCurve, 6 Exp1,
    /// 7 SineRecip, 8 Exp3, 9 Constant).
    pub interp: u32,
}

fn read_graph(r: &mut Reader, n: usize) -> Result<Vec<GraphPoint>> {
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(GraphPoint { from: r.f32()?, to: r.f32()?, interp: r.u32()? });
    }
    Ok(v)
}

/// One RTPC curve (`AK::RTPC::ReadRtpcCurves`).
#[derive(Debug, Clone)]
pub struct Rtpc {
    pub rtpc_id: u32,
    /// AkRtpcType: 0 GameParameter, 1 MIDIParameter, 2 Modulator.
    pub rtpc_type: u8,
    /// AkRtpcAccum: 0 None, 1 Exclusive, 2 Additive, 3 Multiply, 4 Boolean, 5 Maximum, 6 Filter.
    pub accum: u8,
    /// AkRTPC_ParameterID (target property).
    pub param_id: u32,
    pub curve_id: u32,
    /// AkCurveScaling: 0 None, 2 dB, 3 Log, 4 dBToLin.
    pub scaling: u8,
    pub points: Vec<GraphPoint>,
}

fn read_rtpcs(r: &mut Reader) -> Result<Vec<Rtpc>> {
    let n = r.u16()? as usize;
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        let rtpc_id = r.u32()?;
        let rtpc_type = r.u8()?;
        let accum = r.u8()?;
        let param_id = r.var()?;
        let curve_id = r.u32()?;
        let scaling = r.u8()?;
        let size = r.u16()? as usize;
        let points = read_graph(r, size)?;
        v.push(Rtpc { rtpc_id, rtpc_type, accum, param_id, curve_id, scaling, points });
    }
    Ok(v)
}

#[derive(Debug, Clone)]
pub struct StateProp {
    pub property_id: u32,
    pub accum: u8,
    pub in_db: u8,
}

#[derive(Debug, Clone)]
pub struct StateGroupChunk {
    pub group_id: u32,
    /// AkSyncType.
    pub sync_type: u8,
    /// (state id, CAkState object id)
    pub states: Vec<(u32, u32)>,
}

#[derive(Debug, Clone, Default)]
pub struct StateChunk {
    pub props: Vec<StateProp>,
    pub groups: Vec<StateGroupChunk>,
}

/// `CAkStateAware::ReadStateChunk` (v127..v145 layout).
fn read_state_chunk(r: &mut Reader) -> Result<StateChunk> {
    let np = r.var()? as usize;
    let mut props = Vec::with_capacity(np);
    for _ in 0..np {
        props.push(StateProp { property_id: r.var()?, accum: r.u8()?, in_db: r.u8()? });
    }
    let ng = r.var()? as usize;
    let mut groups = Vec::with_capacity(ng);
    for _ in 0..ng {
        let group_id = r.u32()?;
        let sync_type = r.u8()?;
        let ns = r.var()? as usize;
        let mut states = Vec::with_capacity(ns);
        for _ in 0..ns {
            states.push((r.u32()?, r.u32()?));
        }
        groups.push(StateGroupChunk { group_id, sync_type, states });
    }
    Ok(StateChunk { props, groups })
}

#[derive(Debug, Clone)]
pub struct FxSlot {
    pub index: u8,
    pub fx_id: u32,
    pub is_shareset: bool,
    pub is_rendered: bool,
}

#[derive(Debug, Clone, Default)]
pub struct FxChain {
    pub override_parent: bool,
    pub bypass_bits: u8,
    /// Absolute byte offset of `bypass_bits` in the bank (present only when there are slots).
    pub bypass_pos: Option<usize>,
    pub slots: Vec<FxSlot>,
}

fn read_fx_slots(r: &mut Reader, with_override: bool) -> Result<FxChain> {
    let override_parent = if with_override { r.u8()? != 0 } else { false };
    let n = r.u8()? as usize;
    let mut c = FxChain { override_parent, ..Default::default() };
    if n > 0 {
        c.bypass_pos = Some(r.abs_pos());
        c.bypass_bits = r.u8()?;
        for _ in 0..n {
            c.slots.push(FxSlot { index: r.u8()?, fx_id: r.u32()?, is_shareset: r.u8()? != 0, is_rendered: r.u8()? != 0 });
        }
    }
    Ok(c)
}

#[derive(Debug, Clone, Default)]
pub struct MetadataFx {
    pub override_parent: bool,
    /// (slot index, fx id, is shareset)
    pub slots: Vec<(u8, u32, bool)>,
}

#[derive(Debug, Clone)]
pub struct PathVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub duration_ms: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Automation {
    pub path_mode: u8,
    pub transition_time: i32,
    pub vertices: Vec<PathVertex>,
    /// (vertices offset, vertex count)
    pub playlist: Vec<(u32, u32)>,
    /// (x, y, z) random ranges per playlist item
    pub ranges: Vec<(f32, f32, f32)>,
}

/// `CAkParameterNodeBase::SetPositioningParams` (v130+).
#[derive(Debug, Clone, Default)]
pub struct Positioning {
    pub bits: u8,
    pub bits_3d: u8,
    pub automation: Option<Automation>,
    /// Absolute byte range of the positioning block in the bank file (for bank patching).
    pub span: (usize, usize),
}

impl Positioning {
    pub fn override_parent(&self) -> bool {
        self.bits & 1 != 0
    }
    pub fn has_listener_relative_routing(&self) -> bool {
        self.bits & 2 != 0
    }
    /// AkSpeakerPanningType (0 DirectSpeakerAssignment, 1 BalanceFadeHeight, 2 SteeringPanner).
    pub fn panner_type(&self) -> u8 {
        (self.bits >> 2) & 3
    }
    /// Ak3DPositionType (0 Emitter, 1 EmitterWithAutomation, 2 ListenerWithAutomation).
    pub fn position_type_3d(&self) -> u8 {
        (self.bits >> 5) & 3
    }
    /// Ak3DSpatializationMode (0 None, 1 PositionOnly, 2 PositionAndOrientation).
    pub fn spatialization_mode(&self) -> u8 {
        self.bits_3d & 3
    }
    pub fn enable_attenuation(&self) -> bool {
        self.bits_3d & 0x08 != 0
    }
    pub fn enable_diffraction(&self) -> bool {
        self.bits_3d & 0x80 != 0
    }
}

fn read_positioning(r: &mut Reader) -> Result<Positioning> {
    let start = r.abs_pos();
    let bits = r.u8()?;
    let mut p = Positioning { bits, ..Default::default() };
    let has_positioning = bits & 1 != 0;
    let has_3d = has_positioning && (bits >> 1) & 1 != 0;
    if has_positioning && has_3d {
        p.bits_3d = r.u8()?;
        let e3d = (bits >> 5) & 3;
        if e3d != 0 {
            let mut a = Automation { path_mode: r.u8()?, transition_time: r.i32()?, ..Default::default() };
            let nv = r.u32()? as usize;
            for _ in 0..nv {
                a.vertices.push(PathVertex { x: r.f32()?, y: r.f32()?, z: r.f32()?, duration_ms: r.i32()? });
            }
            let np = r.u32()? as usize;
            for _ in 0..np {
                a.playlist.push((r.u32()?, r.u32()?));
            }
            for _ in 0..np {
                a.ranges.push((r.f32()?, r.f32()?, r.f32()?));
            }
            p.automation = Some(a);
        }
    }
    p.span = (start, r.abs_pos());
    Ok(p)
}

#[derive(Debug, Clone, Default)]
pub struct AuxParams {
    pub bits: u8,
    pub aux_ids: Option<[u32; 4]>,
    pub reflections_aux_bus: u32,
    /// Absolute byte range of the aux block in the bank file (for bank patching).
    pub span: (usize, usize),
}

/// `CAkParameterNodeBase::SetAuxParams` (v136+).
fn read_aux(r: &mut Reader) -> Result<AuxParams> {
    let start = r.abs_pos();
    let bits = r.u8()?;
    let aux_ids = if (bits >> 3) & 1 != 0 { Some([r.u32()?, r.u32()?, r.u32()?, r.u32()?]) } else { None };
    let reflections_aux_bus = r.u32()?;
    Ok(AuxParams { bits, aux_ids, reflections_aux_bus, span: (start, r.abs_pos()) })
}

#[derive(Debug, Clone, Default)]
pub struct AdvSettings {
    /// bit0 bKillNewest, bit1 bUseVirtualBehavior, bit3 bIgnoreParentMaxNumInst,
    /// bit4 bIsVVoicesOptOverrideParent
    pub bits: u8,
    /// AkVirtualQueueBehavior (0 FromBeginning, 1 FromElapsedTime, 2 Resume)
    pub virtual_queue_behavior: u8,
    pub max_num_instance: u16,
    /// AkBelowThresholdBehavior (0 ContinueToPlay, 1 KillVoice, 2 SetAsVirtualVoice, 3 KillIfOneShotElseVirtual)
    pub below_threshold_behavior: u8,
    /// bit0 bOverrideHdrEnvelope, bit1 bOverrideAnalysis, bit2 bNormalizeLoudness, bit3 bEnableEnvelope
    pub hdr_bits: u8,
}

/// `CAkParameterNodeBase::SetNodeBaseParams` for CAkParameterNode-derived objects.
#[derive(Debug, Clone, Default)]
pub struct NodeBase {
    pub fx: FxChain,
    pub metadata: MetadataFx,
    pub override_attachment_params: u8,
    pub override_bus_id: u32,
    pub direct_parent_id: u32,
    /// bit0 bPriorityOverrideParent, bit1 bPriorityApplyDistFactor, bits2-5 MIDI flags
    pub priority_bits: u8,
    pub props: Props,
    pub ranged: RangedProps,
    pub positioning: Positioning,
    pub aux: AuxParams,
    pub adv: AdvSettings,
    pub states: StateChunk,
    pub rtpcs: Vec<Rtpc>,
}

fn read_node_base(r: &mut Reader) -> Result<NodeBase> {
    let fx = read_fx_slots(r, true)?; // CAkParameterNode::SetInitialFxParams
    // CAkParameterNode::SetInitialMetadataParams (v137+)
    let mut metadata = MetadataFx { override_parent: r.u8()? != 0, slots: vec![] };
    let n = r.u8()?;
    for _ in 0..n {
        metadata.slots.push((r.u8()?, r.u32()?, r.u8()? != 0));
    }
    let override_attachment_params = r.u8()?;
    let override_bus_id = r.u32()?;
    let direct_parent_id = r.u32()?;
    let priority_bits = r.u8()?;
    let props = read_props(r)?; // CAkParameterNode::SetInitialParams
    let ranged = read_ranged_props(r)?;
    let positioning = read_positioning(r)?;
    let aux = read_aux(r)?;
    let adv = AdvSettings {
        bits: r.u8()?,
        virtual_queue_behavior: r.u8()?,
        max_num_instance: r.u16()?,
        below_threshold_behavior: r.u8()?,
        hdr_bits: r.u8()?,
    };
    let states = read_state_chunk(r)?;
    let rtpcs = read_rtpcs(r)?;
    Ok(NodeBase {
        fx,
        metadata,
        override_attachment_params,
        override_bus_id,
        direct_parent_id,
        priority_bits,
        props,
        ranged,
        positioning,
        aux,
        adv,
        states,
        rtpcs,
    })
}

fn read_children(r: &mut Reader) -> Result<Vec<u32>> {
    let n = r.u32()? as usize;
    (0..n).map(|_| r.u32()).collect()
}

/// `CAkBankMgr::LoadSource` (v113..v150): AkBankSourceData.
#[derive(Debug, Clone)]
pub struct Source {
    /// Codec/plugin id: low 4 bits plugin type (1 codec, 2 source plugin), bits 4-15 company,
    /// high 16 bits id. 0x00040001 = Vorbis.
    pub plugin_id: u32,
    /// AKBKSourceType: 0 Data (in-bank/loaded media), 1 PrefetchStreaming, 2 Streaming.
    pub stream_type: u8,
    pub source_id: u32,
    pub in_memory_media_size: u32,
    /// bit0 bIsLanguageSpecific, bit1 bPrefetch, bit3 bNonCachable, bit7 bHasSource
    pub source_bits: u8,
    /// Source-plugin parameter block (only for plugin type 2).
    pub plugin_params: Option<Vec<u8>>,
}

impl Source {
    pub fn is_language_specific(&self) -> bool {
        self.source_bits & 1 != 0
    }
    pub fn plugin_type(&self) -> u32 {
        self.plugin_id & 0xF
    }
}

fn read_plugin_params(r: &mut Reader, plugin_id: u32) -> Result<Option<Vec<u8>>> {
    // wparser_plg.parse_plugin_params: no size field when the id is 0 / -1.
    if plugin_id == 0 || plugin_id == 0xFFFF_FFFF {
        return Ok(None);
    }
    let size = r.u32()? as usize;
    Ok(Some(r.bytes(size)?.to_vec()))
}

fn read_source(r: &mut Reader) -> Result<Source> {
    let plugin_id = r.u32()?;
    let stream_type = r.u8()?;
    let source_id = r.u32()?;
    let in_memory_media_size = r.u32()?;
    let source_bits = r.u8()?;
    let plugin_params = if plugin_id & 0xF == 2 { read_plugin_params(r, plugin_id)? } else { None };
    Ok(Source { plugin_id, stream_type, source_id, in_memory_media_size, source_bits, plugin_params })
}

// ---------------------------------------------------------------------------------------------
// Object types

#[derive(Debug, Clone)]
pub struct State {
    /// (AkRTPC_ParameterID, value)
    pub props: Vec<(u16, f32)>,
}

#[derive(Debug, Clone)]
pub struct Sound {
    pub source: Source,
    pub node: NodeBase,
}

/// Action type high byte (`ulActionType & 0xFF00`, wwiser `CAkAction_ActionTypes_072`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActionKind {
    Stop,
    Pause,
    Resume,
    Play,
    PlayAndContinue,
    Mute,
    SetAkProp,
    UseState,
    SetState,
    SetGameParameter,
    Event,
    SetSwitch,
    BypassFx,
    Break,
    Trigger,
    Seek,
    Release,
    PlayEvent,
    ResetPlaylist,
    SetFx,
    Unknown,
}

impl ActionKind {
    pub fn from_type(t: u16) -> Self {
        use ActionKind::*;
        match t & 0xFF00 {
            0x0100 => Stop,
            0x0200 => Pause,
            0x0300 => Resume,
            0x0400 => Play,
            0x0500 => PlayAndContinue,
            0x0600 | 0x0700 => Mute,
            0x0800..=0x0F00 | 0x2000 | 0x3000 => SetAkProp,
            0x1000 | 0x1100 => UseState,
            0x1200 => SetState,
            0x1300 | 0x1400 => SetGameParameter,
            0x1500..=0x1700 => Event,
            0x1900 => SetSwitch,
            0x1A00 | 0x1B00 | 0x3300..=0x3700 => BypassFx,
            0x1C00 => Break,
            0x1D00 => Trigger,
            0x1E00 => Seek,
            0x1F00 => Release,
            0x2100 => PlayEvent,
            0x2200 => ResetPlaylist,
            0x3100 | 0x3200 => SetFx,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ActionParams {
    /// Stop/Pause/Resume/ResetPlaylist (CAkActionActive): fade curve, specific bits, exceptions.
    Active { fade_curve: u8, specific: Option<u8>, exceptions: Vec<(u32, bool)> },
    Play { fade_curve: u8, bank_id: u32, bank_type: u32 },
    /// Mute / SetAkProp (CAkActionSetValue + CAkActionSetAkProp specific).
    SetValue { fade_curve: u8, value: Option<ValueModifier>, exceptions: Vec<(u32, bool)> },
    SetGameParameter { fade_curve: u8, bypass_transition: bool, value: ValueModifier, exceptions: Vec<(u32, bool)> },
    SetState { group_id: u32, state_id: u32 },
    SetSwitch { group_id: u32, switch_id: u32 },
    BypassFx { bypass: bool, target_mask: u8, exceptions: Vec<(u32, bool)> },
    Seek { relative_to_duration: bool, value: (f32, f32, f32), snap_to_marker: bool, exceptions: Vec<(u32, bool)> },
    None,
}

#[derive(Debug, Clone, Copy)]
pub struct ValueModifier {
    /// AkValueMeaning: 0 Default, 1 Independent (absolute), 2 Offset (relative).
    pub meaning: u8,
    pub base: f32,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub action_type: u16,
    pub kind: ActionKind,
    /// Target element (or event / state value depending on type).
    pub target_id: u32,
    pub target_is_bus: bool,
    pub props: Props,
    pub ranged: RangedProps,
    pub params: ActionParams,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub actions: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct RanSeqCntr {
    pub node: NodeBase,
    pub loop_count: u16,
    pub loop_mod_min: u16,
    pub loop_mod_max: u16,
    pub transition_time: f32,
    pub transition_time_mod_min: f32,
    pub transition_time_mod_max: f32,
    pub avoid_repeat_count: u16,
    /// AkTransitionMode: 0 Disabled, 1 CrossFadeAmp, 2 CrossFadePower, 3 Delay, 4 SampleAccurate, 5 TriggerRate
    pub transition_mode: u8,
    /// AkRandomMode: 0 Normal, 1 Shuffle
    pub random_mode: u8,
    /// AkContainerMode: 0 Random, 1 Sequence
    pub mode: u8,
    /// bit0 IsUsingWeight, bit1 ResetPlayListAtEachPlay, bit2 IsRestartBackward, bit3 IsContinuous, bit4 IsGlobal
    pub bits: u8,
    pub children: Vec<u32>,
    /// (child id, weight)
    pub playlist: Vec<(u32, i32)>,
}

#[derive(Debug, Clone)]
pub struct SwitchNodeParams {
    pub node_id: u32,
    pub bits: u8,
    pub on_switch_mode: u8,
    pub fade_out_ms: i32,
    pub fade_in_ms: i32,
}

#[derive(Debug, Clone)]
pub struct SwitchCntr {
    pub node: NodeBase,
    /// AkGroupType: 0 Switch, 1 State
    pub group_type: u8,
    pub group_id: u32,
    pub default_switch: u32,
    pub continuous_validation: bool,
    pub children: Vec<u32>,
    pub switches: Vec<(u32, Vec<u32>)>,
    pub params: Vec<SwitchNodeParams>,
}

#[derive(Debug, Clone)]
pub struct ActorMixer {
    pub node: NodeBase,
    pub children: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct DuckInfo {
    pub bus_id: u32,
    pub duck_volume: f32,
    pub fade_out_ms: i32,
    pub fade_in_ms: i32,
    pub fade_curve: u8,
    pub target_prop: u8,
}

/// CAkBus / CAkAuxBus.
#[derive(Debug, Clone, Default)]
pub struct Bus {
    pub override_bus_id: u32,
    pub device_shareset_id: Option<u32>,
    pub props: Props,
    pub positioning: Positioning,
    pub aux: AuxParams,
    /// bit0 KillNewest, bit1 UseVirtualBehavior, bit2 IsMaxNumInstIgnoreParent, bit3 IsBackgroundMusic
    pub bits: u8,
    pub max_num_instance: u16,
    /// bits 0-7 channels, 8-11 AkChannelConfigType, 12-31 mask
    pub channel_config: u32,
    /// bit0 IsHdrBus, bit1 HdrReleaseModeExponential
    pub hdr_bits: u8,
    pub recovery_time_ms: i32,
    pub max_duck_volume: f32,
    pub ducks: Vec<DuckInfo>,
    pub fx: FxChain,
    /// Mixer plugin slot (v90..v145 `fxID_0` / `bIsShareSet_0`).
    pub mixer_fx_id: u32,
    pub mixer_is_shareset: bool,
    pub override_attachment_params: u8,
    pub metadata: Vec<(u8, u32, bool)>,
    pub rtpcs: Vec<Rtpc>,
    pub states: StateChunk,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub layer_id: u32,
    pub rtpcs: Vec<Rtpc>,
    pub crossfade_rtpc_id: u32,
    pub crossfade_rtpc_type: u8,
    /// (child id, crossfade curve)
    pub assocs: Vec<(u32, Vec<GraphPoint>)>,
}

#[derive(Debug, Clone)]
pub struct LayerCntr {
    pub node: NodeBase,
    pub children: Vec<u32>,
    pub layers: Vec<Layer>,
    pub continuous_validation: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MeterInfo {
    pub grid_period: f64,
    pub grid_offset: f64,
    pub tempo: f32,
    pub time_sig_num_beats: u8,
    pub time_sig_beat_value: u8,
    pub override_flag: u8,
}

#[derive(Debug, Clone)]
pub struct Stinger {
    pub trigger_id: u32,
    pub segment_id: u32,
    pub sync_play_at: u32,
    pub cue_filter_hash: u32,
    pub dont_repeat_time: i32,
    pub num_segment_look_ahead: u32,
}

/// `CAkMusicNode::SetMusicNodeParams`.
#[derive(Debug, Clone)]
pub struct MusicNode {
    pub flags: u8,
    pub node: NodeBase,
    pub children: Vec<u32>,
    pub meter: MeterInfo,
    pub stingers: Vec<Stinger>,
}

fn read_music_node(r: &mut Reader) -> Result<MusicNode> {
    let flags = r.u8()?;
    let node = read_node_base(r)?;
    let children = read_children(r)?;
    let meter = MeterInfo {
        grid_period: r.f64()?,
        grid_offset: r.f64()?,
        tempo: r.f32()?,
        time_sig_num_beats: r.u8()?,
        time_sig_beat_value: r.u8()?,
        override_flag: r.u8()?,
    };
    let ns = r.u32()? as usize;
    let mut stingers = Vec::with_capacity(ns);
    for _ in 0..ns {
        stingers.push(Stinger {
            trigger_id: r.u32()?,
            segment_id: r.u32()?,
            sync_play_at: r.u32()?,
            cue_filter_hash: r.u32()?,
            dont_repeat_time: r.i32()?,
            num_segment_look_ahead: r.u32()?,
        });
    }
    Ok(MusicNode { flags, node, children, meter, stingers })
}

#[derive(Debug, Clone)]
pub struct MusicMarker {
    pub id: u32,
    pub position_ms: f64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct MusicSegment {
    pub music: MusicNode,
    pub duration_ms: f64,
    pub markers: Vec<MusicMarker>,
}

#[derive(Debug, Clone)]
pub struct TrackSrcInfo {
    pub track_id: u32,
    pub source_id: u32,
    pub event_id: u32,
    pub play_at: f64,
    pub begin_trim_offset: f64,
    pub end_trim_offset: f64,
    pub src_duration: f64,
}

#[derive(Debug, Clone)]
pub struct ClipAutomation {
    pub clip_index: u32,
    /// AkClipAutomationType: 0 Volume, 1 LPF, 2 HPF, 3 FadeIn, 4 FadeOut
    pub auto_type: u32,
    pub points: Vec<GraphPoint>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MusicFade {
    pub transition_time: i32,
    pub fade_curve: u32,
    pub fade_offset: i32,
}

fn read_fade(r: &mut Reader) -> Result<MusicFade> {
    Ok(MusicFade { transition_time: r.i32()?, fade_curve: r.u32()?, fade_offset: r.i32()? })
}

#[derive(Debug, Clone)]
pub struct TrackSwitch {
    pub group_type: u8,
    pub group_id: u32,
    pub default_switch: u32,
    pub switch_assoc: Vec<u32>,
    pub src_fade: MusicFade,
    pub sync_type: u32,
    pub cue_filter_hash: u32,
    pub dest_fade: MusicFade,
}

#[derive(Debug, Clone)]
pub struct MusicTrack {
    pub flags: u8,
    pub sources: Vec<Source>,
    pub playlist: Vec<TrackSrcInfo>,
    pub num_sub_track: u32,
    pub clip_automation: Vec<ClipAutomation>,
    pub node: NodeBase,
    /// AkMusicTrackType: 0 Normal, 1 Random, 2 Sequence, 3 Switch
    pub track_type: u8,
    pub switch: Option<TrackSwitch>,
    pub look_ahead_time: i32,
}

#[derive(Debug, Clone)]
pub struct TransitionRule {
    pub src_ids: Vec<u32>,
    pub dst_ids: Vec<u32>,
    pub src_fade: MusicFade,
    pub src_sync_type: u32,
    pub src_cue_filter_hash: u32,
    pub play_post_exit: bool,
    pub dst_fade: MusicFade,
    pub dst_cue_filter_hash: u32,
    pub jump_to_id: u32,
    pub jump_to_type: u16,
    pub entry_type: u16,
    pub play_pre_entry: bool,
    pub dest_match_source_cue_name: bool,
    pub transition_object: Option<TransitionObject>,
}

#[derive(Debug, Clone)]
pub struct TransitionObject {
    pub segment_id: u32,
    pub fade_in: MusicFade,
    pub fade_out: MusicFade,
    pub play_pre_entry: bool,
    pub play_post_exit: bool,
}

/// `CAkMusicTransAware::SetMusicTransNodeParams`.
#[derive(Debug, Clone)]
pub struct MusicTransNode {
    pub music: MusicNode,
    pub rules: Vec<TransitionRule>,
}

fn read_music_trans_node(r: &mut Reader) -> Result<MusicTransNode> {
    let music = read_music_node(r)?;
    let nr = r.u32()? as usize;
    let mut rules = Vec::with_capacity(nr);
    for _ in 0..nr {
        let ns = r.u32()? as usize;
        let src_ids = (0..ns).map(|_| r.u32()).collect::<Result<Vec<_>>>()?;
        let nd = r.u32()? as usize;
        let dst_ids = (0..nd).map(|_| r.u32()).collect::<Result<Vec<_>>>()?;
        let src_fade = read_fade(r)?;
        let src_sync_type = r.u32()?;
        let src_cue_filter_hash = r.u32()?;
        let play_post_exit = r.u8()? != 0;
        let dst_fade = read_fade(r)?;
        let dst_cue_filter_hash = r.u32()?;
        let jump_to_id = r.u32()?;
        let jump_to_type = r.u16()?;
        let entry_type = r.u16()?;
        let play_pre_entry = r.u8()? != 0;
        let dest_match_source_cue_name = r.u8()? != 0;
        let has_obj = r.u8()? != 0;
        let transition_object = if has_obj {
            Some(TransitionObject {
                segment_id: r.u32()?,
                fade_in: read_fade(r)?,
                fade_out: read_fade(r)?,
                play_pre_entry: r.u8()? != 0,
                play_post_exit: r.u8()? != 0,
            })
        } else {
            None
        };
        rules.push(TransitionRule {
            src_ids,
            dst_ids,
            src_fade,
            src_sync_type,
            src_cue_filter_hash,
            play_post_exit,
            dst_fade,
            dst_cue_filter_hash,
            jump_to_id,
            jump_to_type,
            entry_type,
            play_pre_entry,
            dest_match_source_cue_name,
            transition_object,
        });
    }
    Ok(MusicTransNode { music, rules })
}

/// One 12-byte `AkDecisionTree::Node`. For nodes above max depth `child_or_audio` holds
/// (children index u16 | children count u16 << 16); leaves hold an audio node id.
#[derive(Debug, Clone, Copy)]
pub struct DecisionNode {
    pub key: u32,
    pub child_or_audio: u32,
    pub weight: u16,
    pub probability: u16,
}

#[derive(Debug, Clone)]
pub struct DecisionTree {
    /// (group id, AkGroupType) per depth level.
    pub arguments: Vec<(u32, u8)>,
    /// AkDecisionTree::Mode: 0 BestMatch, 1 Weighted
    pub mode: u8,
    pub nodes: Vec<DecisionNode>,
}

fn read_decision_tree_args(r: &mut Reader) -> Result<Vec<(u32, u8)>> {
    let depth = r.u32()? as usize;
    let groups = (0..depth).map(|_| r.u32()).collect::<Result<Vec<_>>>()?;
    let types = r.bytes(depth)?;
    Ok(groups.into_iter().zip(types.iter().copied()).collect())
}

fn read_decision_tree(r: &mut Reader, arguments: Vec<(u32, u8)>) -> Result<DecisionTree> {
    let size = r.u32()? as usize;
    let mode = r.u8()?;
    if size % 12 != 0 {
        return r.err(format!("decision tree size {} not a multiple of 12", size));
    }
    let mut nodes = Vec::with_capacity(size / 12);
    for _ in 0..size / 12 {
        nodes.push(DecisionNode { key: r.u32()?, child_or_audio: r.u32()?, weight: r.u16()?, probability: r.u16()? });
    }
    Ok(DecisionTree { arguments, mode, nodes })
}

#[derive(Debug, Clone)]
pub struct MusicSwitchCntr {
    pub trans: MusicTransNode,
    pub continue_playback: bool,
    pub tree: DecisionTree,
}

#[derive(Debug, Clone)]
pub struct MusicPlaylistItem {
    pub segment_id: u32,
    pub playlist_item_id: u32,
    /// RSType: 0 ContinuousSequence, 1 StepSequence, 2 ContinuousRandom, 3 StepRandom (-1 leaf)
    pub rs_type: u32,
    pub loop_count: i16,
    pub loop_min: i16,
    pub loop_max: i16,
    pub weight: u32,
    pub avoid_repeat_count: u16,
    pub is_using_weight: bool,
    pub is_shuffle: bool,
    pub children: Vec<MusicPlaylistItem>,
}

fn read_playlist(r: &mut Reader, count: usize, depth: usize) -> Result<Vec<MusicPlaylistItem>> {
    if depth > 64 {
        return r.err("music playlist too deep");
    }
    // Flat pre-order layout: each item is followed by its children (wwiser parse_playlist_node).
    let mut items = Vec::with_capacity(count);
    for _ in 0..count {
        let segment_id = r.u32()?;
        let playlist_item_id = r.u32()?;
        let nchildren = r.u32()? as usize;
        let mut it = MusicPlaylistItem {
            segment_id,
            playlist_item_id,
            rs_type: r.u32()?,
            loop_count: r.i16()?,
            loop_min: r.i16()?,
            loop_max: r.i16()?,
            weight: r.u32()?,
            avoid_repeat_count: r.u16()?,
            is_using_weight: r.u8()? != 0,
            is_shuffle: r.u8()? != 0,
            children: vec![],
        };
        it.children = read_playlist(r, nchildren, depth + 1)?;
        items.push(it);
    }
    Ok(items)
}

#[derive(Debug, Clone)]
pub struct MusicRanSeqCntr {
    pub trans: MusicTransNode,
    pub num_playlist_items: u32,
    pub playlist: Vec<MusicPlaylistItem>,
}

#[derive(Debug, Clone)]
pub struct ConversionCurve {
    pub scaling: u8,
    pub points: Vec<GraphPoint>,
}

#[derive(Debug, Clone)]
pub struct Cone {
    pub inside_degrees: f32,
    pub outside_degrees: f32,
    pub outside_volume: f32,
    pub lo_pass: f32,
    pub hi_pass: f32,
}

#[derive(Debug, Clone)]
pub struct Attenuation {
    pub height_spread_enabled: bool,
    pub cone: Option<Cone>,
    /// Index into `curves` per AttenuationCurveType (19 slots in v142..v154:
    /// VolumeDry, VolumeAuxGameDef, VolumeAuxUserDef, LowPassFilter, HighPassFilter, Spread,
    /// Focus, ObstructionVolume, ObstructionLPF, ObstructionHPF, OcclusionVolume, OcclusionLPF,
    /// OcclusionHPF, DiffractionVolume, DiffractionLPF, DiffractionHPF, TransmissionVolume,
    /// TransmissionLPF, TransmissionHPF); -1 = unused.
    pub curve_to_use: [i8; 19],
    pub curves: Vec<ConversionCurve>,
    pub rtpcs: Vec<Rtpc>,
}

#[derive(Debug, Clone)]
pub struct DialogueEvent {
    pub probability: u8,
    pub tree: DecisionTree,
    pub props: Props,
    pub ranged: RangedProps,
}

/// CAkFxShareSet / CAkFxCustom (`CAkFxBase::SetInitialValues`).
#[derive(Debug, Clone)]
pub struct FxBase {
    pub plugin_id: u32,
    pub params: Option<Vec<u8>>,
    /// (index, media source id) — e.g. impulse responses for Convolution Reverb.
    pub media: Vec<(u8, u32)>,
    pub rtpcs: Vec<Rtpc>,
    pub states: StateChunk,
    /// (property id, accum, value)
    pub property_values: Vec<(u32, u8, f32)>,
}

fn read_fx_base(r: &mut Reader) -> Result<FxBase> {
    let plugin_id = r.u32()?;
    let params = read_plugin_params(r, plugin_id)?;
    let n = r.u8()? as usize;
    let mut media = Vec::with_capacity(n);
    for _ in 0..n {
        media.push((r.u8()?, r.u32()?));
    }
    let rtpcs = read_rtpcs(r)?;
    let states = read_state_chunk(r)?;
    let nv = r.u16()? as usize;
    let mut property_values = Vec::with_capacity(nv);
    for _ in 0..nv {
        property_values.push((r.var()?, r.u8()?, r.f32()?));
    }
    Ok(FxBase { plugin_id, params, media, rtpcs, states, property_values })
}

#[derive(Debug, Clone)]
pub struct Modulator {
    pub props: Props,
    pub ranged: RangedProps,
    pub rtpcs: Vec<Rtpc>,
}

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub fx: FxBase,
    pub slots: FxChain,
}

#[derive(Debug, Clone)]
pub enum HircBody {
    State(State),
    Sound(Sound),
    Action(Action),
    Event(Event),
    RanSeqCntr(RanSeqCntr),
    SwitchCntr(SwitchCntr),
    ActorMixer(ActorMixer),
    Bus(Bus),
    LayerCntr(LayerCntr),
    MusicSegment(MusicSegment),
    MusicTrack(MusicTrack),
    MusicSwitchCntr(MusicSwitchCntr),
    MusicRanSeqCntr(MusicRanSeqCntr),
    Attenuation(Attenuation),
    DialogueEvent(DialogueEvent),
    FxShareSet(FxBase),
    FxCustom(FxBase),
    AuxBus(Bus),
    LfoModulator(Modulator),
    EnvelopeModulator(Modulator),
    AudioDevice(AudioDevice),
    TimeModulator(Modulator),
    Unknown(Vec<u8>),
}

/// HIRC type ids for bank v127+ (wwiser `get_hirc_dispatch`).
pub fn hirc_type_name(t: u8) -> &'static str {
    match t {
        0x01 => "State",
        0x02 => "Sound",
        0x03 => "Action",
        0x04 => "Event",
        0x05 => "RanSeqCntr",
        0x06 => "SwitchCntr",
        0x07 => "ActorMixer",
        0x08 => "Bus",
        0x09 => "LayerCntr",
        0x0a => "MusicSegment",
        0x0b => "MusicTrack",
        0x0c => "MusicSwitchCntr",
        0x0d => "MusicRanSeqCntr",
        0x0e => "Attenuation",
        0x0f => "DialogueEvent",
        0x10 => "FxShareSet",
        0x11 => "FxCustom",
        0x12 => "AuxBus",
        0x13 => "LFOModulator",
        0x14 => "EnvelopeModulator",
        0x15 => "AudioDevice",
        0x16 => "TimeModulator",
        _ => "Unknown",
    }
}

#[derive(Debug, Clone)]
pub struct HircObject {
    pub hirc_type: u8,
    pub id: u32,
    /// Absolute offset of the section body in the bank file.
    pub offset: usize,
    pub size: usize,
    pub body: HircBody,
}

fn read_bus(r: &mut Reader) -> Result<Bus> {
    let override_bus_id = r.u32()?;
    let device_shareset_id = if override_bus_id == 0 { Some(r.u32()?) } else { None };
    // CAkBus::SetInitialParams
    let props = read_props(r)?;
    let positioning = read_positioning(r)?;
    let aux = read_aux(r)?;
    let bits = r.u8()?;
    let max_num_instance = r.u16()?;
    let channel_config = r.u32()?;
    let hdr_bits = r.u8()?;
    let recovery_time_ms = r.i32()?;
    let max_duck_volume = r.f32()?;
    let nd = r.u32()? as usize;
    let mut ducks = Vec::with_capacity(nd);
    for _ in 0..nd {
        ducks.push(DuckInfo {
            bus_id: r.u32()?,
            duck_volume: r.f32()?,
            fade_out_ms: r.i32()?,
            fade_in_ms: r.i32()?,
            fade_curve: r.u8()?,
            target_prop: r.u8()?,
        });
    }
    // CAkBus::SetInitialFxParams (v136+: AkOwnedEffectSlots::SetInitialValues, then mixer slot)
    let fx = read_fx_slots(r, false)?;
    let mixer_fx_id = r.u32()?;
    let mixer_is_shareset = r.u8()? != 0;
    let override_attachment_params = r.u8()?;
    // CAkBus::SetInitialMetadataParams
    let nm = r.u8()?;
    let mut metadata = Vec::with_capacity(nm as usize);
    for _ in 0..nm {
        metadata.push((r.u8()?, r.u32()?, r.u8()? != 0));
    }
    let rtpcs = read_rtpcs(r)?;
    let states = read_state_chunk(r)?;
    Ok(Bus {
        override_bus_id,
        device_shareset_id,
        props,
        positioning,
        aux,
        bits,
        max_num_instance,
        channel_config,
        hdr_bits,
        recovery_time_ms,
        max_duck_volume,
        ducks,
        fx,
        mixer_fx_id,
        mixer_is_shareset,
        override_attachment_params,
        metadata,
        rtpcs,
        states,
    })
}

fn read_action(r: &mut Reader) -> Result<Action> {
    let action_type = r.u16()?;
    let kind = ActionKind::from_type(action_type);
    let target_id = r.u32()?;
    let target_is_bus = r.u8()? & 1 != 0;
    let props = read_props(r)?;
    let ranged = read_ranged_props(r)?;
    let except = |r: &mut Reader| -> Result<Vec<(u32, bool)>> {
        let n = r.var()? as usize;
        (0..n).map(|_| Ok((r.u32()?, r.u8()? != 0))).collect()
    };
    let value_mod = |r: &mut Reader| -> Result<ValueModifier> {
        Ok(ValueModifier { meaning: r.u8()?, base: r.f32()?, min: r.f32()?, max: r.f32()? })
    };
    use ActionKind::*;
    let params = match kind {
        Stop | Pause | Resume => {
            let fade_curve = r.u8()? & 0x1F;
            let specific = Some(r.u8()?);
            ActionParams::Active { fade_curve, specific, exceptions: except(r)? }
        }
        ResetPlaylist => {
            let fade_curve = r.u8()? & 0x1F;
            ActionParams::Active { fade_curve, specific: None, exceptions: except(r)? }
        }
        Play | PlayAndContinue => ActionParams::Play { fade_curve: r.u8()? & 0x1F, bank_id: r.u32()?, bank_type: r.u32()? },
        Mute => {
            let fade_curve = r.u8()? & 0x1F;
            ActionParams::SetValue { fade_curve, value: None, exceptions: except(r)? }
        }
        SetAkProp => {
            let fade_curve = r.u8()? & 0x1F;
            let value = Some(value_mod(r)?);
            ActionParams::SetValue { fade_curve, value, exceptions: except(r)? }
        }
        SetGameParameter => {
            let fade_curve = r.u8()? & 0x1F;
            let bypass_transition = r.u8()? != 0;
            let value = value_mod(r)?;
            ActionParams::SetGameParameter { fade_curve, bypass_transition, value, exceptions: except(r)? }
        }
        SetState => ActionParams::SetState { group_id: r.u32()?, state_id: r.u32()? },
        SetSwitch => ActionParams::SetSwitch { group_id: r.u32()?, switch_id: r.u32()? },
        BypassFx => {
            let bypass = r.u8()? != 0;
            let target_mask = r.u8()?;
            ActionParams::BypassFx { bypass, target_mask, exceptions: except(r)? }
        }
        Seek => {
            let relative_to_duration = r.u8()? != 0;
            let value = (r.f32()?, r.f32()?, r.f32()?);
            let snap_to_marker = r.u8()? != 0;
            ActionParams::Seek { relative_to_duration, value, snap_to_marker, exceptions: except(r)? }
        }
        UseState | Event | Break | Trigger | Release | PlayEvent => ActionParams::None,
        SetFx | Unknown => return r.err(format!("action type {:#06x} not supported for v145", action_type)),
    };
    Ok(Action { action_type, kind, target_id, target_is_bus, props, ranged, params })
}

/// Parses one HIRC section body (after the u8 type + u32 size header).
pub fn parse_object(hirc_type: u8, r: &mut Reader) -> Result<(u32, HircBody)> {
    let id = r.u32()?;
    let body = match hirc_type {
        0x01 => {
            let n = r.u16()? as usize;
            let ids = (0..n).map(|_| r.u16()).collect::<Result<Vec<_>>>()?;
            let mut props = Vec::with_capacity(n);
            for i in ids {
                props.push((i, r.f32()?));
            }
            HircBody::State(State { props })
        }
        0x02 => HircBody::Sound(Sound { source: read_source(r)?, node: read_node_base(r)? }),
        0x03 => HircBody::Action(read_action(r)?),
        0x04 => {
            let n = r.var()? as usize;
            HircBody::Event(Event { actions: (0..n).map(|_| r.u32()).collect::<Result<_>>()? })
        }
        0x05 => {
            let node = read_node_base(r)?;
            let mut c = RanSeqCntr {
                node,
                loop_count: r.u16()?,
                loop_mod_min: r.u16()?,
                loop_mod_max: r.u16()?,
                transition_time: r.f32()?,
                transition_time_mod_min: r.f32()?,
                transition_time_mod_max: r.f32()?,
                avoid_repeat_count: r.u16()?,
                transition_mode: r.u8()?,
                random_mode: r.u8()?,
                mode: r.u8()?,
                bits: r.u8()?,
                children: vec![],
                playlist: vec![],
            };
            c.children = read_children(r)?;
            let n = r.u16()? as usize;
            for _ in 0..n {
                c.playlist.push((r.u32()?, r.i32()?));
            }
            HircBody::RanSeqCntr(c)
        }
        0x06 => {
            let node = read_node_base(r)?;
            let group_type = r.u8()?;
            let group_id = r.u32()?;
            let default_switch = r.u32()?;
            let continuous_validation = r.u8()? != 0;
            let children = read_children(r)?;
            let ns = r.u32()? as usize;
            let mut switches = Vec::with_capacity(ns);
            for _ in 0..ns {
                let sid = r.u32()?;
                let n = r.u32()? as usize;
                switches.push((sid, (0..n).map(|_| r.u32()).collect::<Result<Vec<_>>>()?));
            }
            let np = r.u32()? as usize;
            let mut params = Vec::with_capacity(np);
            for _ in 0..np {
                params.push(SwitchNodeParams {
                    node_id: r.u32()?,
                    bits: r.u8()?,
                    on_switch_mode: r.u8()? & 7,
                    fade_out_ms: r.i32()?,
                    fade_in_ms: r.i32()?,
                });
            }
            HircBody::SwitchCntr(SwitchCntr {
                node,
                group_type,
                group_id,
                default_switch,
                continuous_validation,
                children,
                switches,
                params,
            })
        }
        0x07 => HircBody::ActorMixer(ActorMixer { node: read_node_base(r)?, children: read_children(r)? }),
        0x08 => HircBody::Bus(read_bus(r)?),
        0x09 => {
            let node = read_node_base(r)?;
            let children = read_children(r)?;
            let nl = r.u32()? as usize;
            let mut layers = Vec::with_capacity(nl);
            for _ in 0..nl {
                let layer_id = r.u32()?;
                let rtpcs = read_rtpcs(r)?;
                let crossfade_rtpc_id = r.u32()?;
                let crossfade_rtpc_type = r.u8()?;
                let na = r.u32()? as usize;
                let mut assocs = Vec::with_capacity(na);
                for _ in 0..na {
                    let child = r.u32()?;
                    let n = r.u32()? as usize;
                    assocs.push((child, read_graph(r, n)?));
                }
                layers.push(Layer { layer_id, rtpcs, crossfade_rtpc_id, crossfade_rtpc_type, assocs });
            }
            let continuous_validation = r.u8()? != 0;
            HircBody::LayerCntr(LayerCntr { node, children, layers, continuous_validation })
        }
        0x0a => {
            let music = read_music_node(r)?;
            let duration_ms = r.f64()?;
            let nm = r.u32()? as usize;
            let mut markers = Vec::with_capacity(nm);
            for _ in 0..nm {
                markers.push(MusicMarker { id: r.u32()?, position_ms: r.f64()?, name: r.stz()? });
            }
            HircBody::MusicSegment(MusicSegment { music, duration_ms, markers })
        }
        0x0b => {
            let flags = r.u8()?;
            let ns = r.u32()? as usize;
            let sources = (0..ns).map(|_| read_source(r)).collect::<Result<Vec<_>>>()?;
            let np = r.u32()? as usize;
            let mut playlist = Vec::with_capacity(np);
            let mut num_sub_track = 0;
            if np > 0 {
                for _ in 0..np {
                    playlist.push(TrackSrcInfo {
                        track_id: r.u32()?,
                        source_id: r.u32()?,
                        event_id: r.u32()?,
                        play_at: r.f64()?,
                        begin_trim_offset: r.f64()?,
                        end_trim_offset: r.f64()?,
                        src_duration: r.f64()?,
                    });
                }
                num_sub_track = r.u32()?;
            }
            let nc = r.u32()? as usize;
            let mut clip_automation = Vec::with_capacity(nc);
            for _ in 0..nc {
                let clip_index = r.u32()?;
                let auto_type = r.u32()?;
                let n = r.u32()? as usize;
                clip_automation.push(ClipAutomation { clip_index, auto_type, points: read_graph(r, n)? });
            }
            let node = read_node_base(r)?;
            let track_type = r.u8()?;
            let switch = if track_type == 3 {
                let group_type = r.u8()?;
                let group_id = r.u32()?;
                let default_switch = r.u32()?;
                let n = r.u32()? as usize;
                let switch_assoc = (0..n).map(|_| r.u32()).collect::<Result<Vec<_>>>()?;
                Some(TrackSwitch {
                    group_type,
                    group_id,
                    default_switch,
                    switch_assoc,
                    src_fade: read_fade(r)?,
                    sync_type: r.u32()?,
                    cue_filter_hash: r.u32()?,
                    dest_fade: read_fade(r)?,
                })
            } else {
                None
            };
            let look_ahead_time = r.i32()?;
            HircBody::MusicTrack(MusicTrack {
                flags,
                sources,
                playlist,
                num_sub_track,
                clip_automation,
                node,
                track_type,
                switch,
                look_ahead_time,
            })
        }
        0x0c => {
            let trans = read_music_trans_node(r)?;
            let continue_playback = r.u8()? != 0;
            let args = read_decision_tree_args(r)?;
            let tree = read_decision_tree(r, args)?;
            HircBody::MusicSwitchCntr(MusicSwitchCntr { trans, continue_playback, tree })
        }
        0x0d => {
            let trans = read_music_trans_node(r)?;
            let num_playlist_items = r.u32()?;
            let playlist = read_playlist(r, 1, 0)?;
            HircBody::MusicRanSeqCntr(MusicRanSeqCntr { trans, num_playlist_items, playlist })
        }
        0x0e => {
            let height_spread_enabled = r.u8()? != 0;
            let cone_enabled = r.u8()?;
            let cone = if cone_enabled & 1 != 0 {
                Some(Cone {
                    inside_degrees: r.f32()?,
                    outside_degrees: r.f32()?,
                    outside_volume: r.f32()?,
                    lo_pass: r.f32()?,
                    hi_pass: r.f32()?,
                })
            } else {
                None
            };
            let mut curve_to_use = [0i8; 19];
            for c in curve_to_use.iter_mut() {
                *c = r.i8()?;
            }
            let nc = r.u8()? as usize;
            let mut curves = Vec::with_capacity(nc);
            for _ in 0..nc {
                let scaling = r.u8()?;
                let n = r.u16()? as usize;
                curves.push(ConversionCurve { scaling, points: read_graph(r, n)? });
            }
            let rtpcs = read_rtpcs(r)?;
            HircBody::Attenuation(Attenuation { height_spread_enabled, cone, curve_to_use, curves, rtpcs })
        }
        0x0f => {
            let probability = r.u8()?;
            let args = read_decision_tree_args(r)?;
            let tree = read_decision_tree(r, args)?;
            HircBody::DialogueEvent(DialogueEvent {
                probability,
                tree,
                props: read_props(r)?,
                ranged: read_ranged_props(r)?,
            })
        }
        0x10 => HircBody::FxShareSet(read_fx_base(r)?),
        0x11 => HircBody::FxCustom(read_fx_base(r)?),
        0x12 => HircBody::AuxBus(read_bus(r)?),
        0x13 | 0x14 | 0x16 => {
            let m = Modulator { props: read_props(r)?, ranged: read_ranged_props(r)?, rtpcs: read_rtpcs(r)? };
            match hirc_type {
                0x13 => HircBody::LfoModulator(m),
                0x14 => HircBody::EnvelopeModulator(m),
                _ => HircBody::TimeModulator(m),
            }
        }
        0x15 => HircBody::AudioDevice(AudioDevice { fx: read_fx_base(r)?, slots: read_fx_slots(r, false)? }),
        _ => HircBody::Unknown(r.bytes(r.remaining())?.to_vec()),
    };
    Ok((id, body))
}

impl HircObject {
    /// Parent/child edges and other object references used for graph walks.
    pub fn node_base(&self) -> Option<&NodeBase> {
        match &self.body {
            HircBody::Sound(s) => Some(&s.node),
            HircBody::RanSeqCntr(c) => Some(&c.node),
            HircBody::SwitchCntr(c) => Some(&c.node),
            HircBody::ActorMixer(c) => Some(&c.node),
            HircBody::LayerCntr(c) => Some(&c.node),
            HircBody::MusicSegment(m) => Some(&m.music.node),
            HircBody::MusicTrack(m) => Some(&m.node),
            HircBody::MusicSwitchCntr(m) => Some(&m.trans.music.node),
            HircBody::MusicRanSeqCntr(m) => Some(&m.trans.music.node),
            _ => None,
        }
    }

    pub fn children(&self) -> &[u32] {
        match &self.body {
            HircBody::RanSeqCntr(c) => &c.children,
            HircBody::SwitchCntr(c) => &c.children,
            HircBody::ActorMixer(c) => &c.children,
            HircBody::LayerCntr(c) => &c.children,
            HircBody::MusicSegment(m) => &m.music.children,
            HircBody::MusicSwitchCntr(m) => &m.trans.music.children,
            HircBody::MusicRanSeqCntr(m) => &m.trans.music.children,
            _ => &[],
        }
    }

    pub fn rtpcs(&self) -> &[Rtpc] {
        if let Some(n) = self.node_base() {
            return &n.rtpcs;
        }
        match &self.body {
            HircBody::Bus(b) | HircBody::AuxBus(b) => &b.rtpcs,
            HircBody::Attenuation(a) => &a.rtpcs,
            HircBody::FxShareSet(f) | HircBody::FxCustom(f) => &f.rtpcs,
            HircBody::AudioDevice(d) => &d.fx.rtpcs,
            HircBody::LfoModulator(m) | HircBody::EnvelopeModulator(m) | HircBody::TimeModulator(m) => &m.rtpcs,
            _ => &[],
        }
    }

    pub fn sources(&self) -> Vec<&Source> {
        match &self.body {
            HircBody::Sound(s) => vec![&s.source],
            HircBody::MusicTrack(t) => t.sources.iter().collect(),
            _ => vec![],
        }
    }
}
