//! Pure-Rust Embedded ECMA-335 CLR / CIL Runtime.
//!
//! Provides a zero-external-dependency `.NET` and `Mono` execution engine for macOS and iOS:
//! - [`metadata`]: PE32/PE32+ `.NET` assembly & ECMA-335 metadata table / CIL body parser.
//! - [`vm`]: ABI-compatible managed heap (`MonoObject`/`MonoString`/`MonoArray`/`GCHandle`),
//!   CIL bytecode interpreter, BCL intrinsics, P/Invoke (`ImplMap`), and Mono internal calls.
//! - 256 static `extern "win64"` native-to-managed delegate trampolines registered with
//!   `crate::register_native_va`.
//! - Embedded context/host engines for `hostfxr.dll`, `coreclr.dll`, `mscoree.dll`, and
//!   `mono-2.0-bdwgc.dll` when no host dylib is installed.

pub mod metadata;
pub mod vm;

use crate::types::{E_FAIL, E_INVALIDARG, S_OK};
use metadata::SigType;
use std::collections::HashMap;
use std::ffi::{c_char, c_void};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use vm::{lock_vm, CilVal, VmError, MONO_ARRAY_DATA_OFFSET, MONO_OBJECT_HEADER_SIZE};

pub const COR_E_FILENOTFOUND: i32 = 0x8007_0002u32 as i32;
pub const COR_E_BADIMAGEFORMAT: i32 = 0x8007_000Bu32 as i32;
pub const COR_E_TYPELOAD: i32 = 0x8013_1522u32 as i32;
pub const COR_E_MISSINGMETHOD: i32 = 0x8013_1513u32 as i32;

pub const HOSTFXR_STATUS_SUCCESS: i32 = 0;
pub const HOSTFXR_INVALID_ARG: i32 = 0x8000_8081u32 as i32;
pub const HOSTFXR_INVALID_CONFIG_FILE: i32 = 0x8000_8092u32 as i32;
pub const HOSTFXR_APP_ARG_NOT_RUNNABLE: i32 = 0x8000_8093u32 as i32;
pub const HOSTFXR_PROPERTY_NOT_FOUND: i32 = 0x8000_80A1u32 as i32;

pub const MONO_IMAGE_OK: i32 = 0;
pub const MONO_IMAGE_ERROR_ERRNO: i32 = 1;
pub const MONO_IMAGE_IMAGE_INVALID: i32 = 3;

pub fn vm_err_to_hresult(e: &VmError) -> i32 {
    match e {
        VmError::FileNotFound(_) => COR_E_FILENOTFOUND,
        VmError::BadImage(_) => COR_E_BADIMAGEFORMAT,
        VmError::MethodNotFound(_) => COR_E_MISSINGMETHOD,
        VmError::Unsupported(_) | VmError::ManagedException { .. } => E_FAIL,
    }
}

/// Translate a Windows path (if it has `\` or a drive letter) into a host path;
/// preserve already-host Unix paths verbatim.
pub fn translate_win_path(p: &str) -> PathBuf {
    if p.is_empty() {
        return PathBuf::new();
    }
    let looks_win = p.contains('\\') || (p.len() >= 2 && p.as_bytes()[1] == b':');
    if looks_win {
        crate::path::to_host(p).unwrap_or_else(|| PathBuf::from(p.replace('\\', "/")))
    } else {
        PathBuf::from(p)
    }
}

pub fn embedded_runtime_dir_and_version() -> (PathBuf, String) {
    let dir = crate::path::default_translator_root()
        .join("Windows/Microsoft.NET/Framework64/v4.0.30319");
    (dir, "v4.0.30319".to_string())
}

// ---------------------------------------------------------------------------
// 256 Static `extern "win64"` Native-to-Managed Delegate Trampolines
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DelegateCallMode {
    DirectAnsi,
    DirectWide,
    ComponentEntryPoint,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DelegateBinding {
    pub asm_id: usize,
    pub method_id: usize,
    pub target_obj: usize,
    pub mode: DelegateCallMode,
}

type TrampolineFn =
    unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64;

const MAX_TRAMPOLINES: usize = 256;

fn trampoline_slots() -> &'static Mutex<[Option<DelegateBinding>; MAX_TRAMPOLINES]> {
    static SLOTS: OnceLock<Mutex<[Option<DelegateBinding>; MAX_TRAMPOLINES]>> = OnceLock::new();
    SLOTS.get_or_init(|| Mutex::new([None; MAX_TRAMPOLINES]))
}

unsafe fn dispatch_trampoline(slot: usize, raw: [u64; 8]) -> u64 {
    let binding = {
        let slots = trampoline_slots().lock().unwrap_or_else(|e| e.into_inner());
        match slots.get(slot).and_then(|s| *s) {
            Some(b) => b,
            None => return 0,
        }
    };

    let (cil_args, ret_sig) = {
        let mut vm = lock_vm();
        let Some(m) = vm.methods.get(binding.method_id) else {
            return 0;
        };
        let sig = m.sig.clone();
        let mut args = Vec::with_capacity(sig.params.len() + 1);
        if binding.target_obj != 0 {
            args.push(CilVal::Obj(binding.target_obj));
        }
        match binding.mode {
            DelegateCallMode::ComponentEntryPoint => {
                // `int Entry(IntPtr args, int sizeBytes)` (or parameterless / 1-arg fallback)
                for (i, pty) in sig.params.iter().enumerate() {
                    let v = raw.get(i).copied().unwrap_or(0);
                    args.push(marshal_win64_arg_to_cil(&mut vm, pty, v, true));
                }
            }
            DelegateCallMode::DirectWide => {
                for (i, pty) in sig.params.iter().enumerate() {
                    let v = raw.get(i).copied().unwrap_or(0);
                    args.push(marshal_win64_arg_to_cil(&mut vm, pty, v, true));
                }
            }
            DelegateCallMode::DirectAnsi => {
                for (i, pty) in sig.params.iter().enumerate() {
                    let v = raw.get(i).copied().unwrap_or(0);
                    args.push(marshal_win64_arg_to_cil(&mut vm, pty, v, false));
                }
            }
        }
        (args, sig.ret)
    };

    match vm::execute_method(binding.method_id, cil_args) {
        Ok(Some(val)) => match ret_sig {
            SigType::Void => 0,
            SigType::R4 => (val.as_f64() as f32).to_bits() as u64,
            SigType::R8 => val.as_f64().to_bits(),
            SigType::I4 | SigType::U4 | SigType::Boolean | SigType::Char | SigType::I1 | SigType::U1 | SigType::I2 | SigType::U2 => {
                val.as_i32() as u32 as u64
            }
            _ => val.as_u64_bits(),
        },
        Ok(None) => 0,
        Err(e) => {
            log::warn!("clr delegate trampoline[{slot}] failed: {e}");
            0
        }
    }
}

unsafe fn marshal_win64_arg_to_cil(
    vm: &mut vm::ClrVm,
    pty: &SigType,
    raw: u64,
    wide_strings: bool,
) -> CilVal {
    match pty {
        SigType::Boolean => CilVal::I32(i32::from((raw as u32) != 0)),
        SigType::I1 => CilVal::I32(raw as i8 as i32),
        SigType::U1 => CilVal::I32(raw as u8 as i32),
        SigType::I2 => CilVal::I32(raw as i16 as i32),
        SigType::U2 | SigType::Char => CilVal::I32(raw as u16 as i32),
        SigType::I4 | SigType::U4 => CilVal::I32(raw as i32),
        SigType::I8 | SigType::U8 => CilVal::I64(raw as i64),
        SigType::R4 => CilVal::F64(f32::from_bits(raw as u32) as f64),
        SigType::R8 => CilVal::F64(f64::from_bits(raw)),
        SigType::IntPtr | SigType::UIntPtr | SigType::Ptr(_) | SigType::FnPtr => {
            CilVal::NativeInt(raw as isize)
        }
        SigType::String => {
            if raw == 0 {
                CilVal::Obj(0)
            } else if vm.heap.objects.contains_key(&(raw as usize)) {
                CilVal::Obj(raw as usize)
            } else {
                let s = if wide_strings {
                    crate::strings::from_wide(raw as *const u16)
                } else {
                    crate::strings::from_ansi(raw as *const u8)
                };
                CilVal::Obj(vm.alloc_string(&s))
            }
        }
        _ => CilVal::Obj(raw as usize),
    }
}

macro_rules! define_trampolines {
    ($($idx:literal => $name:ident),* $(,)?) => {
        $(
            unsafe extern "win64" fn $name(
                a0: u64, a1: u64, a2: u64, a3: u64,
                a4: u64, a5: u64, a6: u64, a7: u64,
            ) -> u64 {
                dispatch_trampoline($idx, [a0, a1, a2, a3, a4, a5, a6, a7])
            }
        )*

        static TRAMPOLINE_TABLE: [TrampolineFn; 256] = [
            $($name),*
        ];
    };
}

