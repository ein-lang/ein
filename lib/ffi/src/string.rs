use super::number::Number;
use std::{
    alloc::Layout, cmp::max, intrinsics::copy_nonoverlapping, ptr::null, str::from_utf8_unchecked,
};

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

    pub const fn empty() -> Self {
        Self {
            bytes: null(),
            length: 0,
        }
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
                length,
            }
        }
    }

    // Indices are inclusive and start from 1.
    pub fn slice(&self, start: Number, end: Number) -> EinString {
        let start = f64::from(start);
        let end = f64::from(end);

        if !start.is_finite() || !end.is_finite() {
            return Self::empty();
        }

        let start = max(start as isize - 1, 0) as usize;
        let end = max(end as isize, 0) as usize;

        let string = unsafe { from_utf8_unchecked(self.as_slice()) };

        if string.is_empty() || start >= string.len() || end <= start {
            return Self::empty();
        }

        let start_index = Self::get_string_index(string, start);
        let end_index = Self::get_string_index(string, end);

        Self {
            bytes: (self.bytes as usize + start_index) as *const u8,
            length: string[start_index..end_index].as_bytes().len(),
        }
    }

    fn get_string_index(string: &str, index: usize) -> usize {
        string
            .char_indices()
            .nth(index)
            .map(|(index, _)| index)
            .unwrap_or_else(|| string.as_bytes().len())
    }
}

unsafe impl Sync for EinString {}

impl From<&str> for EinString {
    fn from(string: &str) -> Self {
        let bytes = string.as_bytes();

        Self {
            bytes: bytes.as_ptr(),
            length: bytes.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_with_ascii() {
        assert_eq!(
            EinString::from("abc")
                .slice(2.0.into(), 2.0.into())
                .as_slice(),
            EinString::from("b").as_slice()
        );
    }

    #[test]
    fn slice_with_negative_index() {
        assert_eq!(
            EinString::from("abc")
                .slice((-1.0).into(), 3.0.into())
                .as_slice(),
            EinString::from("abc").as_slice()
        );
    }

    #[test]
    fn slice_into_whole() {
        assert_eq!(
            EinString::from("abc")
                .slice(1.0.into(), 3.0.into())
                .as_slice(),
            EinString::from("abc").as_slice()
        );
    }

    #[test]
    fn slice_into_empty() {
        assert_eq!(
            EinString::from("abc")
                .slice(4.0.into(), 4.0.into())
                .as_slice(),
            EinString::from("").as_slice()
        );
    }

    #[test]
    fn slice_with_emojis() {
        assert_eq!(
            EinString::from("😀😉😂")
                .slice(2.0.into(), 2.0.into())
                .as_slice(),
            EinString::from("😉").as_slice()
        );
    }
}
