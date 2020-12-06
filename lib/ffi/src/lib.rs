mod closure;
mod string;

use bdwgc_alloc::Allocator;
pub use closure::*;
pub use string::*;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

pub fn initialize() {
    unsafe { Allocator::initialize() }
}
