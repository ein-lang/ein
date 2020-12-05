use super::closure::Closure;
use std::os::raw::c_void;

#[repr(C)]
struct EinString {
    length: usize,
    bytes: [u8; 42], // variadic length array
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
        let one_length = (&*one).length;
        let other_length = (&*other).length;

        if one_length != other_length {
            0
        } else {
            let one_slice_pointer = &(&*one).bytes[0];
            let other_slice_pointer = &(&*other).bytes[0];

            (std::slice::from_raw_parts(one_slice_pointer, one_length)
                == std::slice::from_raw_parts(other_slice_pointer, one_length)) as usize
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_empty_strings() {
        let one = EinString {
            length: 0,
            bytes: [0; 42],
        };
        let other = EinString {
            length: 0,
            bytes: [0; 42],
        };

        assert_eq!(ein_string_equal_entry(&one, &other), 1);
    }

    #[test]
    fn equal_one_byte_strings() {
        let one = EinString {
            length: 1,
            bytes: [0; 42],
        };
        let other = EinString {
            length: 1,
            bytes: [0; 42],
        };

        assert_eq!(ein_string_equal_entry(&one, &other), 1);
    }

    #[test]
    fn not_equal_one_byte_strings() {
        let one = EinString {
            length: 0,
            bytes: [0; 42],
        };
        let other = EinString {
            length: 1,
            bytes: [0; 42],
        };

        assert_eq!(ein_string_equal_entry(&one, &other), 0);
    }

    #[test]
    fn equal_full_length_strings() {
        let one = EinString {
            length: 42,
            bytes: [0; 42],
        };
        let other = EinString {
            length: 42,
            bytes: [0; 42],
        };

        assert_eq!(ein_string_equal_entry(&one, &other), 1);
    }

    #[test]
    fn equal_text_strings() {
        const TEXT: &str = "hello";
        const LENGTH: usize = TEXT.as_bytes().len();

        let mut one = EinString {
            length: LENGTH,
            bytes: [0; 42],
        };
        one.bytes[..LENGTH].copy_from_slice(TEXT.as_bytes());

        let mut other = EinString {
            length: LENGTH,
            bytes: [0; 42],
        };
        other.bytes[..LENGTH].copy_from_slice(TEXT.as_bytes());

        assert_eq!(ein_string_equal_entry(&one, &other), 1);
    }

    #[test]
    fn not_equal_text_strings() {
        const TEXT: &str = "hello";
        const LENGTH: usize = TEXT.as_bytes().len();

        let mut one = EinString {
            length: LENGTH,
            bytes: [0; 42],
        };
        one.bytes[..LENGTH].copy_from_slice(TEXT.as_bytes());

        let mut other = EinString {
            length: LENGTH,
            bytes: [0; 42],
        };
        other.bytes[..LENGTH].copy_from_slice(TEXT.as_bytes());
        other.bytes[0] = 'x' as u8;

        assert_eq!(ein_string_equal_entry(&one, &other), 0);
    }
}
