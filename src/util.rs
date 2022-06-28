extern "C" {
    fn stream(s: u8);
    fn emit();
    fn emit_error() -> !;
}

pub fn print(str: &str) {
    for x in str.as_bytes() {
        unsafe { stream(*x) };
    }
}

pub fn println(str: &str) {
    print(str);
    unsafe {
        emit();
    }
}

pub fn error(str: &str) -> ! {
    print(str);
    unsafe {
        emit_error();
    }
}
