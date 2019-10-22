use core::compile::ObjectBlob;

#[derive(Debug)]
pub struct ModuleObjectBlob {
    object_blob: ObjectBlob,
}

impl ModuleObjectBlob {
    pub fn new(object_blob: ObjectBlob) -> Self {
        Self { object_blob }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.object_blob.as_bytes()
    }
}
