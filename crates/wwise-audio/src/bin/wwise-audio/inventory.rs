//! `wwise-audio inventory`: parse every bank (and .wem header) shipped with the game and report
//! what the Wwise runtime replacement has to support.

use wwise_audio::bnk::hirc::{hirc_type_name, prop, ActionKind, HircBody};
use wwise_audio::bnk::{plugin_name, Bank};
use wwise_audio::data::GameAudio;
use wwise_audio::wem::Wem;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::Mutex;

#[derive(Default)]
struct Totals {
    banks: usize,
    bank_errors: Vec<(String, String)>,
    placeholder_banks: BTreeMap<String, usize>,
    placeholder_examples: Vec<String>,
    didx_non_riff: BTreeMap<String, usize>,
    versions: BTreeMap<u32, usize>,
    bank_types: BTreeMap<u32, usize>,
    chunk_sets: BTreeMap<String, usize>,
    hirc_type: BTreeMap<u8, usize>,
    hirc_unique: BTreeMap<u8, HashSet<u32>>,
    hirc_errors: Vec<(String, u8, u32, String)>,
    action_types: BTreeMap<u16, usize>,
    action_kinds: BTreeMap<String, usize>,
    source_plugins: BTreeMap<u32, usize>,
    stream_types: BTreeMap<u8, usize>,
    lang_specific_sources: usize,
    streamed_source_ids: HashSet<u32>,
    memory_source_ids: HashSet<u32>,
    fx_plugins: BTreeMap<u32, (usize, HashSet<u32>)>,
    fx_media: usize,
    device_plugins: BTreeMap<u32, usize>,
    mixer_plugins: BTreeMap<u32, usize>,
    fx_refs_by_owner: BTreeMap<&'static str, usize>,
    rtpc_types: BTreeMap<u8, usize>,
    rtpc_params: BTreeMap<u32, usize>,
    rtpc_ids: HashSet<u32>,
    container_modes: BTreeMap<String, usize>,
    switch_group_types: BTreeMap<u8, usize>,
    music_track_types: BTreeMap<u8, usize>,
    positioning: BTreeMap<String, usize>,
    props_used: BTreeMap<u8, usize>,
    ducking_buses: usize,
    hdr_buses: usize,
    aux_sends: usize,
    reflections_aux: usize,
    diffraction: usize,
    attenuation_curves: BTreeMap<usize, usize>,
    cone_attenuations: usize,
    didx_media: usize,
    didx_bytes: u64,
    didx_codecs: BTreeMap<String, usize>,
    didx_prefetch: usize,
    dialogue_events: usize,
    stingers: usize,
    transition_rules: usize,
    state_props_states: usize,
    max_tree_depth: usize,
    markers: usize,
    init: Option<String>,
}

fn kind_name(k: ActionKind) -> String {
    format!("{:?}", k)
}

