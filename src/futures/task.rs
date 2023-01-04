use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, RawWaker, Waker},
};

use super::waker::task_waker;

pub struct TaskFuture {
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
    waker: Waker,
}

/// Rust executor
pub struct Task {
    future: RefCell<Option<TaskFuture>>,
    queued: Cell<bool>,
}

impl Task {
    pub fn new(future: Pin<Box<dyn Future<Output = ()> + 'static>>) -> Rc<Self> {
        let task = Rc::new(Self {
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
        RawWaker::new(Rc::into_raw(self) as *const (), &task_waker::WAKER)
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

// #[repr(C)]
// pub enum TaskResult<T> {
//     Ready(T),
//     None,
// }

// Lua executor
// We don't store the waker as it is up to the future to store it
// pub struct LuaTask {
//     task_id: u32,
//     future: Pin<Box<dyn Future<Output = ()> + 'static>>,
// }

// impl LuaTask {
//     pub fn wake_by_ref(self: &Rc<Self>) {
//         unsafe { wake_task(self.task_id) };
//     }

//     pub fn raw_waker(self: Rc<Self>) -> RawWaker {
//         RawWaker::new(Rc::into_raw(self) as *const (), &lua_task_waker::WAKER)
//     }
// }

// rust bindings pass closure that returns FfiFuture

// (nevermind) lua code creates LuaTask with a unique id (createPointer(awakenFunc))

// rust bindings call new_task which is passed with FfiFuture
// lua bindings update that pointer with the awaker

// rust code calls wake_task with custom waker
// lua code awakenFunc is called and the closure's poll function is called (passed from async closures)

// impl Drop for LuaTask {
//     fn drop(&mut self) {
//         TASKS.with(|v| {
//             let mut tasks = v.borrow_mut();
//             tasks.retain(|v| v.task_id != self.task_id);
//         });
//     }
// }
