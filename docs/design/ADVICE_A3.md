# Advice for A3: Wwise 2022.1.4 mixing semantics, for parity

Status: advisory memo, 2026-10-05. Read-only research on `AlanWake2.exe` (statically linked
Wwise 2022.1.4) with `tools/ghidra_decompile.sh`, `tools/fndb.sh` and the exe's data tables.

- **[V]** means verified in the decompiled or disassembled code at the cited VA.
- **[A]** means assumed, from Wwise documentation or general knowledge. Confirm these with the A2
  offline harness before relying on them for a −90 dB target.

AK is built without RTTI here, so almost every VA below is unnamed in fndb. The names in
`backticks` are my identifications, made from structure and constants.

Speaker bits in Wwise channel masks (`AkChannelConfig`: `uNumChannels:8 | eConfigType:4 |
uChannelMask:20`):

| Speaker | Bit |
|---|---|
| FL / FR / FC | 0x1 / 0x2 / 0x4 |
| LFE | 0x8 |
| BL / BR | 0x10 / 0x20 |
| BC | 0x100 |
| SL / SR | 0x200 / 0x400 |
| TOP | 0x800 |
| HFL / HFC / HFR | 0x1000 / 0x2000 / 0x4000 |
| HBL / HBC / HBR | 0x8000 / 0x10000 / 0x20000 |

Config types: 0 = anonymous, 1 = standard, 2 = ambisonic. **LFE is always the last channel** in
the AK buffer order. Matrices are `[in_channel][out_channel]` with the output row stride padded
to a multiple of 8 floats (`(nOut+7)&~7`). Mirror that layout if you copy kernels.

---

## 0. Two global facts that affect every answer

### 0.1 dB → linear is `AkMath::FastPow10`, not `powf` [V]
Seen inline at 0x1415d8270, 0x141646d50, 0x1416389a0, 0x141646770, 0x1415f2db0, 0x141603110,
0x14160bba0 and about 30 others (search `out/audio/ak_consts.txt` for `27866352`).
Disassembly (0x1415d85a3 ff.):
```c
float dBToLin(float dB) {            // exact op order; SSE scalar, no FMA
    float x = dB * 0.05f;            // mulss
    if (!(x >= -37.0f)) return 0.0f; // comiss; < -37 (i.e. < -740 dB) => 0
    float t = x * 27866352.0f;       // mulss  (2^23 * log2(10))
    t = t + 1065353216.0f;           // addss  (127<<23, exact in float) -> rounded to float!
    int32_t i = (int32_t)(int64_t)t; // cvttss2si (truncate)
    float m = bitcast<float>((i & 0x007FFFFF) + 0x3F800000);   // mantissa in [1,2)
    float e = bitcast<float>( i & 0xFF800000);                  // 2^exponent
    return ((m * 0.32518977f + 0.020805772f) * m + 0.65304345f) * e;
}
```
- The float add before truncation keeps only 24 significant bits of `t`.
- The polynomial has a relative error of about 1e-3 (≈0.01 dB) against `powf`.
- **Using `powf(10, dB/20)` will miss −90 dB on any non-0 dB gain.** Port this function
  bit-exactly.
- `FastLinTodB` also exists (constants `1.15811861, 127, 6.02060032, 20`, e.g. 0x1415ada30 and
  0x1415d89a0, with a −96.3 dB floor elsewhere). I haven't decoded it; it is used for threshold
  and metering decisions, not the gain path.

### 0.2 The mixer kernels are dispatched by SIMD level, and the AVX2 kernels use FMA [V]
- `AkMixer::*` entry stubs (0x14163ccd0..0x14163d1f0) branch on the global `0x143931f80`. A value
  of 1 selects the AVX2 kernels at 0x14165f…/0x14166…; otherwise the SSE kernels at
  0x141656…/0x141659… run.
- The init at 0x14163cbd0 sets the global to 1 if
  `AkRuntimeEnvironmentMgr::Instance()->GetSIMDSupport(0x80)` reports AVX2.
- The SSE ramp kernel 0x141658fc0 has **no FMA** (separate `mulps`/`addps`, `divss` for 1/N).
  The AVX2 ramp kernel `sub_141660020` → 0x141661610 uses `vfmadd231ss`/`ps` (97 FMAs in that
  range), so its rounding differs.
