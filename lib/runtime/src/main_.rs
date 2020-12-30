#![cfg(not(test))]

use bdwgc_alloc::Allocator;
use std::os::raw::{c_int, c_void};

extern "C" {
    static ein_main: extern "C" fn(environment: *const c_void, argument: ffi::None) -> ffi::None;
}

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

#[no_mangle]
pub extern "C" fn main() -> c_int {
    unsafe { Allocator::initialize() }

    unsafe { ein_main(std::ptr::null(), ffi::None::new()) };

    unreachable!()
}
