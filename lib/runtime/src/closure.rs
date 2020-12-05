use std::os::raw::c_void;

pub struct Closure {
    #[allow(dead_code)]
    entry_pointer: *mut c_void,
    #[allow(dead_code)]
    arity: usize,
}

impl Closure {
    pub const fn new(entry_pointer: *mut c_void, arity: usize) -> Self {
        Self {
            entry_pointer,
            arity,
        }
    }
}

unsafe impl Sync for Closure {}
