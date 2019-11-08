#![cfg(not(test))]

extern "C" {
    fn ein_init();
    static ein_main: extern "C" fn(environment: *const u8, argument: f64) -> f64;
}

#[no_mangle]
pub extern "C" fn main() -> std::os::raw::c_int {
    unsafe { ein_init() }

    println!("{}", unsafe { ein_main(std::ptr::null(), 42.0) });

    0
}
