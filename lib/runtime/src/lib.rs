mod closure;
mod string;

use bdwgc_alloc::Allocator;
use std::alloc::Layout;
use std::io::Write;
use std::os::raw::c_void;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

#[cfg(not(test))]
mod main {
    use super::*;
    use std::os::raw::c_int;

    extern "C" {
        static ein_main: extern "C" fn(environment: *const u8, argument: f64) -> f64;
    }

    #[no_mangle]
    pub extern "C" fn main() -> c_int {
        unsafe { Allocator::initialize() }

        println!("{}", unsafe { ein_main(std::ptr::null(), 42.0) });

        0
    }
}

#[no_mangle]
pub extern "C" fn ein_malloc(size: usize) -> *mut c_void {
    (unsafe { std::alloc::alloc(Layout::from_size_align(size, 8).unwrap()) }) as *mut c_void
}

#[no_mangle]
pub extern "C" fn ein_panic() -> c_void {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);

    stderr
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    write!(&mut stderr, "UNEXPECTED RUNTIME PANIC").unwrap();
    stderr.set_color(ColorSpec::new().set_fg(None)).unwrap();

    std::process::exit(-1);
}
