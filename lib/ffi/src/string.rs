use super::{arc::ArcBuffer, number::Number};
use std::{cmp::max, str::from_utf8_unchecked};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct EinString {
    buffer: ArcBuffer,
}

impl EinString {
    pub fn new(buffer: ArcBuffer) -> Self {
        Self { buffer }
    }

    pub fn empty() -> Self {
        Self {
            buffer: ArcBuffer::new(0),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    pub fn len(&self) -> usize {
        self.buffer.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn join(&self, other: &Self) -> Self {
        let mut buffer = ArcBuffer::new(self.len() + other.len());

        buffer.as_slice_mut()[..self.len()].copy_from_slice(self.as_slice());
        buffer.as_slice_mut()[self.len()..].copy_from_slice(other.as_slice());

        Self { buffer }
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
            string[Self::get_byte_index(string, start)..Self::get_byte_index(string, end)].into()
        }
    }

    fn get_byte_index(string: &str, index: usize) -> usize {
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
            buffer: ArcBuffer::new(0),
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
            buffer: bytes.into(),
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
    fn join_empty() {
        assert_eq!(
            EinString::from("").join(&EinString::from("")),
            EinString::from("")
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
