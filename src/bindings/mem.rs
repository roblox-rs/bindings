#[no_mangle]
fn __alloc_string(capacity: usize) -> String {
    String::with_capacity(capacity)
}

#[no_mangle]
fn __drop_box_dyn_fn(_ptr: Box<dyn Fn(String, bool) -> String>) {}
