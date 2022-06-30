#[allow(improper_ctypes)]
extern "C" {
    fn __lib_task_delay(amount: f64, task: Box<dyn FnOnce()>);
    fn __lib_task_defer(task: Box<dyn FnOnce()>);
    fn __lib_task_spawn(task: Box<dyn FnOnce()>);
    fn __lib_task_wait(amount: f64) -> f64;
}

pub fn task_delay<F: FnOnce() + 'static>(amount: f64, value: F) {
    unsafe { __lib_task_delay(amount, Box::new(value)) }
}

pub fn task_defer<F: FnOnce() + 'static>(value: F) {
    unsafe { __lib_task_defer(Box::new(value)) }
}

pub fn task_spawn<F: FnOnce() + 'static>(value: F) {
    unsafe { __lib_task_spawn(Box::new(value)) }
}

pub fn task_wait(amount: f64) -> f64 {
    unsafe { __lib_task_wait(amount) }
}
