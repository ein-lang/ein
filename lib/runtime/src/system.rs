#![cfg(not(test))]

use std::fs::File;
use std::io::Write;
use std::os::raw::c_void;
use std::os::unix::io::FromRawFd;

#[repr(C)]
pub struct System {
    fd_write: *const ffi::Closure,
}

unsafe impl Sync for System {}

static FD_WRITE: ffi::Closure = ffi::Closure::new(system_fd_write as *const c_void, 2);

pub static SYSTEM: System = System {
    fd_write: &FD_WRITE,
};

extern "C" fn system_fd_write(
    _environment: *const c_void,
    fd: ffi::Number,
    buffer: ffi::EinString,
) -> ffi::Number {
    let mut file = unsafe { File::from_raw_fd(f64::from(fd) as i32) };

    let byte_count = (file.write(buffer.as_slice()).unwrap() as f64).into();

    std::mem::forget(file);

    byte_count
}