- **For bit-exact references, force one path in the original.** In the hybrid loader, write
  `0` to `0x143931f80` after `AK::SoundEngine::Init`, or mask AVX2 in our CPUID/
  `IsProcessorFeaturePresent`. Then mirror the SSE kernel semantics in Rust (no `mul_add`, no
  auto-FMA: build without `target-feature=+fma` for this code, or use explicit separate ops).

---

## 1. 2D panning gain law

### 1.1 Dispatcher: `CAkSpeakerPan::GetSpeakerVolumes2DPan` @ 0x1415e1400 → 0x1415e1730 [V]
- 0x1415e1400 switches on the input and output config types:
  - std→std: 0x1415e1730.
  - ambisonic→std: decoder 0x1415df500.
  - std→ambisonic: encoder via 0x1415dd640 (SN3D/N3D constants).
  - anonymous: identity diagonal.
- 0x1415e1730 switches on the **speaker panning type** byte (Wwise `AkSpeakerPanningType`
  [A for the names]: 0 = DirectSpeakerAssignment, 1 = BalanceFadeHeight, 2 = SteeringPanner):
  - **type 0, or mono output** → no positional panning ("direct assignment", §4):
    - **Mono → stereo: L = R = 0.70710677 (−3.01 dB)**, a hard-coded constant [V].
    - Mono → output with centre (mask has FL|FR|FC): `L = R = sqrt((1−c)·0.5)`,
      `C = sqrt(c)`, with `c` = centre % / 100 (Positioning "Center %") [V].
    - Mono → mono: 1.0.
    - Multichannel input → `0x1415dbd30` (standard downmix tables, §4).
  - **type 1 (balance-fade)** → `0x1415da9d0` (§1.2), then LFE pass-through.
  - **type 2 (steering)** → 0x1415e4800 (not analysed).
  - Afterwards, always (0x1415e4300 and the tail of 0x1415e1730): the input LFE row is zeroed.
    **LFE → LFE = 1.0 only if the output has an LFE; otherwise the LFE is dropped.**

