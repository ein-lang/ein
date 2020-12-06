#![cfg(not(test))]

use bdwgc_alloc::Allocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

pub fn initialize() {
    unsafe { Allocator::initialize() }
}
