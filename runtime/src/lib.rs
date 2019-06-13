extern "C" {
    fn sloth_init();
    static sloth_main: extern "C" fn(environment: *const u8, argument: f64) -> f64;
}

#[no_mangle]
pub extern "C" fn main() -> std::os::raw::c_int {
    unsafe { sloth_init() }

    println!("{}", unsafe { sloth_main(std::ptr::null(), 42.0) });

    0
}
