use super::type_::Type;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Value {
    internal: LLVMValueRef,
}

impl Value {
    pub(super) fn new(internal: LLVMValueRef) -> Self {
        Self { internal }
    }

    pub fn set_initializer(self, value: Value) {
        unsafe { LLVMSetInitializer(self.into(), value.into()) };
    }

    pub fn type_(self) -> Type {
        unsafe { LLVMTypeOf(self.into()) }.into()
    }

    pub fn is_global_variable(self) -> Value {
        unsafe { LLVMIsAGlobalVariable(self.into()) }.into()
    }
}

impl From<LLVMValueRef> for Value {
    fn from(value_ref: LLVMValueRef) -> Self {
        Self::new(value_ref)
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
