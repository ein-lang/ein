use std::{os::raw::c_void, ptr::null};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Closure {
    entry_pointer: *const c_void,
    drop_function: *const u8,
    arity: usize,
}

impl Closure {
    pub const fn new(entry_pointer: *const c_void, arity: usize) -> Self {
        Self {
            entry_pointer,
            drop_function: null(),
            arity,
        }
    }
}

unsafe impl Sync for Closure {}
