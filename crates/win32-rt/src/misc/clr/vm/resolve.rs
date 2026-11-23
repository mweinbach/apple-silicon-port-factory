//! Per-assembly metadata token resolution caches (`MethodDef`/`MemberRef`/`MethodSpec`,
//! `Field`/`MemberRef`, `#US` user-string interning, `TypeRef`/`TypeSpec`, `StandAloneSig`,
//! and virtual dispatch cache) plus `BclIntrinsic` classification.

use super::super::metadata::{method_attr, method_impl_attr, table, MethodSig, SigType};
use super::value::{CilVal, StorageKind};
use super::ClrVm;
use std::sync::Arc;

/// Fast copyable enum replacing per-call string allocations & linear matching for hot BCL methods.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BclIntrinsic {
    // System.Object
    ObjectCtor,
    ObjectReferenceEquals,
    ObjectEquals,
    ObjectGetHashCode,
    ObjectToString,
    ObjectGetType,

    // System.Type
    TypeGetTypeFromHandle,
    TypeGetName,
    TypeGetFullName,
    TypeGetNamespace,

    // System.String
    StringGetLength,
    StringGetChars,
    StringConcat,
    StringConcatArray,
    StringOpEquality,
    StringOpInequality,
    StringEquals,
    StringCompare,
    StringIsNullOrEmpty,
    StringIsNullOrWhiteSpace,
    StringSubstring,
    StringContains,
    StringStartsWith,
    StringEndsWith,
    StringIndexOfChar,
    StringIndexOfStr,
    StringToUpper,
    StringToLower,
    StringTrim,
    StringReplaceChar,
    StringReplaceStr,
    StringToCharArray,
    StringFormat,
    StringToString,
    StringGetHashCode,

    // System.Text.StringBuilder
    SbAppendChar,
    SbAppendVal,
    SbToString,
    SbGetLength,
    SbClear,

    // System.Math / System.MathF
    MathAbsF64,
    MathAbsI64,
    MathAbsI32,
    MathMinF64,
    MathMinI64,
    MathMinI32,
    MathMaxF64,
    MathMaxI64,
    MathMaxI32,
    MathClampF64,
    MathClampI64,
    MathClampI32,
    MathSignF64,
    MathSignInt,
    MathSqrt,
    MathPow,
    MathExp,
    MathLog,
    MathLog10,
    MathLog2,
    MathSin,
    MathCos,
    MathTan,
    MathAsin,
    MathAcos,
    MathAtan,
    MathAtan2,
    MathSinh,
    MathCosh,
    MathTanh,
    MathFloor,
    MathCeiling,
    MathRound,
    MathTruncate,
    MathFma,
    MathUnknown,

    // System.Array
    ArrayGetLength,
    ArrayGetLongLength,
    ArrayGetRank,
    ArrayCopy,
    ArrayClear,
    ArrayEmpty,

    // System.Buffer
    BufferBlockCopy,
    BufferByteLength,
    BufferGetByte,
    BufferSetByte,
    BufferMemoryCopy,

    // System.Collections.Generic.List<T>
    ListAdd,
    ListGetCount,
    ListGetItem,
    ListSetItem,
    ListClear,
    ListContains,
    ListIndexOf,
    ListRemoveAt,
    ListRemove,
    ListToArray,

    // System.Collections.Generic.Dictionary<K, V>
    DictAdd,
    DictGetCount,
    DictGetItem,
    DictSetItem,
    DictContainsKey,
    DictTryGetValue,
    DictRemove,
    DictClear,

    // System.Environment
    EnvGetTickCount,
    EnvGetTickCount64,
    EnvGetProcessorCount,
    EnvGetIs64Bit,
    EnvGetNewLine,
    EnvGetCurrentDirectory,
    EnvGetMachineName,
    EnvGetUserName,
    EnvGetEnvVar,
    EnvSetEnvVar,
    EnvGetCommandLineArgs,
    EnvGetExitCode,
    EnvSetExitCode,
    EnvExit,

    // System.Runtime.InteropServices.Marshal
    MarshalAllocHGlobal,
    MarshalReAllocHGlobal,
    MarshalFreeHGlobal,
    MarshalReadByte,
    MarshalReadInt16,
    MarshalReadInt32,
    MarshalReadInt64,
    MarshalReadIntPtr,
    MarshalWriteByte,
    MarshalWriteInt16,
    MarshalWriteInt32,
    MarshalWriteInt64,
    MarshalWriteIntPtr,
    MarshalCopyFromArray,
    MarshalCopyToArray,
    MarshalStringToAnsi,
    MarshalStringToUni,
    MarshalPtrToStringAnsi,
    MarshalPtrToStringUni,
    MarshalGetLastWin32Error,
    MarshalSizeOf,
    MarshalGetFnPtrForDelegate,

    // System.Runtime.CompilerServices.RuntimeHelpers
    RuntimeHelpersInitArray,
    RuntimeHelpersGetHashCode,

    // System.Console
    ConsoleWrite,

    // System.Exception
    ExceptionGetMessage,

    // System.BitConverter
    BitConvSingleToI32,
    BitConvI32ToSingle,
    BitConvDoubleToI64,
    BitConvI64ToDouble,

    // System.IntPtr / UIntPtr
    IntPtrToI32,
    IntPtrToI64,
    IntPtrToPtr,
    IntPtrExplicitToI32,
    IntPtrExplicitToI64,
    IntPtrExplicitToNative,
    IntPtrOpEq,
    IntPtrOpNeq,

    // Primitives & System.Convert
    Int32Parse,
    Int64Parse,
    PrimToStringI32,
    PrimToStringU32,
    PrimToStringI64,
    PrimToStringU64,
    PrimToStringF32,
    PrimToStringF64,
    PrimToStringBool,
    PrimToStringU8,
    ConvertToI32,
    ConvertToI64,
    ConvertToF64,
    ConvertToString,

    // System.GC & base .ctor fallback
    GcNoop,
    BaseCtorNoop,
}

