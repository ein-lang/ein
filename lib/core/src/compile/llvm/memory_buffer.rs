use llvm_sys::core::*;
use llvm_sys::prelude::*;

#[derive(Debug)]
pub struct MemoryBuffer {
    memory_buffer: LLVMMemoryBufferRef,
}

impl MemoryBuffer {
    pub fn new(memory_buffer: LLVMMemoryBufferRef) -> Self {
        Self { memory_buffer }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                LLVMGetBufferStart(self.memory_buffer) as *const u8,
                LLVMGetBufferSize(self.memory_buffer),
            )
        }
    }
}
