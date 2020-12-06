#[repr(C)]
pub struct EinString {
    pub bytes: *const u8, // variadic length array
    pub length: usize,
}

unsafe impl Sync for EinString {}
