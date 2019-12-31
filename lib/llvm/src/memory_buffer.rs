use super::module::Module;
use llvm_sys::bit_reader::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

#[derive(Debug)]
pub struct MemoryBuffer {
    internal: LLVMMemoryBufferRef,
}

impl MemoryBuffer {
    pub fn new(internal: LLVMMemoryBufferRef) -> Self {
        Self { internal }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                LLVMGetBufferStart(self.internal) as *const u8,
                LLVMGetBufferSize(self.internal),
            )
        }
    }

    pub fn get_bitcode_module(&self) -> Module {
        let mut module = std::mem::MaybeUninit::uninit();

        unsafe {
            LLVMGetBitcodeModule2(self.internal, module.as_mut_ptr());
            module.assume_init().into()
        }
    }
}

impl From<&[u8]> for MemoryBuffer {
    fn from(buffer: &[u8]) -> Self {
        Self {
            internal: unsafe {
                LLVMCreateMemoryBufferWithMemoryRangeCopy(
                    &buffer[0] as *const u8 as *const i8,
                    buffer.len(),
                    std::ptr::null(),
                )
            },
        }
    }
}
