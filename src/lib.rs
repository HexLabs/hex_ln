#![no_std]
#![feature(core_intrinsics)]
pub mod gfx;
pub mod gui;
pub mod io;
pub mod math;
pub mod mem;

/// # optional log to stdout
#[cfg(feature = "log")]
pub fn init_logging() {
    extern crate std;
    pub use log::*;
    static LOGGER: Log = Log;
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("failed to init logs");
    log::info!("Greetz");

    struct Log;
    impl log::Log for Log {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Debug
        }

        fn log(&self, record: &Record) {
            use std::println;
            println!("[{:>5}] {}", record.level(), record.args());
        }

        fn flush(&self) {}
    }
}
