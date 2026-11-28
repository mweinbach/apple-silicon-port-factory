//! Builtin `eossdk-win64-shipping.dll` (Epic Online Services SDK, offline mode).
//!
//! Replaces the 19.3 MB game-dir PE DLL via `BUILTIN_OVERRIDES`:
//! * `EOS_Initialize`, `EOS_Logging_SetCallback`, `EOS_Logging_SetLogLevel`,
//!   and `EOS_Shutdown` return `EOS_Success` (`0`).
//! * `EOS_Platform_Create` returns a non-null opaque handle and
//!   `EOS_Platform_CheckForLauncherAndRestart` returns `EOS_NoChange` (`0x14`)
//!   so the game runs directly without requesting a launcher restart.
//! * Sub-interface getters (`GetAuthInterface`, `GetConnectInterface`,
//!   `GetEcomInterface`, `GetStatsInterface`, `GetAchievementsInterface`,
//!   `GetUserInfoInterface`) return non-null opaque handles.
//! * Async operations (`EOS_Auth_Login`, `EOS_Connect_Login`,
//!   `EOS_Connect_CreateUser`, `EOS_Ecom_QueryOwnership`,
//!   `EOS_Achievements_QueryDefinitions`, `EOS_Achievements_QueryPlayerAchievements`,
//!   `EOS_Achievements_UnlockAchievements`, `EOS_Stats_IngestStat`) queue their
//!   `(completion_delegate, client_data)` pair and fire it from
//!   `EOS_Platform_Tick` with `EOS_NoConnection` (`6`), matching the real EOS
//!   tick-driven completion contract so caller futures resolve cleanly.

use std::ffi::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

pub const EOS_SUCCESS: i32 = 0;
pub const EOS_INVALID_PARAMETERS: i32 = 3;
pub const EOS_LIMIT_EXCEEDED: i32 = 4;
pub const EOS_NO_CONNECTION: i32 = 6;
pub const EOS_NOT_FOUND: i32 = 18;
pub const EOS_NO_CHANGE: i32 = 20;

/// Maximum UTF-8 byte length of an `EOS_EpicAccountId` or `EOS_ProductUserId`
/// string (excluding NUL), matching `eos_common.h`.
pub const EOS_EPICACCOUNTID_MAX_LENGTH: usize = 32;
pub const EOS_PRODUCTUSERID_MAX_LENGTH: usize = 32;

const H_PLATFORM: usize = 0xE050_0001;
const H_AUTH: usize = 0xE050_0002;
const H_CONNECT: usize = 0xE050_0003;
const H_ECOM: usize = 0xE050_0004;
const H_STATS: usize = 0xE050_0005;
const H_ACHIEVEMENTS: usize = 0xE050_0006;
const H_USERINFO: usize = 0xE050_0007;
const H_LEADERBOARDS: usize = 0xE050_0008;
const H_LOBBY: usize = 0xE050_0009;
const H_FRIENDS: usize = 0xE050_000A;
const H_ACCOUNT_BASE: usize = 0xE050_1000;
const H_PUID_BASE: usize = 0xE050_2000;

