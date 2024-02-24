use crate::internal;

pub fn delay<F: FnMut() + 'static>(amount: f64, value: F) {
    internal::task_delay(amount, value)
}

pub fn defer<F: FnMut() + 'static>(value: F) {
    internal::task_defer(value)
}

pub fn spawn<F: FnMut() + 'static>(value: F) {
    internal::task_spawn(value)
}

pub async fn wait(amount: f64) -> f64 {
    internal::task_wait(amount).await
}
