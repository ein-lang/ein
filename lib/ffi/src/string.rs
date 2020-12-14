use std::{alloc::Layout, intrinsics::copy_nonoverlapping};

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

    pub fn join(&self, other: &Self) -> EinString {
        unsafe {
            let length = self.length + other.length;
            let pointer = std::alloc::alloc(Layout::from_size_align_unchecked(length, 8));

            copy_nonoverlapping(self.bytes, pointer, self.length);
            copy_nonoverlapping(
                other.bytes,
                (pointer as usize + self.length) as *mut u8,
                other.length,
            );

            Self {
                bytes: pointer,
                length: self.length + other.length,
            }
        }
    }
}

unsafe impl Sync for EinString {}