define_trampolines! {
    0 => tramp_000, 1 => tramp_001, 2 => tramp_002, 3 => tramp_003,
    4 => tramp_004, 5 => tramp_005, 6 => tramp_006, 7 => tramp_007,
    8 => tramp_008, 9 => tramp_009, 10 => tramp_010, 11 => tramp_011,
    12 => tramp_012, 13 => tramp_013, 14 => tramp_014, 15 => tramp_015,
    16 => tramp_016, 17 => tramp_017, 18 => tramp_018, 19 => tramp_019,
    20 => tramp_020, 21 => tramp_021, 22 => tramp_022, 23 => tramp_023,
    24 => tramp_024, 25 => tramp_025, 26 => tramp_026, 27 => tramp_027,
    28 => tramp_028, 29 => tramp_029, 30 => tramp_030, 31 => tramp_031,
    32 => tramp_032, 33 => tramp_033, 34 => tramp_034, 35 => tramp_035,
    36 => tramp_036, 37 => tramp_037, 38 => tramp_038, 39 => tramp_039,
    40 => tramp_040, 41 => tramp_041, 42 => tramp_042, 43 => tramp_043,
    44 => tramp_044, 45 => tramp_045, 46 => tramp_046, 47 => tramp_047,
    48 => tramp_048, 49 => tramp_049, 50 => tramp_050, 51 => tramp_051,
    52 => tramp_052, 53 => tramp_053, 54 => tramp_054, 55 => tramp_055,
    56 => tramp_056, 57 => tramp_057, 58 => tramp_058, 59 => tramp_059,
    60 => tramp_060, 61 => tramp_061, 62 => tramp_062, 63 => tramp_063,
    64 => tramp_064, 65 => tramp_065, 66 => tramp_066, 67 => tramp_067,
    68 => tramp_068, 69 => tramp_069, 70 => tramp_070, 71 => tramp_071,
    72 => tramp_072, 73 => tramp_073, 74 => tramp_074, 75 => tramp_075,
    76 => tramp_076, 77 => tramp_077, 78 => tramp_078, 79 => tramp_079,
    80 => tramp_080, 81 => tramp_081, 82 => tramp_082, 83 => tramp_083,
    84 => tramp_084, 85 => tramp_085, 86 => tramp_086, 87 => tramp_087,
    88 => tramp_088, 89 => tramp_089, 90 => tramp_090, 91 => tramp_091,
    92 => tramp_092, 93 => tramp_093, 94 => tramp_094, 95 => tramp_095,
    96 => tramp_096, 97 => tramp_097, 98 => tramp_098, 99 => tramp_099,
    100 => tramp_100, 101 => tramp_101, 102 => tramp_102, 103 => tramp_103,
    104 => tramp_104, 105 => tramp_105, 106 => tramp_106, 107 => tramp_107,
    108 => tramp_108, 109 => tramp_109, 110 => tramp_110, 111 => tramp_111,
    112 => tramp_112, 113 => tramp_113, 114 => tramp_114, 115 => tramp_115,
    116 => tramp_116, 117 => tramp_117, 118 => tramp_118, 119 => tramp_119,
    120 => tramp_120, 121 => tramp_121, 122 => tramp_122, 123 => tramp_123,
    124 => tramp_124, 125 => tramp_125, 126 => tramp_126, 127 => tramp_127,
    128 => tramp_128, 129 => tramp_129, 130 => tramp_130, 131 => tramp_131,
    132 => tramp_132, 133 => tramp_133, 134 => tramp_134, 135 => tramp_135,
    136 => tramp_136, 137 => tramp_137, 138 => tramp_138, 139 => tramp_139,
    140 => tramp_140, 141 => tramp_141, 142 => tramp_142, 143 => tramp_143,
    144 => tramp_144, 145 => tramp_145, 146 => tramp_146, 147 => tramp_147,
    148 => tramp_148, 149 => tramp_149, 150 => tramp_150, 151 => tramp_151,
    152 => tramp_152, 153 => tramp_153, 154 => tramp_154, 155 => tramp_155,
    156 => tramp_156, 157 => tramp_157, 158 => tramp_158, 159 => tramp_159,
    160 => tramp_160, 161 => tramp_161, 162 => tramp_162, 163 => tramp_163,
    164 => tramp_164, 165 => tramp_165, 166 => tramp_166, 167 => tramp_167,
    168 => tramp_168, 169 => tramp_169, 170 => tramp_170, 171 => tramp_171,
    172 => tramp_172, 173 => tramp_173, 174 => tramp_174, 175 => tramp_175,
    176 => tramp_176, 177 => tramp_177, 178 => tramp_178, 179 => tramp_179,
    180 => tramp_180, 181 => tramp_181, 182 => tramp_182, 183 => tramp_183,
    184 => tramp_184, 185 => tramp_185, 186 => tramp_186, 187 => tramp_187,
    188 => tramp_188, 189 => tramp_189, 190 => tramp_190, 191 => tramp_191,
    192 => tramp_192, 193 => tramp_193, 194 => tramp_194, 195 => tramp_195,
    196 => tramp_196, 197 => tramp_197, 198 => tramp_198, 199 => tramp_199,
    200 => tramp_200, 201 => tramp_201, 202 => tramp_202, 203 => tramp_203,
    204 => tramp_204, 205 => tramp_205, 206 => tramp_206, 207 => tramp_207,
    208 => tramp_208, 209 => tramp_209, 210 => tramp_210, 211 => tramp_211,
    212 => tramp_212, 213 => tramp_213, 214 => tramp_214, 215 => tramp_215,
    216 => tramp_216, 217 => tramp_217, 218 => tramp_218, 219 => tramp_219,
    220 => tramp_220, 221 => tramp_221, 222 => tramp_222, 223 => tramp_223,
    224 => tramp_224, 225 => tramp_225, 226 => tramp_226, 227 => tramp_227,
    228 => tramp_228, 229 => tramp_229, 230 => tramp_230, 231 => tramp_231,
    232 => tramp_232, 233 => tramp_233, 234 => tramp_234, 235 => tramp_235,
    236 => tramp_236, 237 => tramp_237, 238 => tramp_238, 239 => tramp_239,
    240 => tramp_240, 241 => tramp_241, 242 => tramp_242, 243 => tramp_243,
    244 => tramp_244, 245 => tramp_245, 246 => tramp_246, 247 => tramp_247,
    248 => tramp_248, 249 => tramp_249, 250 => tramp_250, 251 => tramp_251,
    252 => tramp_252, 253 => tramp_253, 254 => tramp_254, 255 => tramp_255,
}

/// Allocate (or reuse) a static `extern "win64"` native-to-managed trampoline and
/// register it with `crate::register_native_va`.
pub fn alloc_delegate_trampoline(binding: DelegateBinding) -> Result<usize, VmError> {
    let mut slots = trampoline_slots().lock().unwrap_or_else(|e| e.into_inner());
    for (i, s) in slots.iter().enumerate() {
        if *s == Some(binding) {
            let ptr = TRAMPOLINE_TABLE[i] as *const () as usize;
            crate::register_native_va(ptr as u64, ptr);
            return Ok(ptr);
        }
    }
    for (i, s) in slots.iter_mut().enumerate() {
        if s.is_none() {
            *s = Some(binding);
            let ptr = TRAMPOLINE_TABLE[i] as *const () as usize;
            crate::register_native_va(ptr as u64, ptr);
            return Ok(ptr);
        }
    }
    Err(VmError::Unsupported(
        "exhausted 256 static CLR delegate trampoline slots".into(),
    ))
}

// ---------------------------------------------------------------------------
// Minimal Pure-Rust JSON Parser for `.runtimeconfig.json`
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum JsonVal {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<JsonVal>),
    Obj(Vec<(String, JsonVal)>),
}

impl JsonVal {
    fn as_obj(&self) -> Option<&[(String, JsonVal)]> {
        match self {
            JsonVal::Obj(m) => Some(m),
            _ => None,
        }
    }
    fn get(&self, key: &str) -> Option<&JsonVal> {
        self.as_obj()?
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }
    fn as_str(&self) -> Option<&str> {
        match self {
            JsonVal::Str(s) => Some(s),
            _ => None,
        }
    }
    fn to_prop_string(&self) -> Option<String> {
        match self {
            JsonVal::Str(s) => Some(s.clone()),
            JsonVal::Bool(b) => Some(if *b { "true" } else { "false" }.to_string()),
            JsonVal::Num(n) => Some(n.clone()),
            _ => None,
        }
    }
}

