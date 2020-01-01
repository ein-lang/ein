use super::type_::*;
use super::value::*;
use llvm_sys::core::*;
use std::os::raw::c_uint;

pub fn const_int(int_type: Type, value: u64) -> Value {
    unsafe { LLVMConstInt(int_type.into(), value, 0) }.into()
}

pub fn const_real(real_type: Type, value: f64) -> Value {
    unsafe { LLVMConstReal(real_type.into(), value) }.into()
}

pub fn const_named_struct(type_: Type, values: &[Value]) -> Value {
    unsafe {
        LLVMConstNamedStruct(
            type_.into(),
            values
                .iter()
                .map(|value| value.into())
                .collect::<Vec<_>>()
                .as_mut_ptr(),
            values.len() as c_uint,
        )
    }
    .into()
}

pub fn get_undef(type_: Type) -> Value {
    unsafe { LLVMGetUndef(type_.into()) }.into()
}