/// Fast copyable enum for BCL `newobj` targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BclNewObjIntrinsic {
    Object,
    StringFromCharArray,
    StringFromCharPtr,
    StringFromCharCount,
    StringFromCharArraySlice,
    StringEmpty,
    IntPtr,
    StringBuilderEmpty,
    StringBuilderFromString,
    ListEmpty,
    ListWithCapacity,
    DictionaryEmpty,
    DictionaryWithCapacity,
    Exception(usize),
    GenericObject(usize),
}

/// Resolved execution target cached per `MethodInfo` or `MemberRef`/`MethodSpec` token.
#[derive(Clone, Debug)]
pub(crate) enum ResolvedTarget {
    Managed(usize),
    Bcl(BclIntrinsic),
    PInvoke(usize),
    InternalCall { fn_ptr: usize, ret_sig: SigType },
    DelegateInvoke,
    UnsupportedBcl(Arc<str>),
}

/// Cached per-assembly entry for a `call`/`callvirt` token (`MemberRef` / `MethodSpec`).
#[derive(Clone, Debug)]
pub(crate) struct CachedCallEntry {
    pub argc: usize,
    pub has_this: bool,
    pub param_count: i32,
    pub target: ResolvedTarget,
    pub method_name: Arc<str>,
}

/// Cached per-assembly entry for a `newobj` token.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ResolvedNewObj {
    ManagedClass {
        ctor_mid: usize,
        class_id: usize,
    },
    ManagedValueType {
        ctor_mid: usize,
        class_id: usize,
        size: usize,
    },
    Delegate {
        class_id: usize,
    },
    Bcl(BclNewObjIntrinsic),
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CachedNewObjEntry {
    pub argc: usize,
    pub kind: ResolvedNewObj,
}

/// Cached per-assembly entry for a field token (`Field` / `MemberRef`).
#[derive(Clone, Copy, Debug)]
pub(crate) enum ResolvedField {
    Field {
        field_id: usize,
        class_id: usize,
        struct_offset: usize,
        size: usize,
        is_static: bool,
        kind: StorageKind,
    },
    BclConst(CilVal),
}