struct JsonParser<'a> {
    b: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    fn skip_ws(&mut self) {
        while self.pos < self.b.len() {
            match self.b[self.pos] {
                b' ' | b'\t' | b'\r' | b'\n' => self.pos += 1,
                _ => break,
            }
        }
    }
    fn peek(&mut self) -> Option<u8> {
        self.skip_ws();
        self.b.get(self.pos).copied()
    }
    fn parse_val(&mut self) -> Result<JsonVal, ()> {
        match self.peek().ok_or(())? {
            b'{' => self.parse_obj(),
            b'[' => self.parse_arr(),
            b'"' => self.parse_str().map(JsonVal::Str),
            b't' => {
                if self.b[self.pos..].starts_with(b"true") {
                    self.pos += 4;
                    Ok(JsonVal::Bool(true))
                } else {
                    Err(())
                }
            }
            b'f' => {
                if self.b[self.pos..].starts_with(b"false") {
                    self.pos += 5;
                    Ok(JsonVal::Bool(false))
                } else {
                    Err(())
                }
            }
            b'n' => {
                if self.b[self.pos..].starts_with(b"null") {
                    self.pos += 4;
                    Ok(JsonVal::Null)
                } else {
                    Err(())
                }
            }
            b'-' | b'0'..=b'9' => {
                let start = self.pos;
                while self.pos < self.b.len()
                    && matches!(
                        self.b[self.pos],
                        b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9'
                    )
                {
                    self.pos += 1;
                }
                Ok(JsonVal::Num(
                    String::from_utf8_lossy(&self.b[start..self.pos]).into_owned(),
                ))
            }
            _ => Err(()),
        }
    }
    fn parse_str(&mut self) -> Result<String, ()> {
        self.skip_ws();
        if self.b.get(self.pos) != Some(&b'"') {
            return Err(());
        }
        self.pos += 1;
        let mut out = String::new();
        while self.pos < self.b.len() {
            let c = self.b[self.pos];
            self.pos += 1;
            if c == b'"' {
                return Ok(out);
            }
            if c == b'\\' {
                let esc = *self.b.get(self.pos).ok_or(())?;
                self.pos += 1;
                match esc {
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'/' => out.push('/'),
                    b'b' => out.push('\x08'),
                    b'f' => out.push('\x0C'),
                    b'n' => out.push('\n'),
                    b'r' => out.push('\r'),
                    b't' => out.push('\t'),
                    b'u' => {
                        let hex = self.b.get(self.pos..self.pos + 4).ok_or(())?;
                        self.pos += 4;
                        let s = std::str::from_utf8(hex).map_err(|_| ())?;
                        let code = u32::from_str_radix(s, 16).map_err(|_| ())?;
                        if let Some(ch) = char::from_u32(code) {
                            out.push(ch);
                        }
                    }
                    _ => out.push(esc as char),
                }
            } else {
                out.push(c as char);
            }
        }
        Err(())
    }
    fn parse_arr(&mut self) -> Result<JsonVal, ()> {
        self.pos += 1; // '['
        let mut items = Vec::new();
        if self.peek() == Some(b']') {
            self.pos += 1;
            return Ok(JsonVal::Arr(items));
        }
        loop {
            items.push(self.parse_val()?);
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    return Ok(JsonVal::Arr(items));
                }
                _ => return Err(()),
            }
        }
    }
    fn parse_obj(&mut self) -> Result<JsonVal, ()> {
        self.pos += 1; // '{'
        let mut entries = Vec::new();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Ok(JsonVal::Obj(entries));
        }
        loop {
            let k = self.parse_str()?;
            if self.peek() != Some(b':') {
                return Err(());
            }
            self.pos += 1;
            let v = self.parse_val()?;
            entries.push((k, v));
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(JsonVal::Obj(entries));
                }
                _ => return Err(()),
            }
        }
    }
}

fn parse_runtimeconfig_json(path: &Path) -> Result<HashMap<String, String>, i32> {
    let raw = std::fs::read_to_string(path).map_err(|_| HOSTFXR_INVALID_CONFIG_FILE)?;
    let trimmed = raw.trim_start_matches('\u{FEFF}');
    let mut parser = JsonParser {
        b: trimmed.as_bytes(),
        pos: 0,
    };
    let root = parser.parse_val().map_err(|_| HOSTFXR_INVALID_CONFIG_FILE)?;
    if root.as_obj().is_none() {
        return Err(HOSTFXR_INVALID_CONFIG_FILE);
    }
    let mut props = HashMap::new();
    if let Some(opts) = root.get("runtimeOptions") {
        if let Some(tfm) = opts.get("tfm").and_then(|v| v.as_str()) {
            props.insert("TARGET_FRAMEWORK_MONIKER".to_string(), tfm.to_string());
        }
        if let Some(fw) = opts.get("framework") {
            if let Some(n) = fw.get("name").and_then(|v| v.as_str()) {
                props.insert("FX_PRODUCT_NAME".to_string(), n.to_string());
            }
            if let Some(ver) = fw.get("version").and_then(|v| v.as_str()) {
                props.insert("FX_PRODUCT_VERSION".to_string(), ver.to_string());
            }
        }
        if let Some(cp) = opts.get("configProperties").and_then(|v| v.as_obj()) {
            for (k, v) in cp {
                if let Some(sv) = v.to_prop_string() {
                    props.insert(k.clone(), sv);
                }
            }
        }
        if let Some(JsonVal::Arr(arr)) = opts.get("additionalProbingPaths") {
            let mut vm = lock_vm();
            for item in arr {
                if let Some(p) = item.as_str() {
                    let pb = translate_win_path(p);
                    if !vm.search_dirs.contains(&pb) {
                        vm.search_dirs.push(pb);
                    }
                }
            }
        }
    }
    Ok(props)
}

// ---------------------------------------------------------------------------
// Embedded HostFXR Context Store
// ---------------------------------------------------------------------------

const HOSTFXR_HANDLE_BASE: usize = 0x4846_5800;

struct HostfxrContext {
    app_assembly: Option<PathBuf>,
    app_args: Vec<String>,
    properties: HashMap<String, String>,
}

struct HostfxrState {
    next_id: usize,
    contexts: HashMap<usize, HostfxrContext>,
}

fn hostfxr_state() -> &'static Mutex<HostfxrState> {
    static ST: OnceLock<Mutex<HostfxrState>> = OnceLock::new();
    ST.get_or_init(|| {
        Mutex::new(HostfxrState {
            next_id: 1,
            contexts: HashMap::new(),
        })
    })
}

pub fn hostfxr_is_embedded_handle(handle: usize) -> bool {
    if handle == 0 {
        return false;
    }
    hostfxr_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .contexts
        .contains_key(&handle)
}

pub fn hostfxr_init_for_config(
    cfg_path: &Path,
    _host_path: Option<&Path>,
    _dotnet_root: Option<&Path>,
) -> Result<usize, i32> {
    if !cfg_path.is_file() {
        return Err(HOSTFXR_INVALID_CONFIG_FILE);
    }
    let mut props = parse_runtimeconfig_json(cfg_path)?;
    let base_dir = cfg_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    props
        .entry("APP_CONTEXT_BASE_DIRECTORY".to_string())
        .or_insert_with(|| base_dir.to_string_lossy().into_owned());

    // Derive companion `<App>.dll` from `<App>.runtimeconfig.json` if present.
    let app_assembly = cfg_path.file_name().and_then(|f| {
        let s = f.to_string_lossy();
        let stem = s.strip_suffix(".runtimeconfig.json")?;
        let dll = base_dir.join(format!("{stem}.dll"));
        if dll.is_file() {
            Some(dll)
        } else {
            let exe = base_dir.join(format!("{stem}.exe"));
            exe.is_file().then_some(exe)
        }
    });

    {
        let mut vm = lock_vm();
        if !vm.search_dirs.contains(&base_dir) {
            vm.search_dirs.push(base_dir.clone());
        }
    }

    let mut st = hostfxr_state().lock().unwrap_or_else(|e| e.into_inner());
    let handle = HOSTFXR_HANDLE_BASE + st.next_id * 8;
    st.next_id += 1;
    st.contexts.insert(
        handle,
        HostfxrContext {
            app_assembly,
            app_args: Vec::new(),
            properties: props,
        },
    );
    Ok(handle)
}

pub fn hostfxr_init_for_cmdline(
    args: &[String],
    _host_path: Option<&Path>,
    _dotnet_root: Option<&Path>,
) -> Result<usize, i32> {
    let Some(first) = args.iter().find(|a| !a.is_empty() && !a.starts_with('-')) else {
        return Err(HOSTFXR_INVALID_ARG);
    };
    let app_path = translate_win_path(first);
    if !app_path.is_file() {
        return Err(HOSTFXR_APP_ARG_NOT_RUNNABLE);
    }
    // Validate that the target is actually a valid .NET PE assembly
    let bytes = std::fs::read(&app_path).map_err(|_| HOSTFXR_APP_ARG_NOT_RUNNABLE)?;
    if metadata::AssemblyImage::parse(&bytes).is_err() {
        return Err(HOSTFXR_APP_ARG_NOT_RUNNABLE);
    }

    let base_dir = app_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let mut props = HashMap::new();
    if let Some(stem) = app_path.file_stem() {
        let cfg = base_dir.join(format!("{}.runtimeconfig.json", stem.to_string_lossy()));
        if cfg.is_file() {
            if let Ok(parsed) = parse_runtimeconfig_json(&cfg) {
                props = parsed;
            }
        }
    }
    props
        .entry("APP_CONTEXT_BASE_DIRECTORY".to_string())
        .or_insert_with(|| base_dir.to_string_lossy().into_owned());
    {
        let mut vm = lock_vm();
        if !vm.search_dirs.contains(&base_dir) {
            vm.search_dirs.push(base_dir);
        }
    }
    let app_args: Vec<String> = args.iter().skip(1).cloned().collect();

    let mut st = hostfxr_state().lock().unwrap_or_else(|e| e.into_inner());
    let handle = HOSTFXR_HANDLE_BASE + st.next_id * 8;
    st.next_id += 1;
    st.contexts.insert(
        handle,
        HostfxrContext {
            app_assembly: Some(app_path),
            app_args,
            properties: props,
        },
    );
    Ok(handle)
}

