extern "C" {
    fn sloth_main() -> std::os::raw::c_double;
}

#[no_mangle]
pub extern "C" fn main() -> std::os::raw::c_int {
    println!("{}", unsafe { sloth_main() });

    0
}
