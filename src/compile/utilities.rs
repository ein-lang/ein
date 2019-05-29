pub fn c_string(string: &str) -> *const std::os::raw::c_char {
    std::ffi::CString::new(string).unwrap().as_ptr()
}