pub fn hostfxr_get_property(handle: usize, key: &str) -> Result<String, i32> {
    let st = hostfxr_state().lock().unwrap_or_else(|e| e.into_inner());
    let ctx = st.contexts.get(&handle).ok_or(HOSTFXR_INVALID_ARG)?;
    ctx.properties
        .get(key)
        .cloned()
        .ok_or(HOSTFXR_PROPERTY_NOT_FOUND)
}

pub fn hostfxr_set_property(handle: usize, key: &str, val: Option<&str>) -> i32 {
    let mut st = hostfxr_state().lock().unwrap_or_else(|e| e.into_inner());
    let Some(ctx) = st.contexts.get_mut(&handle) else {
        return HOSTFXR_INVALID_ARG;
    };
    if let Some(v) = val {
        ctx.properties.insert(key.to_string(), v.to_string());
    } else {
        ctx.properties.remove(key);
    }
    HOSTFXR_STATUS_SUCCESS
}

pub fn hostfxr_get_properties(handle: usize) -> Result<Vec<(String, String)>, i32> {
    let st = hostfxr_state().lock().unwrap_or_else(|e| e.into_inner());
    let ctx = st.contexts.get(&handle).ok_or(HOSTFXR_INVALID_ARG)?;
    let mut pairs: Vec<(String, String)> = ctx
        .properties
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(pairs)
}

pub fn hostfxr_run_app(handle: usize) -> i32 {
    let (asm_path, args) = {
        let st = hostfxr_state().lock().unwrap_or_else(|e| e.into_inner());
        let Some(ctx) = st.contexts.get(&handle) else {
            return HOSTFXR_INVALID_ARG;
        };
        let Some(p) = ctx.app_assembly.clone() else {
            return HOSTFXR_APP_ARG_NOT_RUNNABLE;
        };
        (p, ctx.app_args.clone())
    };
    match execute_entry_point_file(&asm_path, &args) {
        Ok(code) => code,
        Err(hr) => hr,
    }
}

pub fn hostfxr_close(handle: usize) -> i32 {
    let mut st = hostfxr_state().lock().unwrap_or_else(|e| e.into_inner());
    if st.contexts.remove(&handle).is_some() {
        HOSTFXR_STATUS_SUCCESS
    } else {
        HOSTFXR_INVALID_ARG
    }
}

pub fn hostfxr_load_assembly(asm_path: &Path) -> i32 {
    let mut vm = lock_vm();
    match vm.load_assembly_file(asm_path) {
        Ok(_) => HOSTFXR_STATUS_SUCCESS,
        Err(e) => vm_err_to_hresult(&e),
    }
}

pub fn hostfxr_load_assembly_bytes(bytes: &[u8]) -> i32 {
    let mut vm = lock_vm();
    match vm.load_assembly_bytes(bytes, None) {
        Ok(_) => HOSTFXR_STATUS_SUCCESS,
        Err(e) => vm_err_to_hresult(&e),
    }
}

pub fn hostfxr_load_assembly_and_get_fp(
    asm_path: Option<&Path>,
    type_name: &str,
    method_name: &str,
    is_component_entry: bool,
) -> Result<usize, i32> {
    if type_name.is_empty() || method_name.is_empty() {
        return Err(HOSTFXR_INVALID_ARG);
    }
    let binding = {
        let mut vm = lock_vm();
        let asm_id = if let Some(p) = asm_path {
            Some(vm.load_assembly_file(p).map_err(|e| vm_err_to_hresult(&e))?)
        } else {
            None
        };
        let cid = match asm_id {
            Some(aid) => vm
                .find_class_by_full_name_in_assembly(aid, type_name)
                .or_else(|| vm.find_class_by_full_name_any(type_name)),
            None => vm.find_class_by_full_name_any(type_name),
        }
        .ok_or(COR_E_TYPELOAD)?;
        let mid = vm
            .find_method_in_class(cid, method_name, -1)
            .ok_or(COR_E_MISSINGMETHOD)?;
        let resolved_asm = vm.methods[mid].asm_id;
        DelegateBinding {
            asm_id: resolved_asm,
            method_id: mid,
            target_obj: 0,
            mode: if is_component_entry {
                DelegateCallMode::ComponentEntryPoint
            } else {
                DelegateCallMode::DirectWide
            },
        }
    };
    alloc_delegate_trampoline(binding).map_err(|e| vm_err_to_hresult(&e))
}

// ---------------------------------------------------------------------------
// Embedded CoreCLR Host Store
// ---------------------------------------------------------------------------

const CORECLR_HANDLE_BASE: usize = 0x434C_5200;

struct CoreClrHost {
    domain_id: u32,
    exe_dir: Option<PathBuf>,
    tpa_paths: Vec<PathBuf>,
    app_paths: Vec<PathBuf>,
    latched_exit_code: i32,
}

struct CoreClrState {
    next_id: usize,
    hosts: HashMap<usize, CoreClrHost>,
}

fn coreclr_state() -> &'static Mutex<CoreClrState> {
    static ST: OnceLock<Mutex<CoreClrState>> = OnceLock::new();
    ST.get_or_init(|| {
        Mutex::new(CoreClrState {
            next_id: 1,
            hosts: HashMap::new(),
        })
    })
}

pub fn coreclr_is_embedded_handle(handle: usize) -> bool {
    if handle == 0 {
        return false;
    }
    coreclr_state()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .hosts
        .contains_key(&handle)
}

pub fn coreclr_initialize(
    exe_path: &str,
    domain_name: &str,
    properties: &[(String, String)],
) -> Result<(usize, u32), i32> {
    let exe_pb = if exe_path.is_empty() {
        None
    } else {
        Some(translate_win_path(exe_path))
    };
    let exe_dir = exe_pb
        .as_ref()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()));

    let mut tpa_paths = Vec::new();
    let mut app_paths = Vec::new();
    for (k, v) in properties {
        if k == "TRUSTED_PLATFORM_ASSEMBLIES" {
            for item in v.split([';', ':']).filter(|s| !s.is_empty()) {
                tpa_paths.push(translate_win_path(item));
            }
        } else if k == "APP_PATHS" || k == "APP_CONTEXT_BASE_DIRECTORY" {
            for item in v.split([';', ':']).filter(|s| !s.is_empty()) {
                app_paths.push(translate_win_path(item));
            }
        }
    }

    let domain_id = {
        let mut vm = lock_vm();
        if let Some(d) = &exe_dir {
            if !vm.search_dirs.contains(d) {
                vm.search_dirs.push(d.clone());
            }
        }
        for d in &app_paths {
            if !vm.search_dirs.contains(d) {
                vm.search_dirs.push(d.clone());
            }
        }
        let dom_ptr = vm.create_domain(if domain_name.is_empty() {
            "DefaultDomain"
        } else {
            domain_name
        });
        *vm.domains.get(&dom_ptr).unwrap_or(&1)
    };

    let mut st = coreclr_state().lock().unwrap_or_else(|e| e.into_inner());
    let handle = CORECLR_HANDLE_BASE + st.next_id * 8;
    st.next_id += 1;
    st.hosts.insert(
        handle,
        CoreClrHost {
            domain_id,
            exe_dir,
            tpa_paths,
            app_paths,
            latched_exit_code: 0,
        },
    );
    Ok((handle, domain_id))
}

pub fn coreclr_execute_assembly(
    host_handle: usize,
    domain_id: u32,
    asm_path: &Path,
    args: &[String],
) -> Result<u32, i32> {
    {
        let st = coreclr_state().lock().unwrap_or_else(|e| e.into_inner());
        let Some(h) = st.hosts.get(&host_handle) else {
            return Err(E_INVALIDARG);
        };
        if domain_id != 0 && h.domain_id != domain_id {
            return Err(E_INVALIDARG);
        }
    }
    let exit_code = execute_entry_point_file(asm_path, args)?;
    {
        let mut st = coreclr_state().lock().unwrap_or_else(|e| e.into_inner());
        if let Some(h) = st.hosts.get_mut(&host_handle) {
            h.latched_exit_code = exit_code;
        }
    }
    Ok(exit_code as u32)
}

