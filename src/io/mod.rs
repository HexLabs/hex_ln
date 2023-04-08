pub mod event;
pub mod window;

#[allow(
    dead_code,
    improper_ctypes,
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types
)]
mod sdl {
    include!(concat!(env!("OUT_DIR"), "/sdl.rs"));
}

pub use sdl::SDL_Keycode as Keycode;
