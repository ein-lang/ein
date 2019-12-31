use llvm_sys::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct BasicBlock {
    internal: LLVMBasicBlockRef,
}

impl BasicBlock {
    pub fn new(internal: LLVMBasicBlockRef) -> Self {
        Self { internal }
    }
}

impl From<LLVMBasicBlockRef> for BasicBlock {
    fn from(block_ref: LLVMBasicBlockRef) -> Self {
        Self::new(block_ref)
    }
}

impl From<BasicBlock> for LLVMBasicBlockRef {
    fn from(block: BasicBlock) -> Self {
        block.internal
    }
}

impl From<&BasicBlock> for LLVMBasicBlockRef {
    fn from(block: &BasicBlock) -> Self {
        block.internal
    }
}
