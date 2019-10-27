use super::llvm;

pub struct ObjectBlob {
    memory_buffer: llvm::MemoryBuffer,
}

impl ObjectBlob {
    pub fn new(memory_buffer: llvm::MemoryBuffer) -> Self {
        Self { memory_buffer }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.memory_buffer.as_bytes()
    }
}
