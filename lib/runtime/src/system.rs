use std::fs::File;
use std::io::Write;
use std::os::raw::c_void;
use std::os::unix::io::FromRawFd;

#[no_mangle]
static _ein_fd_write: ffi::Closure = ffi::Closure::new(fd_write as *const c_void, 2);

extern "C" fn fd_write(
    _environment: *const c_void,
    fd: ffi::Number,
    buffer: ffi::EinString,
) -> ffi::Number {
    let mut file = unsafe { File::from_raw_fd(f64::from(fd) as i32) };

    (file.write(buffer.as_slice()).unwrap() as f64).into()
}