fn scan_bank(path: &str, buf: &[u8], t: &mut Totals) {
    // Some shipped .bnk files are tiny placeholders (observed: 1 byte, ASCII "0") — not banks.
    if buf.len() < 16 && !buf.starts_with(b"BKHD") {
        *t.placeholder_banks.entry(format!("{} bytes {:02x?}", buf.len(), buf)).or_default() += 1;
        t.placeholder_examples.push(path.to_string());
        return;
    }
    let bank = match Bank::parse(buf) {
        Ok(b) => b,
        Err(e) => {
            t.bank_errors.push((path.to_string(), e.to_string()));
            return;
        }
    };
    t.banks += 1;
    *t.versions.entry(bank.header.version).or_default() += 1;
    *t.bank_types.entry(bank.header.bank_type).or_default() += 1;
    let cs: Vec<String> = bank.chunks.iter().map(|c| c.name()).collect();
    *t.chunk_sets.entry(cs.join(",")).or_default() += 1;
    for (ty, id, _off, e) in &bank.hirc_errors {
        t.hirc_errors.push((path.to_string(), *ty, *id, e.clone()));
    }
    // DIDX media
    let mut media_users: HashMap<u32, String> = HashMap::new();
    for o in &bank.hirc {
        for s in o.sources() {
            media_users.entry(s.source_id).or_insert_with(|| format!("source plugin {:#010x} {}", s.plugin_id, plugin_name(s.plugin_id)));
        }
        if let HircBody::FxShareSet(f) | HircBody::FxCustom(f) = &o.body {
            for (_, mid) in &f.media {
                media_users.entry(*mid).or_insert_with(|| format!("fx media of {:#010x} {}", f.plugin_id, plugin_name(f.plugin_id)));
            }
        }
    }
    for m in &bank.media_index {
        t.didx_media += 1;
        t.didx_bytes += m.size as u64;
        if let Some(w) = bank.media(buf, m.id) {
            if !w.starts_with(b"RIFF") {
                let user = media_users.get(&m.id).cloned().unwrap_or_else(|| "unreferenced in own bank".into());
                let magic: Vec<u8> = w.iter().take(4).copied().collect();
                *t.didx_non_riff.entry(format!("magic {:02x?} <- {}", magic, user)).or_default() += 1;
                continue;
            }
            match Wem::parse(w) {
                Ok(wem) => {
                    if wem.truncated {
                        t.didx_prefetch += 1;
                    }
                    *t.didx_codecs.entry(format!("{} {}ch {}Hz", wem.codec.name(), wem.channels, wem.sample_rate)).or_default() += 1;
                }
                Err(e) => *t.didx_codecs.entry(format!("PARSE ERROR {}", e.msg)).or_default() += 1,
            }
        }
    }
    if path.ends_with("/Init.bnk") {
        let mut s = String::new();
        use std::fmt::Write;
        for (id, dll) in &bank.plugins {
            let _ = writeln!(s, "    {:#010x}  {:<34} dll={:?}", id, plugin_name(*id), dll);
        }
        if let Some(g) = &bank.global_settings {
            let _ = writeln!(
                s,
                "  STMG: filter behavior {}, volume threshold {} dB, max voices {}, max dangerous virtual voices {}\n  STMG: {} state groups ({} with custom transitions), {} switch groups bound to RTPCs, {} game parameters (RTPC ramping), {} acoustic textures",
                g.filter_behavior,
                g.volume_threshold,
                g.max_num_voices,
                g.max_num_dangerous_virt_voices,
                g.state_groups.len(),
                g.state_groups.iter().filter(|x| !x.transitions.is_empty()).count(),
                g.switch_groups.len(),
                g.rtpc_ramping.len(),
                g.acoustic_textures.len()
            );
            let mut builtin: BTreeMap<u8, usize> = BTreeMap::new();
            let mut ramp: BTreeMap<u32, usize> = BTreeMap::new();
            for r in &g.rtpc_ramping {
                *builtin.entry(r.bind_to_builtin).or_default() += 1;
                *ramp.entry(r.ramp_type).or_default() += 1;
            }
            let _ = writeln!(s, "  STMG: game parameter built-in bindings {:?}; ramp types {:?}", builtin, ramp);
        }
        let _ = writeln!(s, "  ENVS: {} obstruction/occlusion curves ({} enabled)", bank.env_curves.len(), bank.env_curves.iter().filter(|c| c.enabled).count());
        t.init = Some(s);
    }
    for o in &bank.hirc {
        *t.hirc_type.entry(o.hirc_type).or_default() += 1;
        t.hirc_unique.entry(o.hirc_type).or_default().insert(o.id);
        for r in o.rtpcs() {
            *t.rtpc_types.entry(r.rtpc_type).or_default() += 1;
            *t.rtpc_params.entry(r.param_id).or_default() += 1;
            t.rtpc_ids.insert(r.rtpc_id);
        }
        if let Some(n) = o.node_base() {
            for (p, _) in &n.props.0 {
                *t.props_used.entry(*p).or_default() += 1;
            }
            let pos = &n.positioning;
            if pos.override_parent() {
                let key = if pos.has_listener_relative_routing() {
                    format!(
                        "3D (spatialization {} pos-type {} atten {} automation {})",
                        pos.spatialization_mode(),
                        pos.position_type_3d(),
                        pos.enable_attenuation(),
                        pos.automation.is_some()
                    )
                } else {
                    format!("2D (panner {})", pos.panner_type())
                };
                *t.positioning.entry(key).or_default() += 1;
            }
            if pos.enable_diffraction() {
                t.diffraction += 1;
            }
            if n.aux.aux_ids.is_some() {
                t.aux_sends += 1;
            }
            if n.aux.reflections_aux_bus != 0 {
                t.reflections_aux += 1;
            }
            if !n.fx.slots.is_empty() {
                *t.fx_refs_by_owner.entry("node fx slots").or_default() += n.fx.slots.len();
            }
            for g in &n.states.groups {
                t.state_props_states += g.states.len();
            }
        }
        for s in o.sources() {
            *t.source_plugins.entry(s.plugin_id).or_default() += 1;
            *t.stream_types.entry(s.stream_type).or_default() += 1;
            if s.is_language_specific() {
                t.lang_specific_sources += 1;
            }
            if s.stream_type == 0 {
                t.memory_source_ids.insert(s.source_id);
            } else {
                t.streamed_source_ids.insert(s.source_id);
            }
        }
        match &o.body {
            HircBody::Action(a) => {
                *t.action_types.entry(a.action_type).or_default() += 1;
                *t.action_kinds.entry(kind_name(a.kind)).or_default() += 1;
            }
            HircBody::RanSeqCntr(c) => {
                let m = format!(
                    "{} ({}) loop={} transition_mode={}",
                    if c.mode == 0 { "Random" } else { "Sequence" },
                    if c.random_mode == 0 { "normal" } else { "shuffle" },
                    if c.loop_count == 1 { "once".to_string() } else if c.loop_count == 0 { "infinite".into() } else { c.loop_count.to_string() },
                    c.transition_mode
                );
                *t.container_modes.entry(m).or_default() += 1;
            }
            HircBody::SwitchCntr(c) => *t.switch_group_types.entry(c.group_type).or_default() += 1,
            HircBody::MusicTrack(m) => *t.music_track_types.entry(m.track_type).or_default() += 1,
            HircBody::MusicSegment(m) => {
                t.stingers += m.music.stingers.len();
                t.markers += m.markers.len();
            }
            HircBody::MusicSwitchCntr(m) => {
                t.transition_rules += m.trans.rules.len();
                t.max_tree_depth = t.max_tree_depth.max(m.tree.arguments.len());
                t.stingers += m.trans.music.stingers.len();
            }
            HircBody::MusicRanSeqCntr(m) => {
                t.transition_rules += m.trans.rules.len();
                t.stingers += m.trans.music.stingers.len();
            }
            HircBody::DialogueEvent(d) => {
                t.dialogue_events += 1;
                t.max_tree_depth = t.max_tree_depth.max(d.tree.arguments.len());
            }
            HircBody::Bus(b) | HircBody::AuxBus(b) => {
                if !b.ducks.is_empty() {
                    t.ducking_buses += 1;
                }
                if b.hdr_bits & 1 != 0 {
                    t.hdr_buses += 1;
                }
                if b.mixer_fx_id != 0 {
                    *t.mixer_plugins.entry(b.mixer_fx_id).or_default() += 1;
                }
                *t.fx_refs_by_owner.entry("bus fx slots").or_default() += b.fx.slots.len();
            }
            HircBody::FxShareSet(f) | HircBody::FxCustom(f) => {
                let e = t.fx_plugins.entry(f.plugin_id).or_default();
                e.0 += 1;
                e.1.insert(o.id);
                t.fx_media += f.media.len();
            }
            HircBody::AudioDevice(d) => *t.device_plugins.entry(d.fx.plugin_id).or_default() += 1,
            HircBody::Attenuation(a) => {
                *t.attenuation_curves.entry(a.curves.len()).or_default() += 1;
                if a.cone.is_some() {
                    t.cone_attenuations += 1;
                }
            }
            _ => {}
        }
    }
}

