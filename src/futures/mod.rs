pub mod ffi;
mod task;
mod waker;

use std::future::Future;

use self::task::Task;

pub fn spawn<F: Future<Output = ()> + 'static>(future: F) {
    Task::new(Box::pin(future)).execute();
}
