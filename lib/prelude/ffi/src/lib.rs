#[no_mangle]
extern "C" fn _ein_equal_strings(one: ffi::EinString, other: ffi::EinString) -> ffi::Boolean {
    (one.as_slice() == other.as_slice()).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_empty_strings() {
        let string = ffi::EinString::empty();

        assert_eq!(_ein_equal_strings(string.clone(), string), true.into());
    }

    #[test]
    fn equal_one_byte_strings() {
        let string = ffi::EinString::from(vec![0u8]);

        assert_eq!(_ein_equal_strings(string.clone(), string), true.into());
    }

    #[test]
    fn not_equal_one_byte_strings() {
        let one = ffi::EinString::empty();
        let other = vec![0u8].into();

        assert_eq!(_ein_equal_strings(one, other), false.into());
    }

    #[test]
    fn equal_text_strings() {
        const TEXT: &[u8] = "hello".as_bytes();

        let string = ffi::EinString::from(TEXT);

        assert_eq!(_ein_equal_strings(string.clone(), string), true.into());
    }

    #[test]
    fn not_equal_text_strings() {
        const TEXT: &[u8] = "hello".as_bytes();
        const OTHER_TEXT: &[u8] = "hell0".as_bytes();

        assert_eq!(
            _ein_equal_strings(TEXT.into(), OTHER_TEXT.into(),),
            false.into()
        );
    }
}