pub fn coreclr_create_delegate(
    host_handle: usize,
    domain_id: u32,
    assembly_name: &str,
    type_name: &str,
    method_name: &str,
) -> Result<usize, i32> {
    if assembly_name.is_empty() || type_name.is_empty() || method_name.is_empty() {
        return Err(E_INVALIDARG);
    }
    let (tpa, app_paths, exe_dir) = {
        let st = coreclr_state().lock().unwrap_or_else(|e| e.into_inner());
        let Some(h) = st.hosts.get(&host_handle) else {
            return Err(E_INVALIDARG);
        };
        if domain_id != 0 && h.domain_id != domain_id {
            return Err(E_INVALIDARG);
        }
        (h.tpa_paths.clone(), h.app_paths.clone(), h.exe_dir.clone())
    };

    let short_asm = assembly_name
        .split(',')
        .next()
        .unwrap_or(assembly_name)
        .trim();
    let binding = {
        let mut vm = lock_vm();
        let asm_id = if let Some(aid) = vm.find_assembly_by_name(short_asm) {
            aid
        } else {
            // Locate assembly on disk via TPA, APP_PATHS, exe_dir, or direct path
            let mut candidate: Option<PathBuf> = None;
            let direct = translate_win_path(short_asm);
            if direct.is_file() {
                candidate = Some(direct);
            }
            if candidate.is_none() {
                for p in &tpa {
                    let stem_match = p
                        .file_stem()
                        .is_some_and(|s| s.to_string_lossy().eq_ignore_ascii_case(short_asm));
                    if stem_match && p.is_file() {
                        candidate = Some(p.clone());
                        break;
                    }
                }
            }
            if candidate.is_none() {
                for dir in app_paths.iter().chain(exe_dir.iter()) {
                    for ext in ["dll", "exe"] {
                        let p = dir.join(format!("{short_asm}.{ext}"));
                        if p.is_file() {
                            candidate = Some(p);
                            break;
                        }
                    }
                    if candidate.is_some() {
                        break;
                    }
                }
            }
            let path = candidate.ok_or(COR_E_FILENOTFOUND)?;
            vm.load_assembly_file(&path).map_err(|e| vm_err_to_hresult(&e))?
        };
        let cid = vm
            .find_class_by_full_name_in_assembly(asm_id, type_name)
            .ok_or(COR_E_TYPELOAD)?;
        let mid = vm
            .find_method_in_class(cid, method_name, -1)
            .ok_or(COR_E_MISSINGMETHOD)?;
        DelegateBinding {
            asm_id,
            method_id: mid,
            target_obj: 0,
            mode: DelegateCallMode::DirectAnsi,
        }
    };
    alloc_delegate_trampoline(binding).map_err(|e| vm_err_to_hresult(&e))
}

pub fn coreclr_shutdown(host_handle: usize, domain_id: u32) -> Result<i32, i32> {
    let mut st = coreclr_state().lock().unwrap_or_else(|e| e.into_inner());
    let Some(h) = st.hosts.get(&host_handle) else {
        return Err(E_INVALIDARG);
    };
    if domain_id != 0 && h.domain_id != domain_id {
        return Err(E_INVALIDARG);
    }
    let latched = h.latched_exit_code;
    st.hosts.remove(&host_handle);
    Ok(latched)
}

/// Load a PE32/PE32+ assembly from disk and execute its CLI entry point (`Main`).
pub fn execute_entry_point_file(asm_path: &Path, args: &[String]) -> Result<i32, i32> {
    let (entry_mid, cil_args) = {
        let mut vm = lock_vm();
        let aid = vm
            .load_assembly_file(asm_path)
            .map_err(|e| vm_err_to_hresult(&e))?;
        vm.command_line_args = std::iter::once(asm_path.to_string_lossy().into_owned())
            .chain(args.iter().cloned())
            .collect();
        let mid = vm.entry_point_method(aid).ok_or(COR_E_MISSINGMETHOD)?;
        let pcnt = vm.methods[mid].sig.params.len();
        let cil_args = if pcnt >= 1 {
            let str_cls = vm.core.string;
            let arr = vm.alloc_array_for_class(str_cls, args.len());
            for (i, a) in args.iter().enumerate() {
                let so = vm.alloc_string(a);
                unsafe {
                    std::ptr::write_unaligned(
                        (arr + MONO_ARRAY_DATA_OFFSET + i * 8) as *mut usize,
                        so,
                    );
                }
            }
            vec![CilVal::Obj(arr)]
        } else {
            Vec::new()
        };
        (mid, cil_args)
    };
    let ret = vm::execute_method(entry_mid, cil_args).map_err(|e| vm_err_to_hresult(&e))?;
    let exit_code = match ret {
        Some(v) => v.as_i32(),
        None => lock_vm().env_exit_code,
    };
    Ok(exit_code)
}

/// Execute `static int Method(string argument)` (for `ICLRRuntimeHost::ExecuteInDefaultAppDomain`).
pub fn execute_default_app_domain_method(
    asm_path: &Path,
    type_name: &str,
    method_name: &str,
    argument: &str,
) -> Result<u32, i32> {
    let (mid, cil_args) = {
        let mut vm = lock_vm();
        let aid = vm
            .load_assembly_file(asm_path)
            .map_err(|e| vm_err_to_hresult(&e))?;
        let cid = vm
            .find_class_by_full_name_in_assembly(aid, type_name)
            .ok_or(COR_E_TYPELOAD)?;
        let mid = vm
            .find_method_in_class(cid, method_name, 1)
            .or_else(|| vm.find_method_in_class(cid, method_name, -1))
            .ok_or(COR_E_MISSINGMETHOD)?;
        let pcnt = vm.methods[mid].sig.params.len();
        let cil_args = if pcnt >= 1 {
            vec![CilVal::Obj(vm.alloc_string(argument))]
        } else {
            Vec::new()
        };
        (mid, cil_args)
    };
    let ret = vm::execute_method(mid, cil_args).map_err(|e| vm_err_to_hresult(&e))?;
    Ok(ret.map_or(0, |v| v.as_i32() as u32))
}

pub fn mscoree_create_domain(name: &str) -> u32 {
    let mut vm = lock_vm();
    let ptr = vm.create_domain(name);
    *vm.domains.get(&ptr).unwrap_or(&1)
}

pub fn mscoree_unload_domain(domain_id: u32) -> i32 {
    let mut vm = lock_vm();
    if let Some(ptr) = vm.domains_by_id.remove(&domain_id) {
        vm.domains.remove(&ptr);
        S_OK
    } else {
        E_INVALIDARG
    }
}

pub fn mscoree_domain_ids() -> Vec<u32> {
    let vm = lock_vm();
    let mut ids: Vec<u32> = vm.domains_by_id.keys().copied().collect();
    ids.sort_unstable();
    ids
}

// ---------------------------------------------------------------------------
// Embedded Mono C API Bridge Helpers
// ---------------------------------------------------------------------------

pub fn mono_set_dirs(asm_dir: Option<&Path>, cfg_dir: Option<&Path>) {
    let mut vm = lock_vm();
    for d in [asm_dir, cfg_dir].into_iter().flatten() {
        let pb = d.to_path_buf();
        if !vm.search_dirs.contains(&pb) {
            vm.search_dirs.push(pb);
        }
    }
}

pub fn mono_jit_init(domain_name: &str) -> *mut c_void {
    let mut vm = lock_vm();
    if vm.root_domain == 0 {
        let dom = vm.create_domain(if domain_name.is_empty() {
            "RootDomain"
        } else {
            domain_name
        });
        vm.root_domain = dom;
        vm.current_domain = dom;
    }
    let dom = vm.root_domain;
    vm.attach_current_thread(dom);
    dom as *mut c_void
}

pub fn mono_domain_get() -> *mut c_void {
    let vm = lock_vm();
    vm.current_domain as *mut c_void
}

pub fn mono_get_root_domain() -> *mut c_void {
    let vm = lock_vm();
    vm.root_domain as *mut c_void
}

pub fn mono_domain_set(domain: *mut c_void) -> bool {
    let mut vm = lock_vm();
    let d = domain as usize;
    if vm.is_valid_domain(d) {
        vm.current_domain = d;
        true
    } else {
        false
    }
}

pub fn mono_thread_attach(domain: *mut c_void) -> *mut c_void {
    let mut vm = lock_vm();
    vm.attach_current_thread(domain as usize) as *mut c_void
}

pub fn mono_thread_detach(thread: *mut c_void) {
    let mut vm = lock_vm();
    vm.detach_thread(thread as usize);
}

pub fn mono_thread_current() -> *mut c_void {
    let vm = lock_vm();
    vm.current_thread() as *mut c_void
}

pub fn mono_assembly_open(path: &Path) -> Result<*mut c_void, i32> {
    let mut vm = lock_vm();
    match vm.load_assembly_file(path) {
        Ok(aid) => Ok(vm.assemblies[aid].mono_assembly as *mut c_void),
        Err(VmError::FileNotFound(_)) => Err(MONO_IMAGE_ERROR_ERRNO),
        Err(_) => Err(MONO_IMAGE_IMAGE_INVALID),
    }
}