pub(crate) fn classify_bcl_method(
    ns: &str,
    ty: &str,
    method: &str,
    sig: &MethodSig,
) -> Option<BclIntrinsic> {
    let intr = match (ns, ty, method) {
        // System.Object
        ("System", "Object", ".ctor") => BclIntrinsic::ObjectCtor,
        ("System", "Object", "ReferenceEquals") => BclIntrinsic::ObjectReferenceEquals,
        ("System", "Object", "Equals") => BclIntrinsic::ObjectEquals,
        ("System", "Object", "GetHashCode") => BclIntrinsic::ObjectGetHashCode,
        ("System", "Object", "ToString") => BclIntrinsic::ObjectToString,
        ("System", "Object", "GetType") => BclIntrinsic::ObjectGetType,

        // System.Type
        ("System", "Type", "GetTypeFromHandle") => BclIntrinsic::TypeGetTypeFromHandle,
        ("System", "Type", "get_Name") => BclIntrinsic::TypeGetName,
        ("System", "Type", "get_FullName") => BclIntrinsic::TypeGetFullName,
        ("System", "Type", "get_Namespace") => BclIntrinsic::TypeGetNamespace,

        // System.String
        ("System", "String", "get_Length") => BclIntrinsic::StringGetLength,
        ("System", "String", "get_Chars") => BclIntrinsic::StringGetChars,
        ("System", "String", "Concat") => {
            if sig.params.len() == 1 && matches!(sig.params.first(), Some(SigType::SzArray(_))) {
                BclIntrinsic::StringConcatArray
            } else {
                BclIntrinsic::StringConcat
            }
        }
        ("System", "String", "op_Equality") => BclIntrinsic::StringOpEquality,
        ("System", "String", "op_Inequality") => BclIntrinsic::StringOpInequality,
        ("System", "String", "Equals") => BclIntrinsic::StringEquals,
        ("System", "String", "Compare" | "CompareOrdinal") => BclIntrinsic::StringCompare,
        ("System", "String", "IsNullOrEmpty") => BclIntrinsic::StringIsNullOrEmpty,
        ("System", "String", "IsNullOrWhiteSpace") => BclIntrinsic::StringIsNullOrWhiteSpace,
        ("System", "String", "Substring") => BclIntrinsic::StringSubstring,
        ("System", "String", "Contains") => BclIntrinsic::StringContains,
        ("System", "String", "StartsWith") => BclIntrinsic::StringStartsWith,
        ("System", "String", "EndsWith") => BclIntrinsic::StringEndsWith,
        ("System", "String", "IndexOf") => {
            if matches!(sig.params.first(), Some(SigType::Char)) {
                BclIntrinsic::StringIndexOfChar
            } else {
                BclIntrinsic::StringIndexOfStr
            }
        }
        ("System", "String", "ToUpper" | "ToUpperInvariant") => BclIntrinsic::StringToUpper,
        ("System", "String", "ToLower" | "ToLowerInvariant") => BclIntrinsic::StringToLower,
        ("System", "String", "Trim") => BclIntrinsic::StringTrim,
        ("System", "String", "Replace") => {
            if matches!(sig.params.first(), Some(SigType::Char)) {
                BclIntrinsic::StringReplaceChar
            } else {
                BclIntrinsic::StringReplaceStr
            }
        }
        ("System", "String", "ToCharArray") => BclIntrinsic::StringToCharArray,
        ("System", "String", "Format") => BclIntrinsic::StringFormat,
        ("System", "String", "ToString") => BclIntrinsic::StringToString,
        ("System", "String", "GetHashCode") => BclIntrinsic::StringGetHashCode,

        // System.Math / System.MathF
        ("System", "Math" | "MathF", mname) => {
            let is_f = matches!(sig.ret, SigType::R4 | SigType::R8);
            let is_i64 = matches!(sig.ret, SigType::I8 | SigType::U8);
            match mname {
                "Abs" if is_f => BclIntrinsic::MathAbsF64,
                "Abs" if is_i64 => BclIntrinsic::MathAbsI64,
                "Abs" => BclIntrinsic::MathAbsI32,
                "Min" if is_f => BclIntrinsic::MathMinF64,
                "Min" if is_i64 => BclIntrinsic::MathMinI64,
                "Min" => BclIntrinsic::MathMinI32,
                "Max" if is_f => BclIntrinsic::MathMaxF64,
                "Max" if is_i64 => BclIntrinsic::MathMaxI64,
                "Max" => BclIntrinsic::MathMaxI32,
                "Clamp" if is_f => BclIntrinsic::MathClampF64,
                "Clamp" if is_i64 => BclIntrinsic::MathClampI64,
                "Clamp" => BclIntrinsic::MathClampI32,
                "Sign" => {
                    if matches!(sig.params.first(), Some(SigType::R4 | SigType::R8)) {
                        BclIntrinsic::MathSignF64
                    } else {
                        BclIntrinsic::MathSignInt
                    }
                }
                "Sqrt" => BclIntrinsic::MathSqrt,
                "Pow" => BclIntrinsic::MathPow,
                "Exp" => BclIntrinsic::MathExp,
                "Log" => BclIntrinsic::MathLog,
                "Log10" => BclIntrinsic::MathLog10,
                "Log2" => BclIntrinsic::MathLog2,
                "Sin" => BclIntrinsic::MathSin,
                "Cos" => BclIntrinsic::MathCos,
                "Tan" => BclIntrinsic::MathTan,
                "Asin" => BclIntrinsic::MathAsin,
                "Acos" => BclIntrinsic::MathAcos,
                "Atan" => BclIntrinsic::MathAtan,
                "Atan2" => BclIntrinsic::MathAtan2,
                "Sinh" => BclIntrinsic::MathSinh,
                "Cosh" => BclIntrinsic::MathCosh,
                "Tanh" => BclIntrinsic::MathTanh,
                "Floor" => BclIntrinsic::MathFloor,
                "Ceiling" => BclIntrinsic::MathCeiling,
                "Round" => BclIntrinsic::MathRound,
                "Truncate" => BclIntrinsic::MathTruncate,
                "FusedMultiplyAdd" => BclIntrinsic::MathFma,
                _ => BclIntrinsic::MathUnknown,
            }
        }

        // System.Array
        ("System", "Array", "get_Length" | "GetLength") => BclIntrinsic::ArrayGetLength,
        ("System", "Array", "get_LongLength") => BclIntrinsic::ArrayGetLongLength,
        ("System", "Array", "get_Rank") => BclIntrinsic::ArrayGetRank,
        ("System", "Array", "Copy") => BclIntrinsic::ArrayCopy,
        ("System", "Array", "Clear") => BclIntrinsic::ArrayClear,
        ("System", "Array", "Empty") => BclIntrinsic::ArrayEmpty,

        // System.Buffer
        ("System", "Buffer", "BlockCopy") => BclIntrinsic::BufferBlockCopy,
        ("System", "Buffer", "ByteLength") => BclIntrinsic::BufferByteLength,
        ("System", "Buffer", "GetByte") => BclIntrinsic::BufferGetByte,
        ("System", "Buffer", "SetByte") => BclIntrinsic::BufferSetByte,
        ("System", "Buffer", "MemoryCopy") => BclIntrinsic::BufferMemoryCopy,

        // System.Collections.Generic.List<T>
        ("System.Collections.Generic", "List`1" | "List", "Add") => BclIntrinsic::ListAdd,
        ("System.Collections.Generic", "List`1" | "List", "get_Count") => {
            BclIntrinsic::ListGetCount
        }
        ("System.Collections.Generic", "List`1" | "List", "get_Item") => BclIntrinsic::ListGetItem,
        ("System.Collections.Generic", "List`1" | "List", "set_Item") => BclIntrinsic::ListSetItem,
        ("System.Collections.Generic", "List`1" | "List", "Clear") => BclIntrinsic::ListClear,
        ("System.Collections.Generic", "List`1" | "List", "Contains") => {
            BclIntrinsic::ListContains
        }
        ("System.Collections.Generic", "List`1" | "List", "IndexOf") => BclIntrinsic::ListIndexOf,
        ("System.Collections.Generic", "List`1" | "List", "RemoveAt") => {
            BclIntrinsic::ListRemoveAt
        }
        ("System.Collections.Generic", "List`1" | "List", "Remove") => BclIntrinsic::ListRemove,
        ("System.Collections.Generic", "List`1" | "List", "ToArray") => BclIntrinsic::ListToArray,

        // System.Collections.Generic.Dictionary<K, V>
        ("System.Collections.Generic", "Dictionary`2" | "Dictionary", "Add") => {
            BclIntrinsic::DictAdd
        }
        ("System.Collections.Generic", "Dictionary`2" | "Dictionary", "get_Count") => {
            BclIntrinsic::DictGetCount
        }
        ("System.Collections.Generic", "Dictionary`2" | "Dictionary", "get_Item") => {
            BclIntrinsic::DictGetItem
        }
        ("System.Collections.Generic", "Dictionary`2" | "Dictionary", "set_Item") => {
            BclIntrinsic::DictSetItem
        }
        ("System.Collections.Generic", "Dictionary`2" | "Dictionary", "ContainsKey") => {
            BclIntrinsic::DictContainsKey
        }
        ("System.Collections.Generic", "Dictionary`2" | "Dictionary", "TryGetValue") => {
            BclIntrinsic::DictTryGetValue
        }
        ("System.Collections.Generic", "Dictionary`2" | "Dictionary", "Remove") => {
            BclIntrinsic::DictRemove
        }
        ("System.Collections.Generic", "Dictionary`2" | "Dictionary", "Clear") => {
            BclIntrinsic::DictClear
        }

        // System.Environment
        ("System", "Environment", "get_TickCount") => BclIntrinsic::EnvGetTickCount,
        ("System", "Environment", "get_TickCount64") => BclIntrinsic::EnvGetTickCount64,
        ("System", "Environment", "get_ProcessorCount") => BclIntrinsic::EnvGetProcessorCount,
        ("System", "Environment", "get_Is64BitProcess" | "get_Is64BitOperatingSystem") => {
            BclIntrinsic::EnvGetIs64Bit
        }
        ("System", "Environment", "get_NewLine") => BclIntrinsic::EnvGetNewLine,
        ("System", "Environment", "get_CurrentDirectory") => BclIntrinsic::EnvGetCurrentDirectory,
        ("System", "Environment", "get_MachineName") => BclIntrinsic::EnvGetMachineName,
        ("System", "Environment", "get_UserName") => BclIntrinsic::EnvGetUserName,
        ("System", "Environment", "GetEnvironmentVariable") => BclIntrinsic::EnvGetEnvVar,
        ("System", "Environment", "SetEnvironmentVariable") => BclIntrinsic::EnvSetEnvVar,
        ("System", "Environment", "GetCommandLineArgs") => BclIntrinsic::EnvGetCommandLineArgs,
        ("System", "Environment", "get_ExitCode") => BclIntrinsic::EnvGetExitCode,
        ("System", "Environment", "set_ExitCode") => BclIntrinsic::EnvSetExitCode,
        ("System", "Environment", "Exit") => BclIntrinsic::EnvExit,

        // System.Runtime.InteropServices.Marshal
        ("System.Runtime.InteropServices", "Marshal", mname) => match mname {
            "AllocHGlobal" | "AllocCoTaskMem" => BclIntrinsic::MarshalAllocHGlobal,
            "ReAllocHGlobal" => BclIntrinsic::MarshalReAllocHGlobal,
            "FreeHGlobal" | "FreeCoTaskMem" => BclIntrinsic::MarshalFreeHGlobal,
            "ReadByte" => BclIntrinsic::MarshalReadByte,
            "ReadInt16" => BclIntrinsic::MarshalReadInt16,
            "ReadInt32" => BclIntrinsic::MarshalReadInt32,
            "ReadInt64" => BclIntrinsic::MarshalReadInt64,
            "ReadIntPtr" => BclIntrinsic::MarshalReadIntPtr,
            "WriteByte" => BclIntrinsic::MarshalWriteByte,
            "WriteInt16" => BclIntrinsic::MarshalWriteInt16,
            "WriteInt32" => BclIntrinsic::MarshalWriteInt32,
            "WriteInt64" => BclIntrinsic::MarshalWriteInt64,
            "WriteIntPtr" => BclIntrinsic::MarshalWriteIntPtr,
            "Copy" => {
                if matches!(sig.params.first(), Some(SigType::SzArray(_))) {
                    BclIntrinsic::MarshalCopyFromArray
                } else {
                    BclIntrinsic::MarshalCopyToArray
                }
            }
            "StringToHGlobalAnsi" | "StringToCoTaskMemAnsi" | "StringToCoTaskMemUTF8" => {
                BclIntrinsic::MarshalStringToAnsi
            }
            "StringToHGlobalUni" | "StringToCoTaskMemUni" => BclIntrinsic::MarshalStringToUni,
            "PtrToStringAnsi" | "PtrToStringUTF8" => BclIntrinsic::MarshalPtrToStringAnsi,
            "PtrToStringUni" => BclIntrinsic::MarshalPtrToStringUni,
            "GetLastWin32Error" => BclIntrinsic::MarshalGetLastWin32Error,
            "SizeOf" => BclIntrinsic::MarshalSizeOf,
            "GetFunctionPointerForDelegate" => BclIntrinsic::MarshalGetFnPtrForDelegate,
            _ => return None,
        },

        // System.Runtime.CompilerServices.RuntimeHelpers
        ("System.Runtime.CompilerServices", "RuntimeHelpers", "InitializeArray") => {
            BclIntrinsic::RuntimeHelpersInitArray
        }
        ("System.Runtime.CompilerServices", "RuntimeHelpers", "GetHashCode") => {
            BclIntrinsic::RuntimeHelpersGetHashCode
        }

        // System.Console
        ("System", "Console", "Write" | "WriteLine") => BclIntrinsic::ConsoleWrite,

        // System.Text.StringBuilder
        ("System.Text", "StringBuilder", "Append") => {
            if matches!(sig.params.first(), Some(SigType::Char)) {
                BclIntrinsic::SbAppendChar
            } else {
                BclIntrinsic::SbAppendVal
            }
        }
        ("System.Text", "StringBuilder", "ToString") => BclIntrinsic::SbToString,
        ("System.Text", "StringBuilder", "get_Length") => BclIntrinsic::SbGetLength,
        ("System.Text", "StringBuilder", "Clear") => BclIntrinsic::SbClear,

        // System.Exception
        ("System", _, "get_Message") => BclIntrinsic::ExceptionGetMessage,
        (_, name, "get_Message") if name.ends_with("Exception") => {
            BclIntrinsic::ExceptionGetMessage
        }

        // System.BitConverter
        ("System", "BitConverter", "SingleToInt32Bits") => BclIntrinsic::BitConvSingleToI32,
        ("System", "BitConverter", "Int32BitsToSingle") => BclIntrinsic::BitConvI32ToSingle,
        ("System", "BitConverter", "DoubleToInt64Bits") => BclIntrinsic::BitConvDoubleToI64,
        ("System", "BitConverter", "Int64BitsToDouble") => BclIntrinsic::BitConvI64ToDouble,

        // System.IntPtr / UIntPtr
        ("System", "IntPtr" | "UIntPtr", "ToInt32") => BclIntrinsic::IntPtrToI32,
        ("System", "IntPtr" | "UIntPtr", "ToInt64") => BclIntrinsic::IntPtrToI64,
        ("System", "IntPtr" | "UIntPtr", "ToPointer") => BclIntrinsic::IntPtrToPtr,
        ("System", "IntPtr" | "UIntPtr", "op_Explicit") => match sig.ret {
            SigType::I4 | SigType::U4 => BclIntrinsic::IntPtrExplicitToI32,
            SigType::I8 | SigType::U8 => BclIntrinsic::IntPtrExplicitToI64,
            _ => BclIntrinsic::IntPtrExplicitToNative,
        },
        ("System", "IntPtr" | "UIntPtr", "op_Equality") => BclIntrinsic::IntPtrOpEq,
        ("System", "IntPtr" | "UIntPtr", "op_Inequality") => BclIntrinsic::IntPtrOpNeq,

        // Primitives & Convert
        ("System", "Int32", "Parse") => BclIntrinsic::Int32Parse,
        ("System", "Int64", "Parse") => BclIntrinsic::Int64Parse,
        ("System", "Int32", "ToString") => BclIntrinsic::PrimToStringI32,
        ("System", "UInt32", "ToString") => BclIntrinsic::PrimToStringU32,
        ("System", "Int64", "ToString") => BclIntrinsic::PrimToStringI64,
        ("System", "UInt64", "ToString") => BclIntrinsic::PrimToStringU64,
        ("System", "Single", "ToString") => BclIntrinsic::PrimToStringF32,
        ("System", "Double", "ToString") => BclIntrinsic::PrimToStringF64,
        ("System", "Boolean", "ToString") => BclIntrinsic::PrimToStringBool,
        ("System", "Byte", "ToString") => BclIntrinsic::PrimToStringU8,
        ("System", "Convert", "ToInt32") => BclIntrinsic::ConvertToI32,
        ("System", "Convert", "ToInt64") => BclIntrinsic::ConvertToI64,
        ("System", "Convert", "ToDouble") => BclIntrinsic::ConvertToF64,
        ("System", "Convert", "ToString") => BclIntrinsic::ConvertToString,

        // System.GC
        (
            "System",
            "GC",
            "Collect" | "SuppressFinalize" | "KeepAlive" | "WaitForPendingFinalizers",
        ) => BclIntrinsic::GcNoop,

        // Base .ctor fallback
        (_, _, ".ctor") => BclIntrinsic::BaseCtorNoop,

        _ => return None,
    };
    Some(intr)
}

