use super::llvm;
use super::object_blob::ObjectBlob;

pub struct Module {
    module: llvm::Module,
}

impl Module {
    pub fn new(module: llvm::Module) -> Self {
        Self { module }
    }

    pub fn to_object_blob(&self) -> ObjectBlob {
        ObjectBlob::new(llvm::write_bitcode_to_memory_buffer(&self.module))
    }
}