static NEXT_NOTIFY_ID: AtomicU64 = AtomicU64::new(1);
static PENDING_CALLBACKS: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());
static ACCOUNT_IDS: Mutex<Vec<String>> = Mutex::new(Vec::new());
static PRODUCT_USER_IDS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Generic layout shared by all EOS `*_CallbackInfo` structs (`ResultCode` at
/// `+0x00`, `ClientData` at `+0x08`, followed by zeroed handle/count fields).
#[repr(C)]
struct GenericCallbackInfo {
    result_code: i32,
    _pad: u32,
    client_data: usize,
    extra: [u64; 8],
}

fn queue_callback(client_data: usize, completion_delegate: usize) {
    if completion_delegate != 0 {
        PENDING_CALLBACKS.lock().unwrap().push((completion_delegate, client_data));
    }
}

pub static EXPORTS: &[crate::Export] = exports![
    EOS_Initialize,
    EOS_Shutdown,
    EOS_Logging_SetCallback,
    EOS_Logging_SetLogLevel,
    EOS_Platform_Create,
    EOS_Platform_Release,
    EOS_Platform_Tick,
    EOS_Platform_CheckForLauncherAndRestart,
    EOS_Platform_GetAuthInterface,
    EOS_Platform_GetConnectInterface,
    EOS_Platform_GetEcomInterface,
    EOS_Platform_GetStatsInterface,
    EOS_Platform_GetAchievementsInterface,
    EOS_Platform_GetUserInfoInterface,
    EOS_Platform_GetLeaderboardsInterface,
    EOS_Platform_GetLobbyInterface,
    EOS_Platform_GetFriendsInterface,
    EOS_EResult_ToString,
    EOS_Auth_Login,
    EOS_Auth_GetLoggedInAccountsCount,
    EOS_Auth_GetLoggedInAccountByIndex,
    EOS_Auth_GetLoginStatus,
    EOS_Auth_CopyUserAuthToken,
    EOS_Auth_Token_Release,
    EOS_Auth_AddNotifyLoginStatusChanged,
    EOS_Auth_RemoveNotifyLoginStatusChanged,
    EOS_Connect_Login,
    EOS_Connect_CreateUser,
    EOS_Connect_QueryExternalAccountMappings,
    EOS_Connect_AddNotifyAuthExpiration,
    EOS_Connect_RemoveNotifyAuthExpiration,
    EOS_Ecom_QueryOwnership,
    EOS_Stats_IngestStat,
    EOS_Stats_QueryStats,
    EOS_Stats_CopyStatByName,
    EOS_Stats_Stat_Release,
    EOS_Achievements_QueryDefinitions,
    EOS_Achievements_GetAchievementDefinitionCount,
    EOS_Achievements_CopyAchievementDefinitionV2ByIndex,
    EOS_Achievements_DefinitionV2_Release,
    EOS_Achievements_QueryPlayerAchievements,
    EOS_Achievements_CopyPlayerAchievementByAchievementId,
    EOS_Achievements_PlayerAchievement_Release,
    EOS_Achievements_UnlockAchievements,
    EOS_Achievements_AddNotifyAchievementsUnlockedV2,
    EOS_Achievements_RemoveNotifyAchievementsUnlocked,
    EOS_UserInfo_QueryUserInfo,
    EOS_UserInfo_QueryUserInfoByExternalAccount,
    EOS_UserInfo_CopyUserInfo,
    EOS_UserInfo_Release,
    EOS_Friends_QueryFriends,
    EOS_Leaderboards_QueryLeaderboardUserScores,
    EOS_Leaderboards_QueryLeaderboardRanks,
    EOS_PresenceModification_Release,
    EOS_EpicAccountId_IsValid,
    EOS_EpicAccountId_FromString,
    EOS_EpicAccountId_ToString,
    EOS_ProductUserId_IsValid,
    EOS_ProductUserId_FromString,
    EOS_ProductUserId_ToString,
];

pub unsafe extern "win64" fn EOS_Initialize(_options: *const c_void) -> i32 {
    log::info!("eossdk (builtin): EOS_Initialize -> EOS_Success (offline)");
    EOS_SUCCESS
}

pub unsafe extern "win64" fn EOS_Shutdown() -> i32 {
    PENDING_CALLBACKS.lock().unwrap().clear();
    EOS_SUCCESS
}

pub unsafe extern "win64" fn EOS_Logging_SetCallback(_callback: *const c_void) -> i32 {
    EOS_SUCCESS
}

pub unsafe extern "win64" fn EOS_Logging_SetLogLevel(_category: i32, _level: i32) -> i32 {
    EOS_SUCCESS
}

pub unsafe extern "win64" fn EOS_Platform_Create(_options: *const c_void) -> usize {
    log::info!("eossdk (builtin): EOS_Platform_Create -> {H_PLATFORM:#x} (offline)");
    H_PLATFORM
}

pub unsafe extern "win64" fn EOS_Platform_Release(_handle: usize) {
    PENDING_CALLBACKS.lock().unwrap().clear();
}

pub unsafe extern "win64" fn EOS_Platform_Tick(_handle: usize) {
    let pending: Vec<(usize, usize)> = {
        let mut q = PENDING_CALLBACKS.lock().unwrap();
        if q.is_empty() {
            return;
        }
        std::mem::take(&mut *q)
    };
    for (cb, client_data) in pending {
        let info = GenericCallbackInfo {
            result_code: EOS_NO_CONNECTION,
            _pad: 0,
            client_data,
            extra: [0; 8],
        };
        crate::call_guest8(cb, &info as *const GenericCallbackInfo as u64, 0, 0, 0, 0, 0, 0, 0);
    }
}

pub unsafe extern "win64" fn EOS_Platform_CheckForLauncherAndRestart(_handle: usize) -> i32 {
    EOS_NO_CHANGE
}

pub unsafe extern "win64" fn EOS_Platform_GetAuthInterface(_handle: usize) -> usize {
    H_AUTH
}

pub unsafe extern "win64" fn EOS_Platform_GetConnectInterface(_handle: usize) -> usize {
    H_CONNECT
}

pub unsafe extern "win64" fn EOS_Platform_GetEcomInterface(_handle: usize) -> usize {
    H_ECOM
}

pub unsafe extern "win64" fn EOS_Platform_GetStatsInterface(_handle: usize) -> usize {
    H_STATS
}

pub unsafe extern "win64" fn EOS_Platform_GetAchievementsInterface(_handle: usize) -> usize {
    H_ACHIEVEMENTS
}

pub unsafe extern "win64" fn EOS_Platform_GetUserInfoInterface(_handle: usize) -> usize {
    H_USERINFO
}

pub unsafe extern "win64" fn EOS_Platform_GetLeaderboardsInterface(_handle: usize) -> usize {
    H_LEADERBOARDS
}

pub unsafe extern "win64" fn EOS_Platform_GetLobbyInterface(_handle: usize) -> usize {
    H_LOBBY
}

pub unsafe extern "win64" fn EOS_Platform_GetFriendsInterface(_handle: usize) -> usize {
    H_FRIENDS
}

pub unsafe extern "win64" fn EOS_EResult_ToString(result: i32) -> *const u8 {
    let s: &'static [u8] = match result {
        EOS_SUCCESS => b"EOS_Success\0",
        EOS_INVALID_PARAMETERS | 10 => b"EOS_InvalidParameters\0",
        EOS_LIMIT_EXCEEDED | 22 => b"EOS_LimitExceeded\0",
        EOS_NO_CONNECTION | 1 => b"EOS_NoConnection\0",
        EOS_NOT_FOUND => b"EOS_NotFound\0",
        EOS_NO_CHANGE => b"EOS_NoChange\0",
        2 => b"EOS_InvalidCredentials\0",
        5 => b"EOS_AccessDenied\0",
        16 => b"EOS_NotImplemented\0",
        17 => b"EOS_Canceled\0",
        21 => b"EOS_VersionMismatch\0",
        _ => b"EOS_UnexpectedError\0",
    };
    s.as_ptr()
}