fn classify_bcl_newobj(vm: &ClrVm, ns: &str, ty: &str, sig: &MethodSig) -> BclNewObjIntrinsic {
    match (ns, ty) {
        ("System", "Object") => BclNewObjIntrinsic::Object,
        ("System", "String") => match sig.params.len() {
            1 => match sig.params.first() {
                Some(SigType::SzArray(_)) => BclNewObjIntrinsic::StringFromCharArray,
                Some(SigType::Ptr(_)) => BclNewObjIntrinsic::StringFromCharPtr,
                _ => BclNewObjIntrinsic::StringEmpty,
            },
            2 => BclNewObjIntrinsic::StringFromCharCount,
            3 => BclNewObjIntrinsic::StringFromCharArraySlice,
            _ => BclNewObjIntrinsic::StringEmpty,
        },
        ("System", "IntPtr" | "UIntPtr") => BclNewObjIntrinsic::IntPtr,
        ("System.Text", "StringBuilder") => {
            if sig.params.len() == 1 && matches!(sig.params.first(), Some(SigType::String)) {
                BclNewObjIntrinsic::StringBuilderFromString
            } else {
                BclNewObjIntrinsic::StringBuilderEmpty
            }
        }
        ("System.Collections.Generic", "List`1" | "List") => {
            if sig.params.len() == 1 && matches!(sig.params.first(), Some(SigType::I4)) {
                BclNewObjIntrinsic::ListWithCapacity
            } else {
                BclNewObjIntrinsic::ListEmpty
            }
        }
        ("System.Collections.Generic", "Dictionary`2" | "Dictionary") => {
            if sig.params.len() == 1 && matches!(sig.params.first(), Some(SigType::I4)) {
                BclNewObjIntrinsic::DictionaryWithCapacity
            } else {
                BclNewObjIntrinsic::DictionaryEmpty
            }
        }
        (_, name) if name.ends_with("Exception") => {
            let cid = vm
                .find_class_by_name_any(ns, ty)
                .unwrap_or(vm.core.exception);
            BclNewObjIntrinsic::Exception(cid)
        }
        _ => {
            let cid = vm.find_class_by_name_any(ns, ty).unwrap_or(vm.core.object);
            BclNewObjIntrinsic::GenericObject(cid)
        }
    }
}