pub fn mono_image_open_from_data(data: &[u8]) -> Result<*mut c_void, i32> {
    let mut vm = lock_vm();
    match vm.load_assembly_bytes(data, None) {
        Ok(aid) => Ok(vm.assemblies[aid].mono_image as *mut c_void),
        Err(_) => Err(MONO_IMAGE_IMAGE_INVALID),
    }
}

pub fn mono_assembly_from_image(image: *mut c_void) -> *mut c_void {
    let vm = lock_vm();
    let Some(&aid) = vm.image_by_ptr.get(&(image as usize)) else {
        return std::ptr::null_mut();
    };
    vm.assemblies[aid].mono_assembly as *mut c_void
}

pub fn mono_assembly_get_image(assembly: *mut c_void) -> *mut c_void {
    let vm = lock_vm();
    let Some(&aid) = vm.assembly_by_ptr.get(&(assembly as usize)) else {
        return std::ptr::null_mut();
    };
    vm.assemblies[aid].mono_image as *mut c_void
}

pub fn mono_get_corlib() -> *mut c_void {
    let vm = lock_vm();
    vm.corlib_image() as *mut c_void
}

pub fn mono_image_get_name(image: *mut c_void) -> *const c_char {
    let vm = lock_vm();
    let Some(&aid) = vm.image_by_ptr.get(&(image as usize)) else {
        return std::ptr::null();
    };
    let img_ptr = vm.assemblies[aid].mono_image as *const vm::MonoImage;
    unsafe { (*img_ptr).name_ptr }
}

pub fn mono_class_from_name(image: *mut c_void, ns: &str, name: &str) -> *mut c_void {
    let vm = lock_vm();
    let Some(&aid) = vm.image_by_ptr.get(&(image as usize)) else {
        return std::ptr::null_mut();
    };
    vm.find_class_in_assembly(aid, ns, name)
        .map_or(std::ptr::null_mut(), |cid| vm.classes[cid].mono_class as *mut c_void)
}

pub fn mono_class_get_name(klass: *mut c_void) -> *const c_char {
    let vm = lock_vm();
    let Some(&cid) = vm.class_by_ptr.get(&(klass as usize)) else {
        return std::ptr::null();
    };
    let cp = vm.classes[cid].mono_class as *const vm::MonoClass;
    unsafe { (*cp).name_ptr }
}

pub fn mono_class_get_namespace(klass: *mut c_void) -> *const c_char {
    let vm = lock_vm();
    let Some(&cid) = vm.class_by_ptr.get(&(klass as usize)) else {
        return std::ptr::null();
    };
    let cp = vm.classes[cid].mono_class as *const vm::MonoClass;
    unsafe { (*cp).namespace_ptr }
}

pub fn mono_class_vtable(klass: *mut c_void) -> *mut c_void {
    let vm = lock_vm();
    let Some(&cid) = vm.class_by_ptr.get(&(klass as usize)) else {
        return std::ptr::null_mut();
    };
    vm.classes[cid].mono_vtable as *mut c_void
}

pub fn mono_class_get_method_from_name(
    klass: *mut c_void,
    name: &str,
    param_count: i32,
) -> *mut c_void {
    let vm = lock_vm();
    let Some(&cid) = vm.class_by_ptr.get(&(klass as usize)) else {
        return std::ptr::null_mut();
    };
    vm.find_method_in_class(cid, name, param_count)
        .map_or(std::ptr::null_mut(), |mid| vm.methods[mid].mono_method as *mut c_void)
}

pub fn mono_class_get_field_from_name(klass: *mut c_void, name: &str) -> *mut c_void {
    let vm = lock_vm();
    let Some(&cid) = vm.class_by_ptr.get(&(klass as usize)) else {
        return std::ptr::null_mut();
    };
    vm.find_field_in_class(cid, name)
        .map_or(std::ptr::null_mut(), |fid| vm.fields[fid].mono_field as *mut c_void)
}

pub fn mono_class_get_property_from_name(klass: *mut c_void, name: &str) -> *mut c_void {
    let vm = lock_vm();
    let Some(&cid) = vm.class_by_ptr.get(&(klass as usize)) else {
        return std::ptr::null_mut();
    };
    vm.find_property_in_class(cid, name)
        .map_or(std::ptr::null_mut(), |pid| vm.properties[pid].mono_prop as *mut c_void)
}

pub fn mono_method_get_name(method: *mut c_void) -> *const c_char {
    let vm = lock_vm();
    let Some(&mid) = vm.method_by_ptr.get(&(method as usize)) else {
        return std::ptr::null();
    };
    let mp = vm.methods[mid].mono_method as *const vm::MonoMethod;
    unsafe { (*mp).name_ptr }
}

pub fn mono_field_get_name(field: *mut c_void) -> *const c_char {
    let vm = lock_vm();
    let Some(&fid) = vm.field_by_ptr.get(&(field as usize)) else {
        return std::ptr::null();
    };
    let fp = vm.fields[fid].mono_field as *const vm::MonoClassField;
    unsafe { (*fp).name_ptr }
}

pub fn mono_property_get_name(prop: *mut c_void) -> *const c_char {
    let vm = lock_vm();
    let Some(&pid) = vm.prop_by_ptr.get(&(prop as usize)) else {
        return std::ptr::null();
    };
    let pp = vm.properties[pid].mono_prop as *const vm::MonoProperty;
    unsafe { (*pp).name_ptr }
}

pub fn mono_method_desc_new(spec: &str, include_namespace: bool) -> *mut c_void {
    let desc = vm::parse_mono_method_desc(spec, include_namespace);
    let b = Box::new(desc.clone());
    let ptr = Box::into_raw(b) as usize;
    let mut vm = lock_vm();
    vm.method_descs.insert(ptr, desc);
    ptr as *mut c_void
}

pub fn mono_method_desc_free(desc: *mut c_void) {
    let ptr = desc as usize;
    if ptr == 0 {
        return;
    }
    let removed = lock_vm().method_descs.remove(&ptr).is_some();
    if removed {
        unsafe {
            drop(Box::from_raw(ptr as *mut vm::MonoMethodDesc));
        }
    }
}

pub fn mono_method_desc_search_in_class(desc: *mut c_void, klass: *mut c_void) -> *mut c_void {
    let vm = lock_vm();
    let Some(d) = vm.method_descs.get(&(desc as usize)) else {
        return std::ptr::null_mut();
    };
    let Some(&cid) = vm.class_by_ptr.get(&(klass as usize)) else {
        return std::ptr::null_mut();
    };
    vm::match_method_desc_in_class(&vm, d, cid)
        .map_or(std::ptr::null_mut(), |mid| vm.methods[mid].mono_method as *mut c_void)
}

pub fn mono_method_desc_search_in_image(desc: *mut c_void, image: *mut c_void) -> *mut c_void {
    let vm = lock_vm();
    let Some(d) = vm.method_descs.get(&(desc as usize)) else {
        return std::ptr::null_mut();
    };
    let Some(&aid) = vm.image_by_ptr.get(&(image as usize)) else {
        return std::ptr::null_mut();
    };
    for &cid in &vm.assemblies[aid].class_ids {
        if let Some(mid) = vm::match_method_desc_in_class(&vm, d, cid) {
            return vm.methods[mid].mono_method as *mut c_void;
        }
    }
    std::ptr::null_mut()
}

pub unsafe fn mono_jit_exec(
    domain: *mut c_void,
    assembly: *mut c_void,
    args: &[String],
) -> i32 {
    let (entry_mid, cil_args) = {
        let mut vm = lock_vm();
        if !vm.is_valid_domain(domain as usize) {
            return -1;
        }
        let Some(&aid) = vm.assembly_by_ptr.get(&(assembly as usize)) else {
            return -1;
        };
        let Some(mid) = vm.entry_point_method(aid) else {
            return -1;
        };
        let pcnt = vm.methods[mid].sig.params.len();
        let cli_args = if pcnt >= 1 {
            // Mono passes argv[1..] to Main(string[]) when argv[0] is program name
            let pass_args = if args.len() > 1 { &args[1..] } else { &[] };
            let str_cls = vm.core.string;
            let arr = vm.alloc_array_for_class(str_cls, pass_args.len());
            for (i, a) in pass_args.iter().enumerate() {
                let so = vm.alloc_string(a);
                std::ptr::write_unaligned((arr + MONO_ARRAY_DATA_OFFSET + i * 8) as *mut usize, so);
            }
            vec![CilVal::Obj(arr)]
        } else {
            Vec::new()
        };
        (mid, cli_args)
    };
    match vm::execute_method(entry_mid, cil_args) {
        Ok(Some(v)) => v.as_i32(),
        Ok(None) => lock_vm().env_exit_code,
        Err(_) => -1,
    }
}

