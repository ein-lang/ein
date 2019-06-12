use super::type_::*;
use super::value::*;
use llvm_sys::core::*;

pub unsafe fn const_int(int_type: Type, value: u64) -> Value {
    LLVMConstInt(int_type.into(), value, 0).into()
}

pub unsafe fn const_real(real_type: Type, value: f64) -> Value {
    LLVMConstReal(real_type.into(), value).into()
}

pub unsafe fn const_null(pointer_type: Type) -> Value {
    LLVMConstNull(pointer_type.into()).into()
}

pub unsafe fn get_undef(type_: Type) -> Value {
    LLVMGetUndef(type_.into()).into()
}