impl ClrVm {
    /// Cached `TypeDef`/`TypeRef`/`TypeSpec` token -> `class_id`.
    #[inline]
    pub fn resolve_type_token_cached(&mut self, asm_id: usize, token: u32) -> Option<usize> {
        let tbl = (token >> 24) as u8;
        let rid = (token & 0x00FF_FFFF) as usize;
        if rid == 0 {
            return None;
        }
        if tbl == table::TYPE_DEF {
            return self.assemblies.get(asm_id)?.class_ids.get(rid - 1).copied();
        }
        if let Some(&cid) = self.assemblies.get(asm_id)?.type_cache.get(&token) {
            return Some(cid);
        }
        let cid = self.resolve_type_token(asm_id, token)?;
        self.assemblies[asm_id].type_cache.insert(token, cid);
        Some(cid)
    }

    /// Interned `#US` user string lookup (`ldstr`). Allocates each literal at most once per assembly.
    #[inline]
    pub(crate) fn intern_user_string(&mut self, asm_id: usize, token: u32) -> usize {
        if let Some(&obj) = self.assemblies[asm_id].user_strings.get(&token) {
            return obj;
        }
        let u16s = self.assemblies[asm_id]
            .image
            .as_ref()
            .map(|img| img.get_user_string(token))
            .unwrap_or_default();
        let obj = self.alloc_string_utf16(&u16s);
        self.assemblies[asm_id].user_strings.insert(token, obj);
        obj
    }