pub unsafe fn mono_runtime_invoke(
    method: *mut c_void,
    obj: *mut c_void,
    params: *mut *mut c_void,
    exc: *mut *mut c_void,
) -> *mut c_void {
    if !exc.is_null() {
        std::ptr::write_unaligned(exc, std::ptr::null_mut());
    }
    let (mid, cil_args, ret_sig, asm_id) = {
        let vm = lock_vm();
        let Some(&mid) = vm.method_by_ptr.get(&(method as usize)) else {
            return std::ptr::null_mut();
        };
        let m = &vm.methods[mid];
        let sig = m.sig.clone();
        let mut args = Vec::with_capacity(sig.params.len() + 1);
        if sig.has_this && !sig.explicit_this {
            let cls = &vm.classes[m.class_id];
            if cls.is_value_type && !obj.is_null() {
                args.push(CilVal::ManagedPtr(vm::ManagedAddr::Raw(obj as usize)));
            } else {
                args.push(CilVal::Obj(obj as usize));
            }
        }
        for (i, pty) in sig.params.iter().enumerate() {
            let slot_ptr = if params.is_null() {
                std::ptr::null_mut()
            } else {
                std::ptr::read_unaligned(params.add(i))
            };
            let val = if pty.is_value_type() {
                if slot_ptr.is_null() {
                    vm::default_val_for_sig(pty)
                } else {
                    vm::read_sig_from_ptr(pty, slot_ptr as *const u8)
                }
            } else {
                CilVal::Obj(slot_ptr as usize)
            };
            args.push(val);
        }
        (mid, args, sig.ret, m.asm_id)
    };

    match vm::execute_method(mid, cil_args) {
        Ok(Some(val)) => {
            if ret_sig == SigType::Void {
                std::ptr::null_mut()
            } else if ret_sig.is_value_type() {
                let mut vm = lock_vm();
                let cid = vm.class_id_for_sig(asm_id, &ret_sig);
                vm.box_value(cid, val) as *mut c_void
            } else {
                val.as_obj() as *mut c_void
            }
        }
        Ok(None) => std::ptr::null_mut(),
        Err(VmError::ManagedException { exc_obj, .. }) => {
            if !exc.is_null() {
                std::ptr::write_unaligned(exc, exc_obj as *mut c_void);
            }
            std::ptr::null_mut()
        }
        Err(e) => {
            if !exc.is_null() {
                let mut vm = lock_vm();
                let ec = vm.core.exception;
                let eo = vm.make_exception(ec, e.to_string());
                std::ptr::write_unaligned(exc, eo as *mut c_void);
            }
            std::ptr::null_mut()
        }
    }
}

pub fn mono_add_internal_call(name: &str, method: *const c_void) {
    if !name.is_empty() && !method.is_null() {
        lock_vm().register_internal_call(name, method as usize);
    }
}

pub fn mono_string_new(s: &str) -> *mut c_void {
    lock_vm().alloc_string(s) as *mut c_void
}

pub fn mono_string_new_utf16(u16s: &[u16]) -> *mut c_void {
    lock_vm().alloc_string_utf16(u16s) as *mut c_void
}

pub unsafe fn mono_string_to_utf8(string_obj: *mut c_void) -> *mut c_char {
    if string_obj.is_null() {
        return std::ptr::null_mut();
    }
    let Some(s) = lock_vm().heap.read_string(string_obj as usize) else {
        return std::ptr::null_mut();
    };
    let bytes = s.as_bytes();
    let buf = libc::malloc(bytes.len() + 1) as *mut u8;
    if buf.is_null() {
        return std::ptr::null_mut();
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
    std::ptr::write_unaligned(buf.add(bytes.len()), 0u8);
    buf as *mut c_char
}

pub unsafe fn mono_string_chars(string_obj: *mut c_void) -> *mut u16 {
    if string_obj.is_null() {
        return std::ptr::null_mut();
    }
    let vm = lock_vm();
    if !matches!(
        vm.heap.objects.get(&(string_obj as usize)),
        Some(vm::HeapObjKind::String)
    ) {
        return std::ptr::null_mut();
    }
    (string_obj as usize + vm::MONO_STRING_CHARS_OFFSET) as *mut u16
}

pub unsafe fn mono_string_length(string_obj: *mut c_void) -> i32 {
    if string_obj.is_null() {
        return 0;
    }
    let vm = lock_vm();
    if !matches!(
        vm.heap.objects.get(&(string_obj as usize)),
        Some(vm::HeapObjKind::String)
    ) {
        return 0;
    }
    std::ptr::read_unaligned((string_obj as usize + vm::MONO_STRING_LEN_OFFSET) as *const i32)
}

pub fn mono_object_new(klass: *mut c_void) -> *mut c_void {
    let mut vm = lock_vm();
    let Some(&cid) = vm.class_by_ptr.get(&(klass as usize)) else {
        return std::ptr::null_mut();
    };
    vm.alloc_object_for_class(cid) as *mut c_void
}

pub fn mono_runtime_object_init(this_obj: *mut c_void) {
    if this_obj.is_null() {
        return;
    }
    let ctor_mid = {
        let vm = lock_vm();
        let Some(cid) = vm.object_class_id(this_obj as usize) else {
            return;
        };
        vm.find_method_in_class(cid, ".ctor", 0)
    };
    if let Some(mid) = ctor_mid {
        let _ = vm::execute_method(mid, vec![CilVal::Obj(this_obj as usize)]);
    }
}

pub fn mono_runtime_class_init(vtable: *mut c_void) {
    if vtable.is_null() {
        return;
    }
    let cctor_mid = {
        let mut vm = lock_vm();
        let Some(&cid) = vm.vtable_by_ptr.get(&(vtable as usize)) else {
            return;
        };
        if vm.classes[cid].cctor_ran {
            return;
        }
        vm.classes[cid].cctor_ran = true;
        vm.find_method_in_class(cid, ".cctor", 0)
    };
    if let Some(mid) = cctor_mid {
        let _ = vm::execute_method(mid, Vec::new());
    }
}

pub fn mono_object_unbox(obj: *mut c_void) -> *mut c_void {
    if obj.is_null() {
        return std::ptr::null_mut();
    }
    let vm = lock_vm();
    if !vm.heap.objects.contains_key(&(obj as usize)) {
        return std::ptr::null_mut();
    }
    (obj as usize + MONO_OBJECT_HEADER_SIZE) as *mut c_void
}

pub fn mono_object_get_class(obj: *mut c_void) -> *mut c_void {
    if obj.is_null() {
        return std::ptr::null_mut();
    }
    let vm = lock_vm();
    vm.object_class_id(obj as usize)
        .map_or(std::ptr::null_mut(), |cid| vm.classes[cid].mono_class as *mut c_void)
}

pub unsafe fn mono_value_box(klass: *mut c_void, val: *mut c_void) -> *mut c_void {
    if klass.is_null() || val.is_null() {
        return std::ptr::null_mut();
    }
    let mut vm = lock_vm();
    let Some(&cid) = vm.class_by_ptr.get(&(klass as usize)) else {
        return std::ptr::null_mut();
    };
    let vt = vm.classes[cid].mono_vtable;
    let sz = vm.classes[cid].elem_size.max(1);
    let sig = vm.classes[cid]
        .primitive_sig
        .clone()
        .unwrap_or(SigType::ValueType(0));
    let bytes = std::slice::from_raw_parts(val as *const u8, sz).to_vec();
    vm.heap.alloc_boxed(vt, cid, sig, &bytes) as *mut c_void
}

pub unsafe fn mono_field_get_value(obj: *mut c_void, field: *mut c_void, val: *mut c_void) {
    if field.is_null() || val.is_null() {
        return;
    }
    let vm = lock_vm();
    let Some(&fid) = vm.field_by_ptr.get(&(field as usize)) else {
        return;
    };
    let f = &vm.fields[fid];
    if f.is_static || obj.is_null() {
        let bytes = vm::cil_val_to_bytes(&f.sig, f.static_value);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), val as *mut u8, bytes.len());
    } else {
        let src = (obj as usize + MONO_OBJECT_HEADER_SIZE + f.struct_offset) as *const u8;
        std::ptr::copy_nonoverlapping(src, val as *mut u8, f.size);
    }
}

pub unsafe fn mono_field_set_value(obj: *mut c_void, field: *mut c_void, val: *mut c_void) {
    if field.is_null() || val.is_null() {
        return;
    }
    let mut vm = lock_vm();
    let Some(&fid) = vm.field_by_ptr.get(&(field as usize)) else {
        return;
    };
    let is_static = vm.fields[fid].is_static;
    let sig = vm.fields[fid].sig.clone();
    let off = vm.fields[fid].struct_offset;
    let sz = vm.fields[fid].size;
    if is_static || obj.is_null() {
        let cv = if sig.is_value_type() {
            vm::read_sig_from_ptr(&sig, val as *const u8)
        } else {
            CilVal::Obj(val as usize)
        };
        vm.fields[fid].static_value = cv;
    } else if sig.is_value_type() {
        let dst = (obj as usize + MONO_OBJECT_HEADER_SIZE + off) as *mut u8;
        std::ptr::copy_nonoverlapping(val as *const u8, dst, sz);
    } else {
        // Real Mono: for reference fields, `val` IS the `MonoObject*` pointer itself!
        let dst = (obj as usize + MONO_OBJECT_HEADER_SIZE + off) as *mut usize;
        std::ptr::write_unaligned(dst, val as usize);
    }
}

