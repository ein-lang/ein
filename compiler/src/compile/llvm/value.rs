use llvm_sys::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Value {
    internal: LLVMValueRef,
}

impl Value {
    pub(super) fn new(internal: LLVMValueRef) -> Self {
        Self { internal }
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
