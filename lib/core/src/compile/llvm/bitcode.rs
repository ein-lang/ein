use super::memory_buffer::MemoryBuffer;
use super::module::Module;
use llvm_sys::bit_reader::*;
use llvm_sys::bit_writer::*;
use llvm_sys::prelude::LLVMModuleRef;

pub fn get_bitcode_module(buffer: MemoryBuffer) -> Module {
    unsafe {
        let mut module: LLVMModuleRef = std::mem::uninitialized();
        LLVMGetBitcodeModule2(buffer.internal(), &mut module);
        module.into()
    }
}

pub fn write_bitcode_to_memory_buffer(module: &Module) -> MemoryBuffer {
    MemoryBuffer::new(unsafe { LLVMWriteBitcodeToMemoryBuffer(module.internal()) })
}
