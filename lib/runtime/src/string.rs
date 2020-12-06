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
        (std::slice::from_raw_parts(one.bytes, one.length)
            == std::slice::from_raw_parts(other.bytes, other.length)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::null;

    #[test]
    fn equal_empty_strings() {
        let one = ffi::EinString {
            length: 0,
            bytes: null(),
        };
        let other = ffi::EinString {
            length: 0,
            bytes: null(),
        };

        assert_eq!(ein_string_equal_entry(null(), one, other), 1);
    }

    #[test]
    fn equal_one_byte_strings() {
        let buffer = [0u8];
        let one = ffi::EinString {
            length: 1,
            bytes: &buffer as *const u8,
        };
        let other = ffi::EinString {
            length: 1,
            bytes: &buffer as *const u8,
        };

        assert_eq!(ein_string_equal_entry(null(), one, other), 1);
    }

    #[test]
    fn not_equal_one_byte_strings() {
        let buffer = [0u8];
        let one = ffi::EinString {
            length: 0,
            bytes: null(),
        };
        let other = ffi::EinString {
            length: 1,
            bytes: &buffer as *const u8,
        };

        assert_eq!(ein_string_equal_entry(null(), one, other), 0);
    }

    #[test]
    fn equal_text_strings() {
        const TEXT: &str = "hello";
        const LENGTH: usize = TEXT.as_bytes().len();

        let one = ffi::EinString {
            bytes: TEXT.as_bytes().as_ptr(),
            length: LENGTH,
        };

        let other = ffi::EinString {
            bytes: TEXT.as_bytes().as_ptr(),
            length: LENGTH,
        };

        assert_eq!(ein_string_equal_entry(null(), one, other), 1);
    }

    #[test]
    fn not_equal_text_strings() {
        const TEXT: &str = "hello";
        const OTHER_TEXT: &str = "hell0";
        const LENGTH: usize = TEXT.as_bytes().len();

        let one = ffi::EinString {
            bytes: TEXT.as_bytes().as_ptr(),
            length: LENGTH,
        };

        let other = ffi::EinString {
            bytes: OTHER_TEXT.as_bytes().as_ptr(),
            length: LENGTH,
        };

        assert_eq!(ein_string_equal_entry(null(), one, other), 0);
    }
}
