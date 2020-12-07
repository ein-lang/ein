#![cfg(not(test))]

use std::os::raw::c_void;

#[repr(C)]
pub struct Os {
    fd_write: *const ffi::Closure,
}

unsafe impl Sync for Os {}

static FD_WRITE: ffi::Closure = ffi::Closure::new(os_fd_write as *const c_void, 2);

pub static OS: Os = Os {
    fd_write: &FD_WRITE,
};

extern "C" fn os_fd_write(
    _environment: *const c_void,
    _fd: ffi::Number,
    _buffers: ffi::EinString,
) -> ffi::Number {
    todo!()
}