    /// Cached `StandAloneSig` method signature lookup (for `calli`).
    pub(crate) fn resolve_standalone_sig_cached(
        &mut self,
        asm_id: usize,
        sig_tok: u32,
    ) -> Arc<MethodSig> {
        if let Some(sig) = self.assemblies[asm_id].standalone_sig_cache.get(&sig_tok) {
            return sig.clone();
        }
        let sig = Arc::new(
            self.assemblies[asm_id]
                .image
                .as_ref()
                .and_then(|img| img.parse_standalone_method_sig(sig_tok))
                .unwrap_or(MethodSig {
                    has_this: false,
                    explicit_this: false,
                    ret: SigType::Void,
                    params: Vec::new(),
                }),
        );
        self.assemblies[asm_id]
            .standalone_sig_cache
            .insert(sig_tok, sig.clone());
        sig
    }

    /// Classify a `MethodInfo` by `method_id`, caching the classification on `MethodInfo`.
    pub(crate) fn classify_method_cached(&mut self, mid: usize) -> Option<ResolvedTarget> {
        if let Some(t) = self.methods.get(mid)?.cached_target.as_ref() {
            return Some(t.clone());
        }
        let target = self.classify_method_uncached(mid)?;
        self.methods[mid].cached_target = Some(target.clone());
        Some(target)
    }

    fn classify_method_uncached(&self, mid: usize) -> Option<ResolvedTarget> {
        let m = self.methods.get(mid)?;
        let cls = &self.classes[m.class_id];
        if cls.is_delegate && m.name == "Invoke" {
            return Some(ResolvedTarget::DelegateInvoke);
        }
        if (m.flags & method_attr::PINVOKE_IMPL) != 0 && m.pinvoke.is_some() {
            return Some(ResolvedTarget::PInvoke(mid));
        }
        if (m.impl_flags & method_impl_attr::INTERNAL_CALL) != 0 || m.body.is_none() {
            if let Some(fn_ptr) = self.lookup_internal_call(m.class_id, &m.name) {
                return Some(ResolvedTarget::InternalCall {
                    fn_ptr,
                    ret_sig: m.sig.ret.clone(),
                });
            }
            if m.body.is_none() {
                if let Some(intr) = classify_bcl_method(&cls.namespace, &cls.name, &m.name, &m.sig)
                {
                    return Some(ResolvedTarget::Bcl(intr));
                }
                return Some(ResolvedTarget::UnsupportedBcl(
                    format!("{}.{}::{}", cls.namespace, cls.name, m.name).into(),
                ));
            }
        }
        Some(ResolvedTarget::Managed(mid))
    }

    /// Resolve a `MemberRef` or `MethodSpec` call token once (single `#Blob` signature parse)
    /// and cache it on `LoadedAssembly`.
    pub(crate) fn resolve_call_entry_cached(
        &mut self,
        asm_id: usize,
        token: u32,
    ) -> Option<CachedCallEntry> {
        if let Some(entry) = self.assemblies.get(asm_id)?.call_cache.get(&token) {
            return Some(entry.clone());
        }
        let entry = self.resolve_call_entry_uncached(asm_id, token)?;
        self.assemblies[asm_id].call_cache.insert(token, entry.clone());
        Some(entry)
    }

