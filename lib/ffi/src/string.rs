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

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.bytes, self.length) }
    }
}

unsafe impl Sync for EinString {}
