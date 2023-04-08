#![cfg_attr(
    feature = "no_std",
    no_std,
    no_main,
    feature(lang_items),
    feature(naked_functions),
    feature(default_alloc_error_handler)
)]
use {
    core::ffi::CStr,
    hex_ln::{
        gfx::{
            buffer::Usage,
            mesh::{Mesh, Topology},
            program::Program,
            shader::{POS2D_TEX2D, TEX2D},
            Resource, Target, SWAP_CHAIN,
        },
        gui::{font::Font, widget::TextBox},
        io::{
            event::{enable_text_input, Event, EventChannel, KeyCode},
            window::Window,
        },
        mem::vec::Vec,
    },
};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

#[cfg(feature = "no_std")]
mod no_std {
    #[lang = "eh_personality"]
    fn eh_personality() {}

    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        loop {}
    }

    #[no_mangle]
    #[naked]
    pub unsafe extern "C" fn _start() {
        use core::arch::asm;

        #[allow(dead_code)]
        extern "C" {
            fn exit(_: core::ffi::c_int);
        }

        asm!(
            "mov rdi, rsp",
            "call main",
            "mov rax, 0",
            "call exit",
            options(noreturn)
        )
    }

    #[global_allocator]
    static MALLOC: Malloc = Malloc {};
    struct Malloc;
    unsafe impl core::alloc::GlobalAlloc for Malloc {
        unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
            extern "C" {
                fn malloc(_: usize) -> *mut core::ffi::c_void;
            }

            malloc(layout.size() as _) as _
        }

        unsafe fn dealloc(&self, _ptr: *mut u8, _: core::alloc::Layout) {
            extern "C" {
                fn free(_: *mut core::ffi::c_void);
            }

            free(_ptr as _);
        }
    }
}

#[cfg_attr(feature = "no_std", no_mangle)]
pub fn main() {
    #[cfg(feature = "log")]
    hex_ln::init_logging();
    let window = Window::new(
        unsafe { &CStr::from_ptr("HELLO WORLD\0".as_ptr().cast()) },
        1920,
        1080,
    )
    .expect("window creation failed");

    let font = Font::default();
    let mut greets = TextBox::new([1920, 1080]);
    greets.update("Edit Mode:");
    greets.draw([1920, 1080], &font, 5.0);

    let tex_quad = Mesh::new(
        &[
            ([-1.0, 1.0], [0.0, 1.0]),
            ([1.0, 1.0], [1.0, 1.0]),
            ([-1.0, -1.0], [0.0, 0.0]),
            ([1.0, -1.0], [1.0, 0.0]),
        ],
        Usage::StaticDraw,
        Topology::TriStrip,
    );

    let glyph_prog = Program::new(POS2D_TEX2D, TEX2D);

    let mut events = EventChannel;
    log::set_max_level(log::LevelFilter::Off);

    let mut edit = false;
    loop {
        match events.next() {
            Some(event) => match event {
                Event::Quit => {
                    break;
                }

                Event::Keyboard { down, sym, .. } if sym == KeyCode::SDLK_F1 && down => {
                    edit = !edit;
                    enable_text_input(edit);
                }

                Event::TextInput { text } if edit => {
                    log::set_max_level(log::LevelFilter::Debug);
                    log::debug!("accepted text input of {:?}", text);
                    greets.update(unsafe {
                        core::str::from_utf8_unchecked(
                            &[
                                b"Edit Mode:\n",
                                text.iter()
                                    .map_while(|&ch| if ch != 0 { Some(ch as u8) } else { None })
                                    .collect::<Vec<u8>>()
                                    .as_slice(),
                            ]
                            .concat(),
                        )
                    });
                    greets.draw([1920, 1080], &font, 5.0);
                    log::set_max_level(log::LevelFilter::Off);
                }

                _ => {}
            },

            None => {}
        };

        SWAP_CHAIN.bind();
        SWAP_CHAIN.clear_color([0.0, 0.0, 0.0, 1.0]);
        SWAP_CHAIN.viewport([0, 0], [WIDTH, HEIGHT]);

        if edit {
            glyph_prog.bind();
            greets.view().bind();
            tex_quad.draw();
        }
        window.swap();
    }
}
