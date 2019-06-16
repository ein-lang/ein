use super::module::Module;
use super::utilities::*;
use llvm_sys::bit_writer::*;

pub unsafe fn write_bitcode_to_file(module: Module, path: &str) {
    LLVMWriteBitcodeToFile(module.internal(), c_string(path).as_ptr());
}
