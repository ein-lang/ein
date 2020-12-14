use std::os::raw::c_void;

#[no_mangle]
pub static _ein_equal_strings: ffi::Closure = ffi::Closure::new(equal_strings as *mut c_void, 2);

extern "C" fn equal_strings(
    _environment: *const c_void,
    one: ffi::EinString,
    other: ffi::EinString,
) -> ffi::Boolean {
    (one.as_slice() == other.as_slice()).into()
}

#[no_mangle]
pub static _ein_join_strings: ffi::Closure = ffi::Closure::new(join_strings as *mut c_void, 2);

extern "C" fn join_strings(
    _environment: *const c_void,
    one: ffi::EinString,
    other: ffi::EinString,
) -> ffi::EinString {
    one.join(&other)
}

#[no_mangle]
pub static _ein_slice_string: ffi::Closure = ffi::Closure::new(slice_string as *mut c_void, 3);

extern "C" fn slice_string(
    _environment: *const c_void,
    string: ffi::EinString,
    start: ffi::Number,
    end: ffi::Number,
) -> ffi::EinString {
    string.slice(start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::null;

    #[test]
    fn equal_empty_strings() {
        let string = ffi::EinString::new(null(), 0);

        assert_eq!(equal_strings(null(), string, string), true.into());
    }

    #[test]
    fn equal_one_byte_strings() {
        let string = ffi::EinString::new([0u8].as_ptr(), 1);

        assert_eq!(equal_strings(null(), string, string), true.into());
    }

    #[test]
    fn not_equal_one_byte_strings() {
        let one = ffi::EinString::new(null(), 0);
        let other = ffi::EinString::new([0u8].as_ptr(), 1);

        assert_eq!(equal_strings(null(), one, other), false.into());
    }

    #[test]
    fn equal_text_strings() {
        const TEXT: &[u8] = "hello".as_bytes();

        let string = ffi::EinString::new(TEXT.as_ptr(), TEXT.len());

        assert_eq!(equal_strings(null(), string, string), true.into());
    }

    #[test]
    fn not_equal_text_strings() {
        const TEXT: &[u8] = "hello".as_bytes();
        const OTHER_TEXT: &[u8] = "hell0".as_bytes();

        assert_eq!(
            equal_strings(
                null(),
                ffi::EinString::new(TEXT.as_ptr(), TEXT.len()),
                ffi::EinString::new(OTHER_TEXT.as_ptr(), OTHER_TEXT.len()),
            ),
            false.into()
        );
    }
}
