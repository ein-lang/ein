use super::module::Module;
use llvm_sys::bit_writer::*;
use llvm_sys::core::*;

pub fn write_bitcode_to_memory_buffer(module: Module) -> &'static [u8] {
    unsafe {
        let buffer = LLVMWriteBitcodeToMemoryBuffer(module.internal());

        std::slice::from_raw_parts(
            LLVMGetBufferStart(buffer) as *const u8,
            LLVMGetBufferSize(buffer),
        )
    }
}
