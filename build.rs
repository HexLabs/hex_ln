fn main() {
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=GL");
    if cfg!(feature = "no_std") {
        println!("cargo:rustc-link-arg-bin=hex_ln=-nostartfiles");
    }

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("ERR reading $OUT_DIR"));

    bindgen::builder()
        .use_core()
        .ctypes_prefix("core::ffi")
        .prepend_enum_name(false)
        .header_contents(
            "gl.h",
            "
#define GL_GLEXT_PROTOTYPES
#include <GL/gl.h>
#include <GL/glext.h>
            ",
        )
        .generate()
        .expect("ERR generating gl.h binding")
        .write_to_file(out_dir.join("gl.rs"))
        .expect("ERR writing gl.rs");

    bindgen::builder()
        .use_core()
        .ctypes_prefix("core::ffi")
        .prepend_enum_name(false)
        .header_contents(
            "sdl.h",
            "
#include <SDL2/SDL.h>
#include <SDL2/SDL_opengl.h>
            ",
        )
        .blocklist_item("FP_.*")
        .newtype_enum("SDL_KeyCode")
        .generate()
        .expect("ERR generating sdl.h binding")
        .write_to_file(out_dir.join("sdl.rs"))
        .expect("ERR writing sdl.rs");
}
