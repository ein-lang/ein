use super::memory_buffer::MemoryBuffer;
use super::module::Module;
use llvm_sys::bit_reader::*;
use llvm_sys::bit_writer::*;

pub fn get_bitcode_module(buffer: MemoryBuffer) -> Module {
    let mut module = std::mem::MaybeUninit::uninit();

    unsafe {
        LLVMGetBitcodeModule2(buffer.internal(), module.as_mut_ptr());
        module.assume_init().into()
    }
}

pub fn write_bitcode_to_memory_buffer(module: &Module) -> MemoryBuffer {
    MemoryBuffer::new(unsafe { LLVMWriteBitcodeToMemoryBuffer(module.internal()) })
}
