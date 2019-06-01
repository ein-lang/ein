pub fn c_string(string: &str) -> std::ffi::CString {
    std::ffi::CString::new(string).unwrap()
}
