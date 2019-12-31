use super::type_::*;
use super::value::*;
use llvm_sys::core::*;

pub fn const_int(int_type: Type, value: u64) -> Value {
    unsafe { LLVMConstInt(int_type.into(), value, 0) }.into()
}

pub fn const_real(real_type: Type, value: f64) -> Value {
    unsafe { LLVMConstReal(real_type.into(), value) }.into()
}

pub fn get_undef(type_: Type) -> Value {
    unsafe { LLVMGetUndef(type_.into()) }.into()
}
