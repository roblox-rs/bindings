use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, RawWaker, Waker},
};

use super::waker::WAKER_VTABLE;

pub struct TaskFuture {
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
    waker: Waker,
}

pub struct Task {
    future: RefCell<Option<TaskFuture>>,
    queued: Cell<bool>,
}

impl Task {
    pub fn new(future: Pin<Box<dyn Future<Output = ()> + 'static>>) -> Rc<Task> {
        let task = Rc::new(Task {
            future: RefCell::new(None),
            queued: Cell::new(false),
        });

        let waker = unsafe { Waker::from_raw(task.clone().raw_waker()) };

        task.future.replace(Some(TaskFuture { future, waker }));
        task
    }

    pub fn wake_by_ref(self: &Rc<Self>) {
        if !self.queued.replace(true) {
            let task = self.clone();
            crate::task::defer(move || task.execute());
        }
    }

    pub fn raw_waker(self: Rc<Self>) -> RawWaker {
        RawWaker::new(Rc::into_raw(self) as *const (), &WAKER_VTABLE)
    }

    pub fn execute(&self) {
        self.queued.replace(false);

        let mut task_future = self.future.borrow_mut();
        let future = match task_future.as_mut() {
            Some(future) => future,
            None => return,
        };

        let mut context = Context::from_waker(&future.waker);
        let status = future.future.as_mut().poll(&mut context);

        if status.is_ready() {
            self.future.replace(None);
        }
    }
}
