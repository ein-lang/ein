pub struct Module {
    module: llvm::Module,
}

impl Module {
    pub fn new(module: llvm::Module) -> Self {
        Self { module }
    }

    pub fn deserialize(buffer: &[u8]) -> Self {
        Self::new(llvm::MemoryBuffer::from(buffer).get_bitcode_module())
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.module
            .write_bitcode_to_memory_buffer()
            .as_bytes()
            .into()
    }
}