pub unsafe fn mono_field_static_get_value(
    _vt: *mut c_void,
    field: *mut c_void,
    val: *mut c_void,
) {
    mono_field_get_value(std::ptr::null_mut(), field, val);
}

pub unsafe fn mono_field_static_set_value(
    _vt: *mut c_void,
    field: *mut c_void,
    val: *mut c_void,
) {
    mono_field_set_value(std::ptr::null_mut(), field, val);
}

pub fn mono_property_get_get_method(prop: *mut c_void) -> *mut c_void {
    let vm = lock_vm();
    let Some(&pid) = vm.prop_by_ptr.get(&(prop as usize)) else {
        return std::ptr::null_mut();
    };
    vm.properties[pid]
        .getter_method_id
        .map_or(std::ptr::null_mut(), |mid| vm.methods[mid].mono_method as *mut c_void)
}

pub fn mono_property_get_set_method(prop: *mut c_void) -> *mut c_void {
    let vm = lock_vm();
    let Some(&pid) = vm.prop_by_ptr.get(&(prop as usize)) else {
        return std::ptr::null_mut();
    };
    vm.properties[pid]
        .setter_method_id
        .map_or(std::ptr::null_mut(), |mid| vm.methods[mid].mono_method as *mut c_void)
}

pub fn mono_array_new(eclass: *mut c_void, n: usize) -> *mut c_void {
    let mut vm = lock_vm();
    let elem_cid = vm
        .class_by_ptr
        .get(&(eclass as usize))
        .copied()
        .unwrap_or(vm.core.object);
    vm.alloc_array_for_class(elem_cid, n) as *mut c_void
}

pub unsafe fn mono_array_length(array: *mut c_void) -> usize {
    if array.is_null() {
        return 0;
    }
    let vm = lock_vm();
    match vm.heap.objects.get(&(array as usize)) {
        Some(vm::HeapObjKind::Array { length, .. }) => *length,
        _ => 0,
    }
}

pub unsafe fn mono_array_addr_with_size(
    array: *mut c_void,
    size: i32,
    idx: usize,
) -> *mut u8 {
    if array.is_null() || size <= 0 {
        return std::ptr::null_mut();
    }
    let vm = lock_vm();
    if !matches!(
        vm.heap.objects.get(&(array as usize)),
        Some(vm::HeapObjKind::Array { .. })
    ) {
        return std::ptr::null_mut();
    }
    (array as usize + MONO_ARRAY_DATA_OFFSET + idx.saturating_mul(size as usize)) as *mut u8
}

pub fn mono_array_element_size(eclass: *mut c_void) -> i32 {
    let vm = lock_vm();
    vm.class_by_ptr
        .get(&(eclass as usize))
        .map_or(8, |&cid| vm.classes[cid].elem_size as i32)
}

pub fn mono_gchandle_new(obj: *mut c_void, pinned: bool) -> u32 {
    if obj.is_null() {
        return 0;
    }
    lock_vm().heap.gchandle_new(obj as usize, pinned, false)
}

pub fn mono_gchandle_new_weakref(obj: *mut c_void, _track: bool) -> u32 {
    if obj.is_null() {
        return 0;
    }
    lock_vm().heap.gchandle_new(obj as usize, false, true)
}

pub fn mono_gchandle_get_target(handle: u32) -> *mut c_void {
    lock_vm().heap.gchandle_get(handle) as *mut c_void
}

pub fn mono_gchandle_free(handle: u32) {
    lock_vm().heap.gchandle_free(handle);
}

pub unsafe fn mono_class_iter_methods(klass: *mut c_void, iter: *mut *mut c_void) -> *mut c_void {
    if klass.is_null() || iter.is_null() {
        return std::ptr::null_mut();
    }
    let vm = lock_vm();
    let Some(&cid) = vm.class_by_ptr.get(&(klass as usize)) else {
        return std::ptr::null_mut();
    };
    let idx = std::ptr::read_unaligned(iter) as usize;
    if let Some(&mid) = vm.classes[cid].methods.get(idx) {
        std::ptr::write_unaligned(iter, (idx + 1) as *mut c_void);
        vm.methods[mid].mono_method as *mut c_void
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn mono_class_iter_fields(klass: *mut c_void, iter: *mut *mut c_void) -> *mut c_void {
    if klass.is_null() || iter.is_null() {
        return std::ptr::null_mut();
    }
    let vm = lock_vm();
    let Some(&cid) = vm.class_by_ptr.get(&(klass as usize)) else {
        return std::ptr::null_mut();
    };
    let idx = std::ptr::read_unaligned(iter) as usize;
    if let Some(&fid) = vm.classes[cid].fields.get(idx) {
        std::ptr::write_unaligned(iter, (idx + 1) as *mut c_void);
        vm.fields[fid].mono_field as *mut c_void
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe fn mono_class_iter_properties(
    klass: *mut c_void,
    iter: *mut *mut c_void,
) -> *mut c_void {
    if klass.is_null() || iter.is_null() {
        return std::ptr::null_mut();
    }
    let vm = lock_vm();
    let Some(&cid) = vm.class_by_ptr.get(&(klass as usize)) else {
        return std::ptr::null_mut();
    };
    let idx = std::ptr::read_unaligned(iter) as usize;
    if let Some(&pid) = vm.classes[cid].properties.get(idx) {
        std::ptr::write_unaligned(iter, (idx + 1) as *mut c_void);
        vm.properties[pid].mono_prop as *mut c_void
    } else {
        std::ptr::null_mut()
    }
}

pub fn mono_builtin_class(which: u8) -> *mut c_void {
    let vm = lock_vm();
    let cid = match which {
        0 => vm.core.object,
        1 => vm.core.int32,
        2 => vm.core.uint32,
        3 => vm.core.int64,
        4 => vm.core.byte,
        5 => vm.core.boolean,
        6 => vm.core.single,
        7 => vm.core.double,
        8 => vm.core.string,
        _ => vm.core.object,
    };
    vm.classes[cid].mono_class as *mut c_void
}

#[cfg(test)]
mod tests {
    use super::metadata::elem_type;
    use super::metadata::test_asm::{TestAsmBuilder, TestMethodBody};
    use super::*;

    #[test]
    fn delegate_trampoline_and_pinvoke_execute_end_to_end() {
        let mut b = TestAsmBuilder::new("ClrInteropTest");
        let corlib = b.add_assembly_ref("mscorlib");
        let obj_ref = b.add_type_ref(corlib, "System", "Object");
        let _td = b.add_type_def("Interop", "NativeBridge", Some(obj_ref));

        // Method 1: P/Invoke `kernel32.dll!GetCurrentProcessId()` -> u32
        let m_pid = b.add_method(
            0x2016, // Public | Static | PinvokeImpl
            0,
            "GetCurrentProcessId",
            &[0x00, 0x00, elem_type::U4],
            None,
        );
        b.add_pinvoke(m_pid & 0x00FF_FFFF, "kernel32.dll", "GetCurrentProcessId", 0);

        // Method 2: `CallPidPlus(int bonus)` -> `GetCurrentProcessId() + bonus`
        let mut code = Vec::new();
        code.push(0x28);
        code.extend_from_slice(&m_pid.to_le_bytes());
        code.extend_from_slice(&[0x02, 0x58, 0x2A]); // ldarg.0; add; ret
        let _m_call = b.add_method(
            0x0016,
            0,
            "CallPidPlus",
            &[0x00, 0x01, elem_type::I4, elem_type::I4],
            Some(TestMethodBody::tiny(code)),
        );

        let pe = b.build();
        let (aid, mid) = {
            let mut vm = lock_vm();
            let aid = vm.load_assembly_bytes(&pe, None).unwrap();
            let cid = vm
                .find_class_in_assembly(aid, "Interop", "NativeBridge")
                .unwrap();
            let mid = vm.find_method_in_class(cid, "CallPidPlus", 1).unwrap();
            (aid, mid)
        };

        let fp = alloc_delegate_trampoline(DelegateBinding {
            asm_id: aid,
            method_id: mid,
            target_obj: 0,
            mode: DelegateCallMode::DirectAnsi,
        })
        .expect("allocate trampoline");
        assert_ne!(fp, 0);

        unsafe {
            let f: unsafe extern "win64" fn(i32) -> i32 = std::mem::transmute(fp);
            let expected_pid = crate::kernel32::process::GetCurrentProcessId() as i32;
            assert_eq!(f(17), expected_pid.wrapping_add(17));
        }
    }
}
