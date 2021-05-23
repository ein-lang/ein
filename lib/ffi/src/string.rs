use super::{arc::Arc, number::Number};
use std::{cmp::max, intrinsics::copy_nonoverlapping, str::from_utf8_unchecked};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct EinString {
    bytes: Arc<u8>, // variadic length array
    length: usize,
}

impl EinString {
    pub const fn new(
        bytes: Arc<u8>, // variadic length array
        length: usize,
    ) -> Self {
        Self { bytes, length }
    }

    pub fn empty() -> Self {
        Self {
            bytes: Arc::empty(),
            length: 0,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.bytes.as_ptr(), self.length) }
    }

    pub fn join(&self, other: &Self) -> EinString {
        unsafe {
            let length = self.length + other.length;
            let mut bytes = Arc::buffer(length);

            copy_nonoverlapping(self.bytes.as_ptr(), bytes.as_ptr_mut(), self.length);
            copy_nonoverlapping(
                other.bytes.as_ptr(),
                (bytes.as_ptr_mut() as usize + self.length) as *mut u8,
                other.length,
            );

            Self { bytes, length }
        }
    }

    // Indices are inclusive and start from 1.
    pub fn slice(&self, start: Number, end: Number) -> EinString {
        let start = f64::from(start);
        let end = f64::from(end);

        // TODO Allow infinite ranges
        if !start.is_finite() || !end.is_finite() {
            return Self::empty();
        }

        let start = max(start as isize - 1, 0) as usize;
        let end = max(end as isize, 0) as usize;

        let string = unsafe { from_utf8_unchecked(self.as_slice()) };

        if string.is_empty() || start >= string.chars().count() || end <= start {
            Self::empty()
        } else {
            string[Self::get_string_index(string, start)..Self::get_string_index(string, end)]
                .into()
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

impl Default for EinString {
    fn default() -> Self {
        Self {
            bytes: Arc::empty(),
            length: 0,
        }
    }
}

impl PartialEq for EinString {
    fn eq(&self, other: &EinString) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl From<&[u8]> for EinString {
    fn from(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.into(),
            length: bytes.len(),
        }
    }
}

impl From<&str> for EinString {
    fn from(string: &str) -> Self {
        string.as_bytes().into()
    }
}

impl From<String> for EinString {
    fn from(string: String) -> Self {
        string.as_str().into()
    }
}

impl From<Vec<u8>> for EinString {
    fn from(vec: Vec<u8>) -> Self {
        vec.as_slice().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join() {
        assert_eq!(
            EinString::from("foo").join(&EinString::from("bar")),
            EinString::from("foobar")
        );
    }

    #[test]
    fn slice_with_ascii() {
        assert_eq!(
            EinString::from("abc").slice(2.0.into(), 2.0.into()),
            EinString::from("b")
        );
    }

    #[test]
    fn slice_with_negative_index() {
        assert_eq!(
            EinString::from("abc").slice((-1.0).into(), 3.0.into()),
            EinString::from("abc")
        );
    }

    #[test]
    fn slice_into_whole() {
        assert_eq!(
            EinString::from("abc").slice(1.0.into(), 3.0.into()),
            EinString::from("abc")
        );
    }

    #[test]
    fn slice_into_empty() {
        assert_eq!(
            EinString::from("abc").slice(4.0.into(), 4.0.into()),
            EinString::from("")
        );
    }

    #[test]
    fn slice_with_emojis() {
        assert_eq!(
            EinString::from("😀😉😂").slice(2.0.into(), 2.0.into()),
            EinString::from("😉")
        );
    }

    #[test]
    fn slice_last_with_emojis() {
        assert_eq!(
            EinString::from("😀😉😂").slice(3.0.into(), 3.0.into()),
            EinString::from("😂")
        );
    }
}
