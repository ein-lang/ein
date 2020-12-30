mod main_;
mod string;

use std::alloc::Layout;
use std::os::raw::c_void;

#[no_mangle]
pub extern "C" fn ein_malloc(size: usize) -> *mut c_void {
    (unsafe { std::alloc::alloc(Layout::from_size_align(size, 8).unwrap()) }) as *mut c_void
}