    fn resolve_call_entry_uncached(&mut self, asm_id: usize, token: u32) -> Option<CachedCallEntry> {
        let tbl = (token >> 24) as u8;
        let rid = (token & 0x00FF_FFFF) as usize;
        if rid == 0 {
            return None;
        }
        match tbl {
            table::METHOD_SPEC => {
                let underlying = self
                    .assemblies
                    .get(asm_id)?
                    .image
                    .as_ref()?
                    .method_specs
                    .get(rid - 1)?
                    .method;
                self.resolve_call_entry_uncached(asm_id, underlying)
            }
            table::METHOD_DEF => {
                let mid = *self.assemblies.get(asm_id)?.method_ids.get(rid - 1)?;
                let target = self.classify_method_cached(mid)?;
                let m = &self.methods[mid];
                Some(CachedCallEntry {
                    argc: m.argc,
                    has_this: m.sig.has_this,
                    param_count: m.sig.params.len() as i32,
                    target,
                    method_name: Arc::from(m.name.as_str()),
                })
            }
            table::MEMBER_REF => {
                let (mname, msig, pns, pname) = {
                    let img = self.assemblies.get(asm_id)?.image.as_ref()?;
                    let mr = img.member_refs.get(rid - 1)?;
                    let mname = img.get_string(mr.name).to_string();
                    let msig = img.parse_method_sig(mr.signature);
                    let (pns, pname) = img
                        .resolve_type_token_name(mr.class)
                        .unwrap_or_else(|| (String::new(), String::new()));
                    (mname, msig, pns, pname)
                };
                let pcnt = msig.params.len() as i32;
                let argc =
                    msig.params.len() + usize::from(msig.has_this && !msig.explicit_this);

                let mut resolved_target = None;
                if let Some(cid) = self.find_class_by_name_any(&pns, &pname) {
                    if self.classes[cid].is_delegate && mname == "Invoke" {
                        resolved_target = Some(ResolvedTarget::DelegateInvoke);
                    } else if let Some(mid) = self.find_method_in_class(cid, &mname, pcnt) {
                        resolved_target = self.classify_method_cached(mid);
                    } else if let Some(fn_ptr) = self.lookup_internal_call(cid, &mname) {
                        resolved_target = Some(ResolvedTarget::InternalCall {
                            fn_ptr,
                            ret_sig: msig.ret.clone(),
                        });
                    }
                }

                let target = resolved_target.unwrap_or_else(|| {
                    if let Some(intr) = classify_bcl_method(&pns, &pname, &mname, &msig) {
                        ResolvedTarget::Bcl(intr)
                    } else {
                        ResolvedTarget::UnsupportedBcl(
                            format!("{pns}.{pname}::{mname}").into(),
                        )
                    }
                });

                Some(CachedCallEntry {
                    argc,
                    has_this: msig.has_this,
                    param_count: pcnt,
                    target,
                    method_name: Arc::from(mname.as_str()),
                })
            }
            _ => None,
        }
    }

    /// Cached virtual override resolution for `(asm_id, token, runtime_class_id)` -> `method_id`.
    pub(crate) fn resolve_virtual_override_cached(
        &mut self,
        asm_id: usize,
        token: u32,
        rcid: usize,
        method_name: &str,
        param_count: i32,
    ) -> Option<usize> {
        if let Some(&ov) = self.assemblies[asm_id].vdispatch_cache.get(&(token, rcid)) {
            return (ov != usize::MAX).then_some(ov);
        }
        let found = self.find_method_in_class(rcid, method_name, param_count);
        self.assemblies[asm_id]
            .vdispatch_cache
            .insert((token, rcid), found.unwrap_or(usize::MAX));
        found
    }

    /// Resolve a `newobj` constructor token once and cache it per assembly.
    pub(crate) fn resolve_newobj_cached(
        &mut self,
        asm_id: usize,
        token: u32,
    ) -> Option<CachedNewObjEntry> {
        if let Some(&entry) = self.assemblies.get(asm_id)?.newobj_cache.get(&token) {
            return Some(entry);
        }
        let entry = self.resolve_newobj_uncached(asm_id, token)?;
        self.assemblies[asm_id].newobj_cache.insert(token, entry);
        Some(entry)
    }

    fn resolve_newobj_uncached(&mut self, asm_id: usize, token: u32) -> Option<CachedNewObjEntry> {
        let tbl = (token >> 24) as u8;
        let rid = (token & 0x00FF_FFFF) as usize;
        if rid == 0 {
            return None;
        }
        match tbl {
            table::METHOD_SPEC => {
                let underlying = self
                    .assemblies
                    .get(asm_id)?
                    .image
                    .as_ref()?
                    .method_specs
                    .get(rid - 1)?
                    .method;
                self.resolve_newobj_uncached(asm_id, underlying)
            }
            table::METHOD_DEF => {
                let ctor_mid = *self.assemblies.get(asm_id)?.method_ids.get(rid - 1)?;
                let argc = self.methods[ctor_mid].sig.params.len();
                let cls_id = self.methods[ctor_mid].class_id;
                let kind = if self.classes[cls_id].is_delegate {
                    ResolvedNewObj::Delegate { class_id: cls_id }
                } else if self.classes[cls_id].is_value_type {
                    ResolvedNewObj::ManagedValueType {
                        ctor_mid,
                        class_id: cls_id,
                        size: self.value_type_size(cls_id),
                    }
                } else {
                    ResolvedNewObj::ManagedClass {
                        ctor_mid,
                        class_id: cls_id,
                    }
                };
                Some(CachedNewObjEntry { argc, kind })
            }
            table::MEMBER_REF => {
                let (msig, pns, pname) = {
                    let img = self.assemblies.get(asm_id)?.image.as_ref()?;
                    let mr = img.member_refs.get(rid - 1)?;
                    let msig = img.parse_method_sig(mr.signature);
                    let (pns, pname) = img
                        .resolve_type_token_name(mr.class)
                        .unwrap_or_else(|| (String::new(), String::new()));
                    (msig, pns, pname)
                };
                let argc = msig.params.len();
                if let Some(cid) = self.find_class_by_name_any(&pns, &pname) {
                    if self.classes[cid].is_delegate {
                        return Some(CachedNewObjEntry {
                            argc,
                            kind: ResolvedNewObj::Delegate { class_id: cid },
                        });
                    }
                    if let Some(ctor_mid) = self.find_method_in_class(cid, ".ctor", argc as i32) {
                        if self.methods[ctor_mid].body.is_some() {
                            let kind = if self.classes[cid].is_value_type {
                                ResolvedNewObj::ManagedValueType {
                                    ctor_mid,
                                    class_id: cid,
                                    size: self.value_type_size(cid),
                                }
                            } else {
                                ResolvedNewObj::ManagedClass {
                                    ctor_mid,
                                    class_id: cid,
                                }
                            };
                            return Some(CachedNewObjEntry { argc, kind });
                        }
                    }
                }
                let intr = classify_bcl_newobj(self, &pns, &pname, &msig);
                Some(CachedNewObjEntry {
                    argc,
                    kind: ResolvedNewObj::Bcl(intr),
                })
            }
            _ => None,
        }
    }

