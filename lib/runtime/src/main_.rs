#![cfg(not(test))]

use super::system::{System, SYSTEM};
use bdwgc_alloc::Allocator;
use std::os::raw::{c_int, c_void};

extern "C" {
    static ein_main:
        extern "C" fn(environment: *const c_void, argument: *const System) -> ffi::Number;
}

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

#[no_mangle]
pub extern "C" fn main() -> c_int {
    unsafe { Allocator::initialize() }

    f64::from(unsafe { ein_main(std::ptr::null(), &SYSTEM) }) as c_int
}
