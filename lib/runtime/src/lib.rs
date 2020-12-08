mod main_;
mod system;
mod string;

use std::alloc::Layout;
use std::io::Write;
use std::os::raw::c_void;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
