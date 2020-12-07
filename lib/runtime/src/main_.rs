#![cfg(not(test))]

use super::os::{Os, OS};
use std::os::raw::{c_int, c_void};

extern "C" {
    static ein_main: extern "C" fn(environment: *const c_void, argument: *const Os) -> f64;
}

#[no_mangle]
pub extern "C" fn main() -> c_int {
    ffi::initialize();

    (unsafe { ein_main(std::ptr::null(), &OS) }) as c_int
}
