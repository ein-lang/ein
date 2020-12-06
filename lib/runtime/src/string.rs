use super::closure::Closure;
use std::os::raw::c_void;

#[repr(C)]
struct EinString {
    bytes: *const u8, // variadic length array
    length: usize,
}

#[no_mangle]
pub static ein_string_equal: Closure = Closure::new(ein_string_equal_entry as *mut c_void, 2);

#[no_mangle]
extern "C" fn ein_string_equal_entry(
    _environment: *const c_void,
    one: *const EinString,
    other: *const EinString,
) -> usize {
    unsafe {
        (std::slice::from_raw_parts((*one).bytes, (*one).length)
            == std::slice::from_raw_parts((*other).bytes, (*other).length)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::null;

    #[test]
    fn equal_empty_strings() {
        let one = EinString {
            length: 0,
            bytes: null(),
        };
        let other = EinString {
            length: 0,
            bytes: null(),
        };

        assert_eq!(ein_string_equal_entry(null(), &one, &other), 1);
    }

    #[test]
    fn equal_one_byte_strings() {
        let buffer = [0u8];
        let one = EinString {
            length: 1,
            bytes: &buffer as *const u8,
        };
        let other = EinString {
            length: 1,
            bytes: &buffer as *const u8,
        };

        assert_eq!(ein_string_equal_entry(null(), &one, &other), 1);
    }

    #[test]
    fn not_equal_one_byte_strings() {
        let buffer = [0u8];
        let one = EinString {
            length: 0,
            bytes: null(),
        };
        let other = EinString {
            length: 1,
            bytes: &buffer as *const u8,
        };

        assert_eq!(ein_string_equal_entry(null(), &one, &other), 0);
    }

    #[test]
    fn equal_text_strings() {
        const TEXT: &str = "hello";
        const LENGTH: usize = TEXT.as_bytes().len();

        let one = EinString {
            bytes: TEXT.as_bytes().as_ptr(),
            length: LENGTH,
        };

        let other = EinString {
            bytes: TEXT.as_bytes().as_ptr(),
            length: LENGTH,
        };

        assert_eq!(ein_string_equal_entry(null(), &one, &other), 1);
    }

    #[test]
    fn not_equal_text_strings() {
        const TEXT: &str = "hello";
        const OTHER_TEXT: &str = "hell0";
        const LENGTH: usize = TEXT.as_bytes().len();

        let one = EinString {
            bytes: TEXT.as_bytes().as_ptr(),
            length: LENGTH,
        };

        let other = EinString {
            bytes: OTHER_TEXT.as_bytes().as_ptr(),
            length: LENGTH,
        };

        assert_eq!(ein_string_equal_entry(null(), &one, &other), 0);
    }
}
