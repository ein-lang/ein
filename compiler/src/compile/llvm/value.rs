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

    pub unsafe fn set_initializer(&self, value: Value) {
        LLVMSetInitializer(self.into(), value.into());
    }

    pub unsafe fn type_(&self) -> Type {
        LLVMTypeOf(self.into()).into()
    }

    pub unsafe fn is_global_variable(&self) -> Value {
        LLVMIsAGlobalVariable(self.into()).into()
    }

    #[allow(dead_code)]
    pub unsafe fn dump(&self) {
        LLVMDumpValue(self.into())
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
