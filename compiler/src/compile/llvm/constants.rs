use super::prelude::*;
use llvm_sys::core::*;

pub unsafe fn const_int(int_type: Type, value: u64) -> Value {
    LLVMConstInt(int_type, value, 0)
}

pub unsafe fn const_real(real_type: Type, value: f64) -> Value {
    LLVMConstReal(real_type, value)
}

pub unsafe fn const_null(pointer_type: Type) -> Value {
    LLVMConstNull(pointer_type)
}
