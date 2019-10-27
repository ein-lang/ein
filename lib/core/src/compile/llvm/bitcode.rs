use super::memory_buffer::MemoryBuffer;
use super::module::Module;
use llvm_sys::bit_writer::*;

pub fn write_bitcode_to_memory_buffer(module: &Module) -> MemoryBuffer {
    MemoryBuffer::new(unsafe { LLVMWriteBitcodeToMemoryBuffer(module.internal()) })
}
