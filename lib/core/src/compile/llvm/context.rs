use llvm_sys::core::*;
use llvm_sys::prelude::*;

#[derive(Clone, Debug)]
pub struct Context {
    internal: LLVMContextRef,
}

impl Context {
    pub fn new() -> Self {
        Self {
            internal: unsafe { LLVMContextCreate() },
        }
    }

    pub(super) fn internal(&self) -> LLVMContextRef {
        self.internal
    }
}

impl From<LLVMContextRef> for Context {
    fn from(internal: LLVMContextRef) -> Self {
        Self { internal }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
