use std::os::raw::c_void;

#[no_mangle]
pub static ein_string_equal: ffi::Closure =
    ffi::Closure::new(ein_string_equal_entry as *mut c_void, 2);

extern "C" fn ein_string_equal_entry(
    _environment: *const c_void,
    one: ffi::EinString,
    other: ffi::EinString,
) -> usize {
    unsafe {
        (std::slice::from_raw_parts(one.bytes(), one.length())
            == std::slice::from_raw_parts(other.bytes(), other.length())) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::null;

    #[test]
    fn equal_empty_strings() {
        let string = ffi::EinString::new(null(), 0);

        assert_eq!(ein_string_equal_entry(null(), string, string), 1);
    }

    #[test]
    fn equal_one_byte_strings() {
        let string = ffi::EinString::new([0u8].as_ptr(), 1);

        assert_eq!(ein_string_equal_entry(null(), string, string), 1);
    }

    #[test]
    fn not_equal_one_byte_strings() {
        let one = ffi::EinString::new(null(), 0);
        let other = ffi::EinString::new([0u8].as_ptr(), 1);

        assert_eq!(ein_string_equal_entry(null(), one, other), 0);
    }

    #[test]
    fn equal_text_strings() {
        const TEXT: &[u8] = "hello".as_bytes();

        let string = ffi::EinString::new(TEXT.as_ptr(), TEXT.len());

        assert_eq!(ein_string_equal_entry(null(), string, string), 1);
    }

    #[test]
    fn not_equal_text_strings() {
        const TEXT: &[u8] = "hello".as_bytes();
        const OTHER_TEXT: &[u8] = "hell0".as_bytes();

        assert_eq!(
            ein_string_equal_entry(
                null(),
                ffi::EinString::new(TEXT.as_ptr(), TEXT.len()),
                ffi::EinString::new(OTHER_TEXT.as_ptr(), OTHER_TEXT.len()),
            ),
            0
        );
    }
}
