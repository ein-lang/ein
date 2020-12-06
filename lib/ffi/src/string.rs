#[repr(C)]
#[derive(Copy, Clone)]
pub struct EinString {
    bytes: *const u8, // variadic length array
    length: usize,
}

impl EinString {
    pub const fn new(
        bytes: *const u8, // variadic length array
        length: usize,
    ) -> Self {
        Self { bytes, length }
    }

    pub fn bytes(&self) -> *const u8 {
        self.bytes
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

unsafe impl Sync for EinString {}
