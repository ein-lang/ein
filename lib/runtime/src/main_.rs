#![cfg(not(test))]

use std::os::raw::c_int;

extern "C" {
    static ein_main: extern "C" fn(environment: *const u8, argument: f64) -> f64;
}

#[no_mangle]
pub extern "C" fn main() -> c_int {
    ffi::initialize();

    println!("{}", unsafe { ein_main(std::ptr::null(), 42.0) });

    0
}
