// FFI bindings to the underlying C game engine
use std::ffi::CString;
use std::os::raw::{c_int, c_float, c_void};

// Representation of a Sprite in the game engine
#[repr(C)]
pub struct Sprite {
    pub x: c_float,
    pub y: c_float,
    pub width: c_int,
    pub height: c_int,
    pub color: [c_int; 3],
}

// Key constants
pub const GLFW_PRESS: c_int = 1;
pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;

// FFI function declarations
extern "C" {
    pub fn create_game_window(title: *const i8, width: c_int, height: c_int);

    pub fn create_sprite(
        x: c_float,
        y: c_float,
        width: c_int,
        height: c_int,
        r: c_int,
        g: c_int,
        b: c_int,
    ) -> *mut Sprite;

    pub fn render_sprite(sprite: *mut Sprite);

    pub fn update_sprite_position(sprite: *mut Sprite, x: c_float, y: c_float);

    pub fn update_game_window();

    pub fn clear_screen();

    pub fn window_should_close() -> c_int;

    pub fn get_key(window: *mut c_void, key: c_int) -> c_int;

    pub fn get_window() -> *mut c_void;
}

// Safe Rust wrappers around the FFI functions
pub fn create_window(title: &str, width: i32, height: i32) {
    let c_title = CString::new(title).unwrap();
    unsafe { create_game_window(c_title.as_ptr() as *const i8, width, height) }
}

pub fn sprite(x: f32, y: f32, w: i32, h: i32, r: i32, g: i32, b: i32) -> *mut Sprite {
    unsafe { create_sprite(x, y, w, h, r, g, b) }
}

pub fn render(s: *mut Sprite) {
    unsafe { render_sprite(s) }
}

pub fn update_sprite(s: *mut Sprite, x: f32, y: f32) {
    unsafe { update_sprite_position(s, x, y) }
}

pub fn update_window() {
    unsafe { update_game_window() }
}

pub fn clear() {
    unsafe { clear_screen() }
}

pub fn should_close() -> bool {
    unsafe { window_should_close() != 0 }
}

pub fn key_pressed(key: i32) -> bool {
    unsafe { get_key(get_window(), key) == GLFW_PRESS }
}