    /// Resolve a `Field` or `MemberRef` field token (including BCL static constants) with O(1) caching.
    #[inline]
    pub(crate) fn resolve_field_cached(
        &mut self,
        asm_id: usize,
        token: u32,
    ) -> Option<ResolvedField> {
        let tbl = (token >> 24) as u8;
        let rid = (token & 0x00FF_FFFF) as usize;
        if rid == 0 {
            return None;
        }
        if tbl == table::FIELD {
            let fid = *self.assemblies.get(asm_id)?.field_ids.get(rid - 1)?;
            let f = self.fields.get(fid)?;
            return Some(ResolvedField::Field {
                field_id: fid,
                class_id: f.class_id,
                struct_offset: f.struct_offset,
                size: f.size,
                is_static: f.is_static,
                kind: f.kind,
            });
        }
        if let Some(&rf) = self.assemblies.get(asm_id)?.field_cache.get(&token) {
            return Some(rf);
        }
        let rf = self.resolve_field_member_ref_uncached(asm_id, rid)?;
        self.assemblies[asm_id].field_cache.insert(token, rf);
        Some(rf)
    }

    fn resolve_field_member_ref_uncached(
        &mut self,
        asm_id: usize,
        rid: usize,
    ) -> Option<ResolvedField> {
        let (fname, pns, pname) = {
            let img = self.assemblies.get(asm_id)?.image.as_ref()?;
            let mr = img.member_refs.get(rid - 1)?;
            let fname = img.get_string(mr.name).to_string();
            let (pns, pname) = img.resolve_type_token_name(mr.class)?;
            (fname, pns, pname)
        };
        if let Some(cid) = self.find_class_by_name_any(&pns, &pname) {
            if let Some(fid) = self.find_field_in_class(cid, &fname) {
                let f = &self.fields[fid];
                return Some(ResolvedField::Field {
                    field_id: fid,
                    class_id: f.class_id,
                    struct_offset: f.struct_offset,
                    size: f.size,
                    is_static: f.is_static,
                    kind: f.kind,
                });
            }
        }
        let bcl_const = match (pns.as_str(), pname.as_str(), fname.as_str()) {
            ("System", "String", "Empty") => CilVal::Obj(self.empty_string()),
            ("System", "IntPtr" | "UIntPtr", "Zero") => CilVal::NativeInt(0),
            ("System", "Math", "PI") => CilVal::F64(std::f64::consts::PI),
            ("System", "Math", "E") => CilVal::F64(std::f64::consts::E),
            ("System", "MathF", "PI") => CilVal::F64(std::f32::consts::PI as f64),
            ("System", "MathF", "E") => CilVal::F64(std::f32::consts::E as f64),
            ("System", "BitConverter", "IsLittleEndian") => CilVal::I32(1),
            _ => return None,
        };
        Some(ResolvedField::BclConst(bcl_const))
    }

    /// Resolve a `ldftn`/`ldvirtftn` token to a `method_id`.
    pub(crate) fn resolve_method_id_for_ftn(
        &mut self,
        asm_id: usize,
        token: u32,
        virt_recv_obj: Option<usize>,
    ) -> usize {
        let tbl = (token >> 24) as u8;
        let rid = (token & 0x00FF_FFFF) as usize;
        if rid == 0 {
            return 0;
        }
        if tbl == table::METHOD_DEF {
            let Some(&base_mid) = self
                .assemblies
                .get(asm_id)
                .and_then(|a| a.method_ids.get(rid - 1))
            else {
                return 0;
            };
            if let Some(recv) = virt_recv_obj {
                if self.methods[base_mid].is_virtual {
                    if let Some(rcid) = self.object_class_id(recv) {
                        if rcid != self.methods[base_mid].class_id {
                            let mname = self.methods[base_mid].name.clone();
                            let pcnt = self.methods[base_mid].sig.params.len() as i32;
                            if let Some(ov) = self.resolve_virtual_override_cached(
                                asm_id, token, rcid, &mname, pcnt,
                            ) {
                                return ov;
                            }
                        }
                    }
                }
            }
            return base_mid;
        }
        let Some(entry) = self.resolve_call_entry_cached(asm_id, token) else {
            return 0;
        };
        if let Some(recv) = virt_recv_obj {
            if let Some(rcid) = self.object_class_id(recv) {
                if rcid >= self.corlib_class_count {
                    if let Some(ov) = self.resolve_virtual_override_cached(
                        asm_id,
                        token,
                        rcid,
                        &entry.method_name,
                        entry.param_count,
                    ) {
                        return ov;
                    }
                }
            }
        }
        match entry.target {
            ResolvedTarget::Managed(mid) | ResolvedTarget::PInvoke(mid) => mid,
            _ => 0,
        }
    }

    /// Invalidate cached call/field/type resolution entries whenever assemblies or internal calls
    /// are added dynamically so lookups never observe stale results.
    pub(crate) fn invalidate_resolution_caches(&mut self) {
        for m in &mut self.methods {
            m.cached_target = None;
        }
        for asm in &mut self.assemblies {
            asm.call_cache.clear();
            asm.newobj_cache.clear();
            asm.field_cache.clear();
            asm.type_cache.clear();
            asm.vdispatch_cache.clear();
        }
    }
}
