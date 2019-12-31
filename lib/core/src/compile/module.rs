pub struct Module {
    module: llvm::Module,
}

impl Module {
    pub fn new(module: llvm::Module) -> Self {
        Self { module }
    }

    pub fn deserialize(buffer: &[u8]) -> Self {
        Self::new(llvm::get_bitcode_module(buffer.into()))
    }

    pub fn serialize(&self) -> Vec<u8> {
        llvm::write_bitcode_to_memory_buffer(&self.module)
            .as_bytes()
            .into()
    }
}
