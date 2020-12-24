#![cfg(not(test))]

use std::os::raw::{c_int, c_void};

extern "C" {
    static ein_main: extern "C" fn(environment: *const c_void) -> ffi::Number;
}

#[no_mangle]
pub extern "C" fn main() -> c_int {
    ffi::initialize();

    f64::from(unsafe { ein_main(std::ptr::null()) }) as c_int
}
