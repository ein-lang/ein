pub struct Module {
    module: llvm::Module,
}

impl Module {
    pub fn new(module: llvm::Module) -> Self {
        Self { module }
    }

    pub fn deserialize(bitcode: &[u8]) -> Self {
        Self::new(llvm::Module::from_bitcode(bitcode))
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.module.to_bitcode()
    }
}