#[derive(Default)]
struct WemTotals {
    files: usize,
    bytes: u64,
    errors: Vec<(String, String)>,
    by_codec: BTreeMap<String, (usize, u64)>,
    by_category: BTreeMap<(String, String), usize>,
    chunk_sets: BTreeMap<String, usize>,
    loops: usize,
    vorbis_blocksizes: BTreeMap<(u8, u8), usize>,
    /// (channels, AkChannelConfig type, channel mask) -> count
    channel_layouts: BTreeMap<(u16, u8, u32), usize>,
}

fn category(path: &str) -> String {
    let rest = path.strip_prefix("data_pc/audio/").unwrap_or(path);
    let parts: Vec<&str> = rest.split('/').collect();
    match parts.as_slice() {
        ["dialogue", "script", "locale", lang, ..] => format!("dialogue/{}", lang),
        [one] => format!("(root) {}", one.split('_').next().unwrap_or("")),
        [first, ..] => first.to_string(),
        _ => "?".into(),
    }
}

pub fn inventory(sample: usize) -> Result<(), String> {
    let ga = GameAudio::open()?;
    let banks = ga.banks();
    let wems = ga.wems();
    eprintln!("scanning {} banks...", banks.len());
    let t0 = std::time::Instant::now();
    let nthreads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4).min(12);

    let totals = Mutex::new(Totals::default());
    std::thread::scope(|s| {
        for chunk in banks.chunks(banks.len().div_ceil(nthreads)) {
            let ga = &ga;
            let totals = &totals;
            s.spawn(move || {
                let mut local = Totals::default();
                for f in chunk {
                    match ga.read(&f.path) {
                        Ok(buf) => scan_bank(&f.path, &buf, &mut local),
                        Err(e) => local.bank_errors.push((f.path.clone(), e)),
                    }
                }
                merge(&mut totals.lock().unwrap(), local);
            });
        }
    });
    let t = totals.into_inner().unwrap();
    let bank_time = t0.elapsed();

    // .wem headers
    let step = if sample == usize::MAX { 1 } else { (wems.len() / sample.max(1)).max(1) };
    let picked: Vec<_> = wems.iter().step_by(step).collect();
    eprintln!("scanning {} of {} .wem headers...", picked.len(), wems.len());
    let t1 = std::time::Instant::now();
    let wt = Mutex::new(WemTotals::default());
    std::thread::scope(|s| {
        for chunk in picked.chunks(picked.len().div_ceil(nthreads).max(1)) {
            let ga = &ga;
            let wt = &wt;
            s.spawn(move || {
                let mut l = WemTotals::default();
                for f in chunk {
                    let buf = match ga.read(&f.path) {
                        Ok(b) => b,
                        Err(e) => {
                            l.errors.push((f.path.clone(), e));
                            continue;
                        }
                    };
                    l.files += 1;
                    l.bytes += buf.len() as u64;
                    match Wem::parse(&buf) {
                        Ok(w) => {
                            let key = format!("{} {}ch {}Hz", w.codec.name(), w.channels, w.sample_rate);
                            let e = l.by_codec.entry(key).or_default();
                            e.0 += 1;
                            e.1 += buf.len() as u64;
                            *l.by_category.entry((category(&f.path), w.codec.name())).or_default() += 1;
                            *l.chunk_sets.entry(w.chunk_names()).or_default() += 1;
                            if w.loop_points.is_some() {
                                l.loops += 1;
                            }
                            if let Some(v) = &w.vorbis {
                                *l.vorbis_blocksizes.entry((v.blocksize_0_exp, v.blocksize_1_exp)).or_default() += 1;
                            }
                            *l.channel_layouts.entry((w.channels, w.channel_config_type, w.channel_mask)).or_default() += 1;
                        }
                        Err(e) => l.errors.push((f.path.clone(), e.to_string())),
                    }
                }
                let mut g = wt.lock().unwrap();
                g.files += l.files;
                g.bytes += l.bytes;
                g.errors.extend(l.errors);
                g.loops += l.loops;
                for (k, v) in l.by_codec {
                    let e = g.by_codec.entry(k).or_default();
                    e.0 += v.0;
                    e.1 += v.1;
                }
                add_map(&mut g.by_category, l.by_category);
                add_map(&mut g.chunk_sets, l.chunk_sets);
                add_map(&mut g.vorbis_blocksizes, l.vorbis_blocksizes);
                add_map(&mut g.channel_layouts, l.channel_layouts);
            });
        }
    });
    let w = wt.into_inner().unwrap();
    let wem_time = t1.elapsed();

    // Streamed sources vs loose media files
    let media_ids: HashSet<u32> = ga
        .list(wwise_audio::data::MEDIA_DIR)
        .iter()
        .filter_map(|f| f.path.rsplit('/').next()?.strip_suffix(".wem")?.parse().ok())
        .collect();
    let streamed_found = t.streamed_source_ids.iter().filter(|id| media_ids.contains(id)).count();

    // ---- report ----
    println!("# wwise-audio inventory");
    println!("banks parsed: {} / {} ({:.1} s), bank-level errors: {}", t.banks, banks.len(), bank_time.as_secs_f64(), t.bank_errors.len());
    for (p, e) in t.bank_errors.iter().take(10) {
        println!("  BANK ERROR {}: {}", p, e);
    }
    for (k, n) in &t.placeholder_banks {
        let mut ex = t.placeholder_examples.clone();
        ex.sort();
        println!("  placeholder .bnk files (not banks): {} x [{}], e.g. {}", n, k, ex.iter().take(3).cloned().collect::<Vec<_>>().join(", "));
    }
    println!("bank versions: {:?}", t.versions);
    println!("bank types (AkBankTypeEnum): {:?}", t.bank_types);
    println!("chunk layouts:");
    for (k, v) in &t.chunk_sets {
        println!("  {:>5}  {}", v, k);
    }
    let total_objs: usize = t.hirc_type.values().sum();
    println!("\n## HIRC ({} objects, {} parse errors)", total_objs, t.hirc_errors.len());
    for (p, ty, id, e) in t.hirc_errors.iter().take(20) {
        println!("  HIRC ERROR {} type {:#04x} id {}: {}", p, ty, id, e);
    }
    println!("  type                 objects   unique ids");
    for (ty, n) in &t.hirc_type {
        println!("  {:#04x} {:<16} {:>8} {:>10}", ty, hirc_type_name(*ty), n, t.hirc_unique[ty].len());
    }
    println!("\n## Actions by type (ulActionType)");
    for (k, v) in &t.action_kinds {
        println!("  {:<18} {}", k, v);
    }
    println!("  raw: {}", t.action_types.iter().map(|(k, v)| format!("{:#06x}:{}", k, v)).collect::<Vec<_>>().join(" "));
    println!("\n## Sources (Sound + MusicTrack)");
    for (k, v) in &t.source_plugins {
        println!("  plugin {:#010x} {:<24} {}", k, plugin_name(*k), v);
    }
    println!("  stream types (0 in-memory, 1 prefetch+stream, 2 stream): {:?}", t.stream_types);
    println!("  language-specific sources: {}", t.lang_specific_sources);
    println!(
        "  unique streamed source ids: {} (found as data_pc/audio/media/<id>.wem: {}), unique in-memory source ids: {}",
        t.streamed_source_ids.len(),
        streamed_found,
        t.memory_source_ids.len()
    );
    println!("  loose media files in data_pc/audio/media: {}", media_ids.len());
    println!("  DIDX embedded media: {} entries, {:.1} MB, {} prefetch (truncated) copies", t.didx_media, t.didx_bytes as f64 / 1e6, t.didx_prefetch);
    for (k, n) in &t.didx_non_riff {
        println!("    non-RIFF media: {:>5}  {}", n, k);
    }
    for (k, v) in &t.didx_codecs {
        println!("    {:>6}  {}", v, k);
    }
    println!("\n## Effect / plugin usage");
    println!("  FxShareSet+FxCustom objects by plugin (instances, unique ids):");
    for (k, (n, ids)) in &t.fx_plugins {
        println!("    {:#010x} {:<34} {:>6} {:>6}", k, plugin_name(*k), n, ids.len());
    }
    println!("  effect media refs (e.g. convolution IRs): {}", t.fx_media);
    println!("  audio device plugins: {}", t.device_plugins.iter().map(|(k, v)| format!("{:#010x} {} x{}", k, plugin_name(*k), v)).collect::<Vec<_>>().join(", "));
    println!("  bus mixer plugins: {}", t.mixer_plugins.iter().map(|(k, v)| format!("{:#010x} {} x{}", k, plugin_name(*k), v)).collect::<Vec<_>>().join(", "));
    println!("  fx slot refs: {:?}", t.fx_refs_by_owner);
    if let Some(s) = &t.init {
        println!("\n## Init.bnk\n  INIT plugin list (registered by project):\n{}", s);
    }
    println!("\n## Containers / music / dialogue");
    for (k, v) in &t.container_modes {
        println!("  RanSeqCntr {:<60} {}", k, v);
    }
    println!("  SwitchCntr group types (0 switch, 1 state): {:?}", t.switch_group_types);
    println!("  MusicTrack types (0 normal, 1 random, 2 sequence, 3 switch): {:?}", t.music_track_types);
    println!("  music transition rules: {}, stingers: {}, markers: {}", t.transition_rules, t.stingers, t.markers);
    println!("  dialogue events: {}, max decision tree depth: {}", t.dialogue_events, t.max_tree_depth);
    println!("\n## Parameters");
    println!("  RTPC curves by type (0 game param, 1 MIDI, 2 modulator): {:?}; unique RTPC ids: {}", t.rtpc_types, t.rtpc_ids.len());
    println!("  RTPC target params: {}", t.rtpc_params.iter().map(|(k, v)| format!("{}:{}", k, v)).collect::<Vec<_>>().join(" "));
    println!("  state-chunk state entries: {}", t.state_props_states);
    println!("  props set on nodes: {}", t.props_used.iter().map(|(k, v)| format!("{}:{}", prop::name(*k), v)).collect::<Vec<_>>().join(" "));
    println!("\n## Positioning / spatial");
    for (k, v) in &t.positioning {
        println!("  {:<64} {}", k, v);
    }
    println!("  nodes with game/user aux sends: {}, with reflections aux bus: {}, diffraction enabled: {}", t.aux_sends, t.reflections_aux, t.diffraction);
    println!("  attenuations by curve count: {:?}, with cone: {}", t.attenuation_curves, t.cone_attenuations);
    println!("  buses with ducking: {}, HDR buses: {}", t.ducking_buses, t.hdr_buses);

    println!("\n## .wem files ({} scanned of {}, {:.2} GB, {:.1} s, {} errors)", w.files, wems.len(), w.bytes as f64 / 1e9, wem_time.as_secs_f64(), w.errors.len());
    for (p, e) in w.errors.iter().take(10) {
        println!("  WEM ERROR {}: {}", p, e);
    }
    for (k, (n, b)) in &w.by_codec {
        println!("  {:>6}  {:>9.1} MB  {}", n, *b as f64 / 1e6, k);
    }
    println!("  by category:");
    for ((c, k), n) in &w.by_category {
        println!("    {:<28} {:<20} {}", c, k, n);
    }
    println!("  chunk layouts:");
    for (k, v) in &w.chunk_sets {
        println!("    {:>6}  {}", v, k);
    }
    println!("  with smpl loop points: {}", w.loops);
    println!("  Vorbis blocksize exps (short, long): {:?}", w.vorbis_blocksizes);
    println!("  channel layouts (channels, AkChannelConfig type 0=anonymous 1=standard 2=ambisonic, WAVEFORMATEXTENSIBLE-style mask):");
    for ((ch, ty, mask), n) in &w.channel_layouts {
        println!("    {:>6}  {:>2} ch  type {}  mask {:#07x}", n, ch, ty, mask);
    }
    Ok(())
}

