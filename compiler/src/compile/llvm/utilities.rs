use super::value::*;
use llvm_sys::core::*;

pub fn c_string(string: &str) -> std::ffi::CString {
    std::ffi::CString::new(string).unwrap()
}

pub unsafe fn get_param(function: Value, index: std::os::raw::c_uint) -> Value {
    LLVMGetParam(function.into(), index).into()
}
