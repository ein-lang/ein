use super::type_::Type;
use llvm_sys::analysis::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Value {
    internal: LLVMValueRef,
}

impl Value {
    pub fn set_initializer(self, value: Value) {
        unsafe { LLVMSetInitializer(self.into(), value.into()) };
    }

    pub fn type_(self) -> Type {
        unsafe { LLVMTypeOf(self.into()) }.into()
    }

    pub fn is_global_variable(self) -> Value {
        unsafe { LLVMIsAGlobalVariable(self.into()) }.into()
    }

    pub fn get_param(self, index: std::os::raw::c_uint) -> Value {
        unsafe { LLVMGetParam(self.internal, index) }.into()
    }

    pub fn verify_function(self) {
        unsafe {
            LLVMVerifyFunction(
                self.internal,
                LLVMVerifierFailureAction::LLVMAbortProcessAction,
            )
        };
    }
}

impl From<LLVMValueRef> for Value {
    fn from(internal: LLVMValueRef) -> Self {
        Self { internal }
    }
}

impl From<Value> for LLVMValueRef {
    fn from(value: Value) -> Self {
        value.internal
    }
}

impl From<&Value> for LLVMValueRef {
    fn from(value: &Value) -> Self {
        value.internal
    }
}