### 1.2 Balance-fade panner `0x1415da9d0` (x, y, centre%, inCfg, outCfg, out) [V]
Inputs: `x`, `y` ∈ [−1, 1] (the 2D panner's LR and FR positions) are mapped to
`fx = clamp((x+1)/2, 0, 1)` and `fy = clamp((y+1)/2, 0, 1)`.

**Mono input.** This is a **constant-power square-root law, not sin/cos**:
- **Stereo out:** `L = sqrt(1 − fx)`, `R = sqrt(fx)`. **Centre = sqrt(0.5) = −3.0103 dB.**
  L² + R² = 1 everywhere, but the curve is *not* the sin/cos law. At `fx = 0.25`, for example,
  L = 0.8660 versus cos(π/8) = 0.9239, a 0.57 dB difference, which is far above −90 dB.
- **3.0 out (FL FR FC)**, with `c` = centre % / 100:
  - If `fx > 0.5`: `pR = (fx−1)·c + fx` and `w = 1 − fx`.
  - Otherwise: `pR = (1−c)·fx` and `w = fx`.
  - `pC = 2c·w`.
  - Gains: `L = sqrt(1 − pR − pC)`, `R = sqrt(pR)`, `C = sqrt(pC)`.
- 5.0 (0x607), 4.0 (0x603, helper 0x1415e6e10), 6.0 (0x633, helper 0x1415e6f80) and 7.0 (0x637)
  all use the same power-split pattern with `fy` front/back crossfades. They are written out in
  the decompile; for 7.0 the power is split ⅓/⅓/⅓ across front, side and back via
  `(3−4fy)/3`, `(4fy−1)/3`.

**Multichannel input with balance-fade:**
1. Build an identity routing matrix in the *union* layout (in ∪ out).
2. Multiply each output column by the pan vector for that output layout. For stereo out the
   vector is `[sqrt(1−fx), sqrt(fx)]`; 3.0, 4.0 and 5.0 outputs use helpers 0x1415e6e10/6ea0/
   6f80/7080.
3. Scale everything by `sqrt(N_in)`, where N_in = the number of input channels excluding LFE.
4. Fold down to the real output with `0x1415e60f0` (not decoded).

Consequence for **stereo → stereo balance-fade:** at the centre each channel passes at
sqrt(0.5)·sqrt(2) = **1.0**. Hard right gives L→L = 0 and R→R = **√2 (+3 dB)** [V, from code;
worth confirming with a capture because +3 dB at the extreme is surprising].

### 1.3 Not 2D, but you will hit it (3D, stereo out) [V]
- With a stereo output and panning-rule flag `[ctx+0x30] == 1` (most likely
  `AkPanningRule_Headphones` [A]), `0x1415dc7d0` computes `s = sinf(azimuth)` and calls
  0x1415da860.
- 0x1415da860 does `g = (0.70710677 + 0.5·s − 0.20710677·s²)²`, then `powR += g` and
  `powL += 1 − g` (with g clamped at 1), and finally `sqrtps`. It is a quadratic approximation of
  sin(π/4·(1+s)) in the gain domain, and it is also inlined in the big spread/focus routine
  0x1415e1980 (9 KB).
- Otherwise the speaker rule uses VBAP (0x1415da8d0, per-config tables at 0x143815500).
- Remedy calls `SetSpeakerAngles` (0x1424ca150, 0x1424cbf20), so the VBAP angles are game-set.

---

## 2. Ramps: voice start, stop, seek and volume interpolation

### 2.1 Per-frame linear gain ramp in every mix call [V]
- **Signature:** `AkMixer::MixNinNChannels(in, out, AkRamp* {prevGain, nextGain}, prevMx,
  nextMx, 1/N, N)`. Callers: the bus/voice mix loops at 0x141648090 and 0x141648290. The kernel
  is the SSE 0x141658fc0, reached via 0x141657660 (MixOneInNChannels).
- **Per (in channel i, out channel o):**
  ```
  p = prevMx[i][o] * ramp.prev          // mulps
  n = nextMx[i][o] * ramp.next          // mulps
  d = n - p                             // subps
  step = d * (1.0f / N)                 // reciprocal computed once with divss, then mulps
  skip   if |d| <= FLT_EPSILON && |p| <= FLT_EPSILON      (1.1920929e-07)
  const  if |d| <= FLT_EPSILON:  out[s] += in[s] * p       (uses p, not n)
  ramp   else: gain for sample s (0-based) = p + step*(s+1)
  ```
  The ramp **starts at p+step and ends at ≈ n on the last sample** (not at p).
- **Exact SSE accumulation pattern** (matters at −90 dB):
  - Lane gains are initialised as `step*k + p` for k = 1..16.
  - In the 16-sample loop each lane gets `+= 16*step` per iteration.
  - The 4-sample tail loop continues from the first four lanes with `+= 4*step`.
  - The mix is `out = in*gain + out` (separate mul then add).
- **Ramp length N** is the buffer's frame count, i.e. one audio frame. In 0x141648090 the
  wrapper passes `1.0f / (float)AkAudioBuffer.uMaxFrames` (+0x12). The audio frame size is
  `AkInitSettings.uNumSamplesPerFrame`. `docs/AUDIO.md` §2.4 reads 0x200 = 512 [A]; the question
  assumed 1024. **Read it from the running engine** (`uMaxFrames` of a voice buffer, or
  `GetDefaultInitSettings` +0x18 as overridden by Remedy's init) rather than hard-coding it.
- **Spatial matrix on the first frame:** when a connection's cached previous matrix
  (`+0xb8` in the 0xc0-byte connection record) is NULL, it is allocated and filled **with the new
  matrix**, so prev == next. After each mix, `prevMx = nextMx` (memcpy) [V, 0x1416481e0..
  0x14164824d]. **There is no pan or matrix ramp on a voice's first frame.**

### 2.2 What this means for voice start, stop, seek and volume changes
- **Every gain change (volume, RTPC, state, bus volume, pan, attenuation) ramps linearly over
  exactly one frame**, starting on the frame where the new value is computed [V]. No other
  smoothing exists in the mixer.
- **Voice start** [A, partly V]: the matrix doesn't ramp on frame 1 (above). I didn't locate the
  `AkRamp.prev` initialisation for a brand-new voice. Wwise behaviour as I understand it: a voice
  started with 0 ms transition starts at full gain (prev = next, **no fade-in**). A voice started
  with a Play fade-in uses the transition curve, evaluated once per frame and linearly ramped
  inside the frame.
  - **Test (A2):** play a DC (all-ones) .wem with no transition and read samples 0..N of the
    capture. A flat first frame means no fade-in; a 0→1 line means a one-frame fade-in.
- **Voice stop with 0 ms transition** [A]: Wwise applies a minimal fade-out to avoid clicks.
  Because of the mixer, that is a one-frame linear ramp from the last gain to 0 (target 0 on the
  final frame), after which the voice is released. Verify with the same DC test (Stop at a known
  frame; expect a one-frame descending ramp).
- **Pause/Resume and Stop with a transition time** [A]: `AkCurveInterpolation` curves (Log3,
  Sine, Log1, InvSCurve, Linear, SCurve, Exp1, SineRecip, Exp3) are evaluated **per frame** to a
  linear ratio that multiplies the voice gain (it appears as the linear factor `ctx+0x100` in
  §3). They are linearly interpolated within the frame by §2.1. The curve formulas themselves need
  their own decompile; the fast-sin constants 0.225/0.405284703/1.27323949 appear in nearby code
  and may be part of the Sine curves [A].
- **Seek** [A]: I found no crossfade in the code I read. I believe a seek takes effect at the
  next frame boundary with no fade. Confirm with an A2 seek test.

---

## 3. How the volumes combine

[V] facts:
- **Each stage sums in dB, converts once with FastPow10, then multiplies by linear factors:**
  - 0x1415f2db0 / 0x141600880: `voiceGain = dBToLin(ctx[+0x88]) * ctx[+0x100]`. `ctx+0x88` is
    the accumulated dB volume of the voice context; `ctx+0x100` is a linear multiplier (fade /
    transition / mute ratios [A]).
  - 0x141603110: `dB = a[+0x24c] + a[+0xb4]`. The result is cached by that dB value, then
    `lin = dBToLin(dB) * (p[1] * (flag 0x20 ? p[0] : 1))`. That is two dB terms summed, plus
    linear factors.
  - 0x14160bba0: a bus's dB at `+0xec` is converted and passed separately to the bus mix setup
    (0x14164d400). **Bus volume is a separate stage**, applied on that bus's own mix and ramp,
    not folded into the voice.
  - 0x141646770 (an HDR/ducking-style pass): for each bus,
    `att_dB = bus[+0x14] − max(bus[+0x1d0], entry)`. Every voice on that bus gets
    `voice.lin[+0x188] *= dBToLin(att_dB)` and `voice.dB[+0x18c] += att_dB`, then
    `virtual = (voice.dB <= threshold @0x143815b3c)`.

  So **linear gain is a product of separately converted stages, while a parallel dB sum is kept
  for the volume-threshold and virtual-voice decision.** To match bit-for-bit you must reproduce
  the stage grouping and the multiplication order, not a single global dB sum.
- 0x141645fc0 (aux/send gains) initialises the accumulator to −4096 dB (`0xc5800000`), takes the
  max over matching send entries, and converts dB → linear the same way.

[A] (Wwise 2022 documented model; confirm which dB fields feed `ctx+0x88` by decompiling its
writer, for example the setters around 0x1415f2db0 and 0x1415d3d00):
- **Actor-mixer hierarchy:** Volume and Make-Up Gain are each **summed in dB** from the object
  up through its actor-mixer and container parents, together with RTPC, state and modulator
  offsets (also dB, additive). Distance attenuation (dB from curves) is added at the 3D stage.
- **Make-Up Gain** accumulates separately from Volume. It is applied to the voice but excluded
  from HDR and loudness-normalization computations; it still counts for the threshold. The two
  dB numbers end up in one voice gain. Check whether that happens as one sum (single FastPow10)
  or as two FastPow10 factors; with the fast exp the two give different bits.
- **Bus "Voice Volume"** (a bus property applied to each voice routed into the bus) is added to
  the voice's dB. **Bus Volume / Output Bus Volume** are applied to the mixed bus signal, in that
  bus's own `MixNinNChannels` ramp to its parent. Output Bus Volume is the per-voice send gain to
  the output bus.
- **Interpolation:** everything is evaluated once per audio frame and ramped linearly within the
  frame (§2.1). RTPC values themselves may additionally be slewed by "RTPC ramping" (537 params,
  `docs/AUDIO.md`), per frame and before the dB sum.

---

## 4. Standard downmix: 7.1.4 bus → stereo endpoint

Path [V]:
- Bus connection matrix (0x1415d6570, `ctx+0x6c == 0` meaning no 3D) → `GetSpeakerVolumes2DPan`
  0x1415e1400 with panner type `ctx+0x6d`.
- For direct assignment (type 0) with multichannel input → **0x1415dbd30**, which looks up a
  table per output layout indexed by the input speaker's bit position. LFE is removed from the
  mask first (`& 0xfffffff7`) and handled by 0x1415e4300.
- **No normalisation afterwards.** Rows are not rescaled to unit power or unit sum.

Stereo table @ **0x14268c5e0** (float[18][2], read from the exe) [V]:

| input | L | R |
|---|---|---|
| FL | 1 | 0 |
| FR | 0 | 1 |
| FC | 0.70710677 | 0.70710677 |
| LFE | — (dropped, see below) | — |
| BL | 0.70710677 | 0 |
| BR | 0 | 0.70710677 |
| BC | 0.70710677 | 0.70710677 |
| SL | 0.70710677 | 0 |
| SR | 0 | 0.70710677 |
| TOP | 0.70710677 | 0.70710677 |
| HFL | 0.70710677 | 0 |
| HFC | 0.5 | 0.5 |
| HFR | 0 | 0.70710677 |
| HBL | 0.5 | 0 |
| HBC | 0 | 0 |
| HBR | 0 | 0.5 |

So for 7.1.4 (FL FR FC LFE BL BR SL SR HFL HFR HBL HBR, LFE stored last):
```
L = FL + 0.70710677·(FC + BL + SL + HFL) + 0.5·HBL
R = FR + 0.70710677·(FC + BR + SR + HFR) + 0.5·HBR
LFE: dropped (row zeroed; LFE→LFE=1 only if the output has an LFE)   [V, 0x1415e4300]
```
- Other output tables: mono @0x14268c580, 3.0 @0x14268c670, 4.0 @0x14268c770, 5.0 @0x14268c890,
  6.0 (0x633) @0x14268ca00, 7.0 @0x14268cbb0.
- When the output also has height channels, matching heights are routed 1:1 by the tail of
  0x1415dbd30.
- The dump script is in this conversation's scratch dir (`rd.py <va> <cols> <rows>`); a
  one-liner using the PE section table reproduces it.
- These tables don't depend on the panning rule or the speaker angles [V: there is no such
  input to 0x1415dbd30 or 0x1415e1730].
- Caveat [A]: this is the *direct-assignment* downmix. If a bus in Remedy's data enables the
  balance-fade panner (`ctx+0x6d == 1`), the 7.1.4 → stereo fold goes through `0x1415da9d0` →
  `0x1415e60f0` instead, which I haven't decoded. Check the bus positioning flags in
  `Init.bnk`/`Global.bnk`.
- Also open: in 0x1415d6570, when `[conn+0x94] == 0` the matrix is filled with
  `1/sqrt(N_in)/N_out` (anonymous or standard outputs) or `1/sqrt(N_in)` on the diagonal. This
  looks like a "no positioning / pass-through" mode for some send type; identify what sets
  `+0x94` before trusting §4 for aux sends.

---

## 5. Suggested A2 parity tests

| Test | Expected (if this memo is right) |
|---|---|
| Mono DC, direct assignment, stereo out | L = R = 0.70710677 × voice gain |
| Mono DC, balance-fade x = −0.5 (fx = 0.25) | L = 0.8660254, R = 0.5 (sqrt law, not sin/cos) |
| Stereo DC, balance-fade hard right | L→L = 0, R→R = 1.4142 |
| Volume step 0 → −6 dB mid-play | one-frame ramp, first sample `p + (n−p)/N`, gain `dBToLin(−6)` = FastPow10, not 0.501187 |
| Play, 0 ms transition | flat first frame (no fade-in) [A] |
| Stop, 0 ms transition | one-frame linear ramp to 0 [A] |
| 7.1.4 impulse per channel → stereo | the table in §4, LFE silent |

Run every reference with the AK SIMD level forced to SSE (§0.2) to get deterministic bits.