pub unsafe extern "win64" fn EOS_Auth_Login(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_Auth_GetLoggedInAccountsCount(_handle: usize) -> i32 {
    0
}

pub unsafe extern "win64" fn EOS_Auth_GetLoggedInAccountByIndex(_handle: usize, _index: i32) -> usize {
    0
}

pub unsafe extern "win64" fn EOS_Auth_GetLoginStatus(_handle: usize, _account_id: usize) -> i32 {
    0 // EOS_LS_NotLoggedIn
}

pub unsafe extern "win64" fn EOS_Auth_CopyUserAuthToken(
    _handle: usize,
    _options: *const c_void,
    _account_id: usize,
    out_token: *mut usize,
) -> i32 {
    if !out_token.is_null() {
        *out_token = 0;
    }
    EOS_NOT_FOUND
}

pub unsafe extern "win64" fn EOS_Auth_Token_Release(_token: usize) {}

pub unsafe extern "win64" fn EOS_Auth_AddNotifyLoginStatusChanged(
    _handle: usize,
    _options: *const c_void,
    _client_data: usize,
    _notification: usize,
) -> u64 {
    NEXT_NOTIFY_ID.fetch_add(1, Ordering::Relaxed)
}

pub unsafe extern "win64" fn EOS_Auth_RemoveNotifyLoginStatusChanged(_handle: usize, _in_id: u64) {}

pub unsafe extern "win64" fn EOS_Connect_Login(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_Connect_CreateUser(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_Connect_QueryExternalAccountMappings(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_Connect_AddNotifyAuthExpiration(
    _handle: usize,
    _options: *const c_void,
    _client_data: usize,
    _notification: usize,
) -> u64 {
    NEXT_NOTIFY_ID.fetch_add(1, Ordering::Relaxed)
}

pub unsafe extern "win64" fn EOS_Connect_RemoveNotifyAuthExpiration(_handle: usize, _in_id: u64) {}

pub unsafe extern "win64" fn EOS_Ecom_QueryOwnership(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_Stats_IngestStat(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_Stats_QueryStats(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_Stats_CopyStatByName(
    _handle: usize,
    _options: *const c_void,
    out_stat: *mut usize,
) -> i32 {
    if !out_stat.is_null() {
        std::ptr::write_unaligned(out_stat, 0);
    }
    EOS_NOT_FOUND
}

pub unsafe extern "win64" fn EOS_Stats_Stat_Release(_stat: usize) {}

pub unsafe extern "win64" fn EOS_Achievements_QueryDefinitions(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_Achievements_GetAchievementDefinitionCount(
    _handle: usize,
    _options: *const c_void,
) -> u32 {
    0
}

pub unsafe extern "win64" fn EOS_Achievements_CopyAchievementDefinitionV2ByIndex(
    _handle: usize,
    _options: *const c_void,
    out_definition: *mut usize,
) -> i32 {
    if !out_definition.is_null() {
        *out_definition = 0;
    }
    EOS_NOT_FOUND
}

pub unsafe extern "win64" fn EOS_Achievements_DefinitionV2_Release(_definition: usize) {}

pub unsafe extern "win64" fn EOS_Achievements_QueryPlayerAchievements(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_Achievements_CopyPlayerAchievementByAchievementId(
    _handle: usize,
    _options: *const c_void,
    out_achievement: *mut usize,
) -> i32 {
    if !out_achievement.is_null() {
        *out_achievement = 0;
    }
    EOS_NOT_FOUND
}

pub unsafe extern "win64" fn EOS_Achievements_PlayerAchievement_Release(_achievement: usize) {}

pub unsafe extern "win64" fn EOS_Achievements_UnlockAchievements(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_Achievements_AddNotifyAchievementsUnlockedV2(
    _handle: usize,
    _options: *const c_void,
    _client_data: usize,
    _notification: usize,
) -> u64 {
    NEXT_NOTIFY_ID.fetch_add(1, Ordering::Relaxed)
}

pub unsafe extern "win64" fn EOS_Achievements_RemoveNotifyAchievementsUnlocked(
    _handle: usize,
    _in_id: u64,
) {
}

pub unsafe extern "win64" fn EOS_UserInfo_QueryUserInfo(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_UserInfo_QueryUserInfoByExternalAccount(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_UserInfo_CopyUserInfo(
    _handle: usize,
    _options: *const c_void,
    out_user_info: *mut usize,
) -> i32 {
    if !out_user_info.is_null() {
        std::ptr::write_unaligned(out_user_info, 0);
    }
    EOS_NOT_FOUND
}

pub unsafe extern "win64" fn EOS_UserInfo_Release(_user_info: usize) {}

pub unsafe extern "win64" fn EOS_Friends_QueryFriends(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_Leaderboards_QueryLeaderboardUserScores(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_Leaderboards_QueryLeaderboardRanks(
    _handle: usize,
    _options: *const c_void,
    client_data: usize,
    completion_delegate: usize,
) {
    queue_callback(client_data, completion_delegate);
}

pub unsafe extern "win64" fn EOS_PresenceModification_Release(_handle: usize) {}

fn lookup_account_id(account_id: usize) -> Option<String> {
    if !(H_ACCOUNT_BASE..H_PUID_BASE).contains(&account_id) {
        return None;
    }
    let idx = account_id - H_ACCOUNT_BASE;
    ACCOUNT_IDS.lock().unwrap().get(idx).cloned()
}

pub unsafe extern "win64" fn EOS_EpicAccountId_IsValid(account_id: usize) -> i32 {
    if !(H_ACCOUNT_BASE..H_PUID_BASE).contains(&account_id) {
        return 0;
    }
    let idx = account_id - H_ACCOUNT_BASE;
    i32::from(idx < ACCOUNT_IDS.lock().unwrap().len())
}

pub unsafe extern "win64" fn EOS_EpicAccountId_FromString(account_id_string: *const u8) -> usize {
    if account_id_string.is_null() {
        return 0;
    }
    let mut bytes = Vec::with_capacity(EOS_EPICACCOUNTID_MAX_LENGTH);
    for i in 0..=EOS_EPICACCOUNTID_MAX_LENGTH {
        let b = std::ptr::read_unaligned(account_id_string.add(i));
        if b == 0 {
            break;
        }
        if i == EOS_EPICACCOUNTID_MAX_LENGTH || !(0x21..=0x7E).contains(&b) {
            return 0;
        }
        bytes.push(b);
    }
    if bytes.is_empty() {
        return 0;
    }
    let Ok(s) = String::from_utf8(bytes) else {
        return 0;
    };
    let mut table = ACCOUNT_IDS.lock().unwrap();
    if let Some(idx) = table.iter().position(|existing| existing == &s) {
        return H_ACCOUNT_BASE + idx;
    }
    let idx = table.len();
    log::info!("eossdk (builtin): EOS_EpicAccountId_FromString({s:?}) -> {:#x}", H_ACCOUNT_BASE + idx);
    table.push(s);
    H_ACCOUNT_BASE + idx
}

pub unsafe extern "win64" fn EOS_EpicAccountId_ToString(
    account_id: usize,
    out_buffer: *mut u8,
    in_out_buffer_length: *mut i32,
) -> i32 {
    if in_out_buffer_length.is_null() {
        return EOS_INVALID_PARAMETERS;
    }
    let Some(s) = lookup_account_id(account_id) else {
        std::ptr::write_unaligned(in_out_buffer_length, 0);
        if !out_buffer.is_null() {
            std::ptr::write_unaligned(out_buffer, 0);
        }
        return EOS_INVALID_PARAMETERS;
    };
    let needed = (s.len() + 1) as i32;
    let cap = std::ptr::read_unaligned(in_out_buffer_length);
    if out_buffer.is_null() || cap <= 0 {
        std::ptr::write_unaligned(in_out_buffer_length, needed);
        return EOS_INVALID_PARAMETERS;
    }
    if cap < needed {
        std::ptr::write_unaligned(out_buffer, 0);
        std::ptr::write_unaligned(in_out_buffer_length, needed);
        return EOS_LIMIT_EXCEEDED;
    }
    std::ptr::copy_nonoverlapping(s.as_ptr(), out_buffer, s.len());
    std::ptr::write_unaligned(out_buffer.add(s.len()), 0);
    std::ptr::write_unaligned(in_out_buffer_length, needed);
    EOS_SUCCESS
}

fn lookup_product_user_id(account_id: usize) -> Option<String> {
    if account_id < H_PUID_BASE {
        return None;
    }
    let idx = account_id - H_PUID_BASE;
    PRODUCT_USER_IDS.lock().unwrap().get(idx).cloned()
}

pub unsafe extern "win64" fn EOS_ProductUserId_IsValid(account_id: usize) -> i32 {
    if account_id < H_PUID_BASE {
        return 0;
    }
    let idx = account_id - H_PUID_BASE;
    i32::from(idx < PRODUCT_USER_IDS.lock().unwrap().len())
}

pub unsafe extern "win64" fn EOS_ProductUserId_FromString(product_user_id_string: *const u8) -> usize {
    if product_user_id_string.is_null() {
        return 0;
    }
    let mut bytes = Vec::with_capacity(EOS_PRODUCTUSERID_MAX_LENGTH);
    for i in 0..=EOS_PRODUCTUSERID_MAX_LENGTH {
        let b = std::ptr::read_unaligned(product_user_id_string.add(i));
        if b == 0 {
            break;
        }
        if i == EOS_PRODUCTUSERID_MAX_LENGTH || !(0x21..=0x7E).contains(&b) {
            return 0;
        }
        bytes.push(b);
    }
    if bytes.is_empty() {
        return 0;
    }
    let Ok(s) = String::from_utf8(bytes) else {
        return 0;
    };
    let mut table = PRODUCT_USER_IDS.lock().unwrap();
    if let Some(idx) = table.iter().position(|existing| existing == &s) {
        return H_PUID_BASE + idx;
    }
    let idx = table.len();
    log::info!("eossdk (builtin): EOS_ProductUserId_FromString({s:?}) -> {:#x}", H_PUID_BASE + idx);
    table.push(s);
    H_PUID_BASE + idx
}

pub unsafe extern "win64" fn EOS_ProductUserId_ToString(
    account_id: usize,
    out_buffer: *mut u8,
    in_out_buffer_length: *mut i32,
) -> i32 {
    if in_out_buffer_length.is_null() {
        return EOS_INVALID_PARAMETERS;
    }
    let Some(s) = lookup_product_user_id(account_id) else {
        std::ptr::write_unaligned(in_out_buffer_length, 0);
        if !out_buffer.is_null() {
            std::ptr::write_unaligned(out_buffer, 0);
        }
        return EOS_INVALID_PARAMETERS;
    };
    let needed = (s.len() + 1) as i32;
    let cap = std::ptr::read_unaligned(in_out_buffer_length);
    if out_buffer.is_null() || cap <= 0 {
        std::ptr::write_unaligned(in_out_buffer_length, needed);
        return EOS_INVALID_PARAMETERS;
    }
    if cap < needed {
        std::ptr::write_unaligned(out_buffer, 0);
        std::ptr::write_unaligned(in_out_buffer_length, needed);
        return EOS_LIMIT_EXCEEDED;
    }
    std::ptr::copy_nonoverlapping(s.as_ptr(), out_buffer, s.len());
    std::ptr::write_unaligned(out_buffer.add(s.len()), 0);
    std::ptr::write_unaligned(in_out_buffer_length, needed);
    EOS_SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epic_account_id_from_string_is_valid_and_to_string_roundtrip() {
        unsafe {
            assert_eq!(EOS_EpicAccountId_IsValid(0), 0);
            assert_eq!(EOS_EpicAccountId_IsValid(0x1234), 0);
            assert_eq!(EOS_EpicAccountId_FromString(std::ptr::null()), 0);
            assert_eq!(EOS_EpicAccountId_FromString(c"".as_ptr().cast()), 0);
            assert_eq!(EOS_EpicAccountId_FromString(c"bad id".as_ptr().cast()), 0);
            assert_eq!(
                EOS_EpicAccountId_FromString(c"0123456789abcdef0123456789abcdef0".as_ptr().cast()),
                0,
                "ids > 32 bytes must be rejected"
            );

            let h1 = EOS_EpicAccountId_FromString(c"Default-Epic-User".as_ptr().cast());
            assert_ne!(h1, 0);
            assert_eq!(EOS_EpicAccountId_IsValid(h1), 1);
            assert_eq!(EOS_EpicAccountId_IsValid(0), 0);
            // Interning returns the same handle for identical strings.
            let h1_again = EOS_EpicAccountId_FromString(c"Default-Epic-User".as_ptr().cast());
            assert_eq!(h1, h1_again);

            // Buffer too small -> EOS_LIMIT_EXCEEDED and reports needed size incl. NUL.
            let mut small = [0xAAu8; 8];
            let mut len = small.len() as i32;
            assert_eq!(EOS_EpicAccountId_ToString(h1, small.as_mut_ptr(), &mut len), EOS_LIMIT_EXCEEDED);
            assert_eq!(len, "Default-Epic-User".len() as i32 + 1);
            assert_eq!(small[0], 0);

            // Exact EOS_EPICACCOUNTID_MAX_LENGTH + 1 buffer succeeds.
            let mut buf = [0xCCu8; EOS_EPICACCOUNTID_MAX_LENGTH + 1];
            let mut cap = buf.len() as i32;
            assert_eq!(EOS_EpicAccountId_ToString(h1, buf.as_mut_ptr(), &mut cap), EOS_SUCCESS);
            assert_eq!(cap, "Default-Epic-User".len() as i32 + 1);
            assert_eq!(&buf[..cap as usize - 1], b"Default-Epic-User");
            assert_eq!(buf[cap as usize - 1], 0);

            // Invalid handle returns EOS_INVALID_PARAMETERS.
            let mut bad_len = 33;
            assert_eq!(EOS_EpicAccountId_ToString(0, buf.as_mut_ptr(), &mut bad_len), EOS_INVALID_PARAMETERS);
            assert_eq!(bad_len, 0);
        }
    }

    #[test]
    fn product_user_id_eresult_and_subinterfaces_offline() {
        unsafe {
            assert_eq!(
                std::ffi::CStr::from_ptr(EOS_EResult_ToString(EOS_SUCCESS).cast()).to_str().unwrap(),
                "EOS_Success"
            );
            assert_eq!(
                std::ffi::CStr::from_ptr(EOS_EResult_ToString(EOS_NO_CONNECTION).cast()).to_str().unwrap(),
                "EOS_NoConnection"
            );
            assert_eq!(
                std::ffi::CStr::from_ptr(EOS_EResult_ToString(EOS_NOT_FOUND).cast()).to_str().unwrap(),
                "EOS_NotFound"
            );
            assert_eq!(
                std::ffi::CStr::from_ptr(EOS_EResult_ToString(0x7FFF).cast()).to_str().unwrap(),
                "EOS_UnexpectedError"
            );

            assert_eq!(EOS_Platform_GetLeaderboardsInterface(H_PLATFORM), H_LEADERBOARDS);
            assert_eq!(EOS_Platform_GetLobbyInterface(H_PLATFORM), H_LOBBY);
            assert_eq!(EOS_Platform_GetFriendsInterface(H_PLATFORM), H_FRIENDS);

            let nid = EOS_Achievements_AddNotifyAchievementsUnlockedV2(H_ACHIEVEMENTS, std::ptr::null(), 0, 0);
            assert_ne!(nid, 0);
            EOS_Achievements_RemoveNotifyAchievementsUnlocked(H_ACHIEVEMENTS, nid);

            let mut out_stat = 0x1234usize;
            assert_eq!(EOS_Stats_CopyStatByName(H_STATS, std::ptr::null(), &mut out_stat), EOS_NOT_FOUND);
            assert_eq!(out_stat, 0);
            EOS_Stats_Stat_Release(0);

            // ProductUserId handle namespace is distinct from EpicAccountId.
            assert_eq!(EOS_ProductUserId_IsValid(0), 0);
            assert_eq!(EOS_ProductUserId_FromString(std::ptr::null()), 0);
            let puid = EOS_ProductUserId_FromString(c"0002aabbccdd11223344".as_ptr().cast());
            assert_ne!(puid, 0);
            assert_eq!(EOS_ProductUserId_IsValid(puid), 1);
            assert_eq!(EOS_EpicAccountId_IsValid(puid), 0);
            let mut buf = [0u8; EOS_PRODUCTUSERID_MAX_LENGTH + 1];
            let mut cap = buf.len() as i32;
            assert_eq!(EOS_ProductUserId_ToString(puid, buf.as_mut_ptr(), &mut cap), EOS_SUCCESS);
            assert_eq!(&buf[..cap as usize - 1], b"0002aabbccdd11223344");
        }
    }
}
