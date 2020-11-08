#![cfg(not(test))]

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

extern "C" {
    static ein_main: extern "C" fn(environment: *const u8, argument: f64) -> f64;
}

#[no_mangle]
pub extern "C" fn main() -> std::os::raw::c_int {
    println!("{}", unsafe { ein_main(std::ptr::null(), 42.0) });

    0
}

#[no_mangle]
pub extern "C" fn ein_panic() -> std::os::raw::c_void {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);

    stderr
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    write!(&mut stderr, "UNEXPECTED RUNTIME PANIC").unwrap();
    stderr.set_color(ColorSpec::new().set_fg(None)).unwrap();

    std::process::exit(-1);
}
