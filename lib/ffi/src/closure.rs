use std::os::raw::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Closure {
    entry_pointer: *const c_void,
    arity: usize,
}

impl Closure {
    pub const fn new(entry_pointer: *const c_void, arity: usize) -> Self {
        Self {
            entry_pointer,
            arity,
        }
    }
}

unsafe impl Sync for Closure {}
