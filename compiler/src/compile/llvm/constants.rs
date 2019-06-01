use super::prelude::*;
use llvm_sys::core::*;

pub unsafe fn const_real(real_type: Type, value: f64) -> Value {
    LLVMConstReal(real_type, value)
}