fn add_map<K: Ord>(a: &mut BTreeMap<K, usize>, b: BTreeMap<K, usize>) {
    for (k, v) in b {
        *a.entry(k).or_default() += v;
    }
}

fn merge(a: &mut Totals, b: Totals) {
    a.banks += b.banks;
    a.bank_errors.extend(b.bank_errors);
    add_map(&mut a.placeholder_banks, b.placeholder_banks);
    a.placeholder_examples.extend(b.placeholder_examples);
    add_map(&mut a.didx_non_riff, b.didx_non_riff);
    add_map(&mut a.versions, b.versions);
    add_map(&mut a.bank_types, b.bank_types);
    add_map(&mut a.chunk_sets, b.chunk_sets);
    add_map(&mut a.hirc_type, b.hirc_type);
    for (k, v) in b.hirc_unique {
        a.hirc_unique.entry(k).or_default().extend(v);
    }
    a.hirc_errors.extend(b.hirc_errors);
    add_map(&mut a.action_types, b.action_types);
    add_map(&mut a.action_kinds, b.action_kinds);
    add_map(&mut a.source_plugins, b.source_plugins);
    add_map(&mut a.stream_types, b.stream_types);
    a.lang_specific_sources += b.lang_specific_sources;
    a.streamed_source_ids.extend(b.streamed_source_ids);
    a.memory_source_ids.extend(b.memory_source_ids);
    for (k, (n, ids)) in b.fx_plugins {
        let e = a.fx_plugins.entry(k).or_default();
        e.0 += n;
        e.1.extend(ids);
    }
    a.fx_media += b.fx_media;
    add_map(&mut a.device_plugins, b.device_plugins);
    add_map(&mut a.mixer_plugins, b.mixer_plugins);
    for (k, v) in b.fx_refs_by_owner {
        *a.fx_refs_by_owner.entry(k).or_default() += v;
    }
    add_map(&mut a.rtpc_types, b.rtpc_types);
    add_map(&mut a.rtpc_params, b.rtpc_params);
    a.rtpc_ids.extend(b.rtpc_ids);
    add_map(&mut a.container_modes, b.container_modes);
    add_map(&mut a.switch_group_types, b.switch_group_types);
    add_map(&mut a.music_track_types, b.music_track_types);
    add_map(&mut a.positioning, b.positioning);
    add_map(&mut a.props_used, b.props_used);
    a.ducking_buses += b.ducking_buses;
    a.hdr_buses += b.hdr_buses;
    a.aux_sends += b.aux_sends;
    a.reflections_aux += b.reflections_aux;
    a.diffraction += b.diffraction;
    add_map(&mut a.attenuation_curves, b.attenuation_curves);
    a.cone_attenuations += b.cone_attenuations;
    a.didx_media += b.didx_media;
    a.didx_bytes += b.didx_bytes;
    add_map(&mut a.didx_codecs, b.didx_codecs);
    a.didx_prefetch += b.didx_prefetch;
    a.dialogue_events += b.dialogue_events;
    a.stingers += b.stingers;
    a.transition_rules += b.transition_rules;
    a.state_props_states += b.state_props_states;
    a.max_tree_depth = a.max_tree_depth.max(b.max_tree_depth);
    a.markers += b.markers;
    if b.init.is_some() {
        a.init = b.init;
    }
}

#[allow(dead_code)]
fn _unused(_: HashMap<u32, u32>, _: BTreeSet<u32>) {}
