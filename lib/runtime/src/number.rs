use std::os::raw::c_void;

#[no_mangle]
pub static _ein_number_to_string: ffi::Closure =
    ffi::Closure::new(convert_number_to_string as *mut c_void, 1);

extern "C" fn convert_number_to_string(
    _environment: *const c_void,
    number: ffi::Number,
) -> ffi::EinString {
    format!("{}", f64::from(number)).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::null;

    #[test]
    fn convert_to_string() {
        assert_eq!(convert_number_to_string(null(), 42.0.into()), "42".into());
    }
}
