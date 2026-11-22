#ifndef PORT_HOST_BRIDGING_HEADER_H
#define PORT_HOST_BRIDGING_HEADER_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Optional startup paths passed to `port_ios_start_background`.
/// Any field may be NULL to auto-discover `<App>.app/game.toml`, `<App>.app/Game`,
/// and `~/Documents/Translator`.
typedef struct PortIosConfig {
    const char *game_dir;
    const char *exe_name;
    const char *manifest_path;
    const char *drive_c_dir;
} PortIosConfig;

/// Static signed ARM64 AOT entry (Milestone #2: offline Mach-O `.o` emitter).
typedef struct StaticAotEntry {
    uint32_t rva;
    uint32_t code_offset;
    uint32_t flags;
} StaticAotEntry;

/// Descriptor embedded in `__DATA,__const` of every offline-compiled `<module>.aot.o`
/// object emitted by `port-runner --emit-static-o` / `port-factory aot-static-obj`.
/// Registered automatically before `main()` via `__DATA,__mod_init_func`.
typedef struct StaticAotModuleDesc {
    uint32_t magic;
    uint32_t version;
    uint64_t knobs;
    uint64_t image_base;
    uint64_t image_size;
    uint64_t image_hash[2];
    uintptr_t code_base;
    uint32_t code_len;
    uint32_t enter_off;
    uint32_t lookup_rip_off;
    uint32_t flags;
    uint32_t entry_count;
    uint32_t _pad;
    const StaticAotEntry *entries;
    const char *module_name;
} StaticAotModuleDesc;

enum {
    STATIC_AOT_DESC_MAGIC = 0x53414F54u,
    STATIC_AOT_DESC_VERSION = 1u,
    STATIC_AOT_MOD_HAS_RAS = 1u << 0,
    STATIC_AOT_MOD_HAS_XMM_PIN = 1u << 1,
    STATIC_AOT_MOD_HAS_AOT_PROBE = 1u << 2,
};

/// Touch phases matching `win32_rt::platform::uikit::TOUCH_*`.
enum {
    PORT_IOS_TOUCH_BEGAN = 0,
    PORT_IOS_TOUCH_MOVED = 1,
    PORT_IOS_TOUCH_ENDED = 2,
    PORT_IOS_TOUCH_CANCELLED = 3,
};

/// Mouse buttons matching `win32_rt::platform::uikit::MOUSE_*`.
enum {
    PORT_IOS_MOUSE_LEFT = 0,
    PORT_IOS_MOUSE_RIGHT = 1,
    PORT_IOS_MOUSE_MIDDLE = 2,
};

/// XInput button bitmask matching `win32_rt::misc::xinput1_4::btn::*`.
enum {
    PORT_IOS_BTN_DPAD_UP        = 0x0001u,
    PORT_IOS_BTN_DPAD_DOWN      = 0x0002u,
    PORT_IOS_BTN_DPAD_LEFT      = 0x0004u,
    PORT_IOS_BTN_DPAD_RIGHT     = 0x0008u,
    PORT_IOS_BTN_START          = 0x0010u,
    PORT_IOS_BTN_BACK           = 0x0020u,
    PORT_IOS_BTN_LEFT_THUMB     = 0x0040u,
    PORT_IOS_BTN_RIGHT_THUMB    = 0x0080u,
    PORT_IOS_BTN_LEFT_SHOULDER  = 0x0100u,
    PORT_IOS_BTN_RIGHT_SHOULDER = 0x0200u,
    PORT_IOS_BTN_GUIDE          = 0x0400u,
    PORT_IOS_BTN_A              = 0x1000u,
    PORT_IOS_BTN_B              = 0x2000u,
    PORT_IOS_BTN_X              = 0x4000u,
    PORT_IOS_BTN_Y              = 0x8000u,
};

/// Host callback invoked when a Windows game calls `XInputSetState(0, ...)` and no
/// rumble-capable physical controller owns slot 0 (`left_motor` = low-frequency 0..65535,
/// `right_motor` = high-frequency 0..65535).
typedef void (*PortIosRumbleCallback)(uint16_t left_motor, uint16_t right_motor);

void port_ios_attach_surface(void *metal_layer, uint32_t width_px, uint32_t height_px, double scale);
void port_ios_resize_surface(uint32_t width_px, uint32_t height_px, double scale);
void port_ios_detach_surface(void);
void port_ios_touch_event(uint32_t phase, float x_px, float y_px);
void port_ios_mouse_button(uint32_t button, bool down, float x_px, float y_px);
void port_ios_scroll_wheel(float x_px, float y_px, int32_t delta_y, int32_t delta_x);
void port_ios_key_event(uint16_t mac_keycode, bool down, bool repeat, uint16_t utf16_char);
void port_ios_text_input(const char *utf8_text);
void port_ios_keyboard_state(bool hardware_connected, bool software_visible, uint32_t software_height_px);
void port_ios_gamepad_state(
    bool connected,
    uint16_t buttons,
    uint8_t left_trigger,
    uint8_t right_trigger,
    int16_t thumb_lx,
    int16_t thumb_ly,
    int16_t thumb_rx,
    int16_t thumb_ry
);
void port_ios_set_rumble_callback(PortIosRumbleCallback cb);
void port_ios_tick(void);
void port_ios_memory_warning(void);
void port_static_aot_register_desc(const StaticAotModuleDesc *desc);
void port_ios_register_static_aot(
    const char *module_name,
    uintptr_t code_base,
    const StaticAotEntry *entries,
    size_t entry_count
);
int32_t port_ios_start_background(const PortIosConfig *cfg);
void port_ios_request_exit(uint32_t code);

#ifdef __cplusplus
}
#endif

#endif /* PORT_HOST_BRIDGING_HEADER_H */
