#[allow(improper_ctypes)]
extern "C" {
    fn lib_task_delay(amount: f64, task: Box<dyn Fn()>);
    fn lib_task_defer(task: Box<dyn Fn()>);
    fn lib_task_spawn(task: Box<dyn Fn()>);
    fn lib_task_wait(amount: f64) -> f64;
}

pub fn delay<F: Fn() + 'static>(amount: f64, value: F) {
    unsafe { lib_task_delay(amount, Box::new(value)) }
}

pub fn defer<F: Fn() + 'static>(value: F) {
    unsafe { lib_task_defer(Box::new(value)) }
}

pub fn spawn<F: Fn() + 'static>(value: F) {
    unsafe { lib_task_spawn(Box::new(value)) }
}

/// This should not be used until async/await is supported as yielding in Rust code is UB.
#[deprecated]
pub fn wait(amount: f64) -> f64 {
    unsafe { lib_task_wait(amount) }
}
