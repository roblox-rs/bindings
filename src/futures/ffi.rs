// Rust future on Lua side
// pub struct RustFuture<F> {}

use std::{
    cell::RefCell,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, RawWaker, Waker},
};

use crate::drop_pointer;

#[cfg(not(feature = "multivalue"))]
use std::collections::HashMap;

use super::waker::lua_task_waker;

extern "C" {
    // Returns the heap allocated value, if any
    fn get_poll_state(future: u32) -> *mut ();
}

#[no_mangle]
fn rors_wake_future(future: u32) {
    #[cfg(not(feature = "multivalue"))]
    WAKERS.with(|v| {
        let mut wakers = v.borrow_mut();
        if let Some(waker) = wakers.get(&future) {
            waker.wake_by_ref();
            wakers.remove(&future);
        }
    });

    #[cfg(feature = "multivalue")]
    WAKERS.with(|v| {
        let mut wakers = v.borrow_mut();
        wakers.retain(|(i, waker)| {
            if *i == future {
                waker.wake_by_ref();
                false
            } else {
                true
            }
        });
    });
}

#[cfg(not(feature = "multivalue"))]
thread_local! {
    static WAKERS: RefCell<HashMap<u32, Waker>> = RefCell::new(HashMap::new());
}

#[cfg(feature = "multivalue")]
thread_local! {
    static WAKERS: RefCell<Vec<(u32, Waker)>> = RefCell::new(Vec::new());
}

/// Yielding lua code future
#[repr(transparent)]
#[must_use = "futures do nothing unless polled"]
pub struct LuaFuture<O>(u32, PhantomData<O>);

impl<O> Future for LuaFuture<O> {
    type Output = O;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 == 0 {
            panic!("attempt to poll dead future");
        }

        unsafe {
            let ptr = get_poll_state(self.0) as *mut O;
            if !ptr.is_null() {
                drop_pointer(self.0);
                self.get_unchecked_mut().0 = 0;

                Poll::Ready(*Box::from_raw(ptr))
            } else {
                #[cfg(not(feature = "multivalue"))]
                WAKERS.with(|v| v.borrow_mut().insert(self.0, cx.waker().clone()));

                #[cfg(feature = "multivalue")]
                WAKERS.with(|v| v.borrow_mut().push((self.0, cx.waker().clone())));

                Poll::Pending
            }
        }
    }
}

extern "C" {
    fn wake_task(task_id: u32);
    fn new_task() -> u32;
}

thread_local! {
    static TASKS: RefCell<Vec<Rc<RustFutureTask>>> = RefCell::new(Vec::new());
}

#[repr(C)]
pub enum TaskResult<T> {
    None,
    Ready(T),
}

pub struct RustFutureTask {
    task_id: u32,
    waker: RefCell<Option<Waker>>,
    future: *mut dyn Future<Output = ()>,
}

impl RustFutureTask {
    pub fn raw_waker(self: Rc<Self>) -> RawWaker {
        RawWaker::new(Rc::into_raw(self) as *const (), &lua_task_waker::WAKER)
    }

    pub fn wake_by_ref(&self) {
        let task_id = self.task_id;
        unsafe { crate::task::defer(move || wake_task(task_id)) }
    }
}

#[repr(C)]
pub struct AsyncStatus(RustFuture, unsafe extern "C" fn(id: u32) -> TaskResult<i32>);

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

#[no_mangle]
pub fn create_async_code() -> AsyncStatus {
    AsyncStatus(
        RustFuture::new(Box::new(async {
            crate::println!("waiting 2 secs");
            crate::task::wait(2.0).await;
            crate::println!("waiting 0.4 secs");
            crate::task::wait(0.4).await;
            crate::println!("returning");
            16
        })),
        RustFuture::poll_future,
    )
}

#[repr(C)]
pub(crate) struct RustFuture(u32);

impl RustFuture {
    pub fn new<O>(future: Box<dyn Future<Output = O>>) -> RustFuture {
        unsafe {
            let raw = Box::into_raw(future);

            let task = Rc::new(RustFutureTask {
                task_id: new_task(),
                waker: RefCell::new(None),
                future: raw as *mut dyn Future<Output = O> as *mut dyn Future<Output = ()>,
            });

            task.waker
                .replace(Some(Waker::from_raw(task.clone().raw_waker())));

            TASKS.with(|cell| {
                let mut borrow = cell.borrow_mut();
                borrow.push(task.clone());
            });

            RustFuture(task.task_id)
        }
    }

    pub(crate) unsafe extern "C" fn poll_future<O>(id: u32) -> TaskResult<O> {
        TASKS.with(|cell| {
            let task = {
                let tasks = cell.borrow();
                if let Some(task) = tasks.iter().find(|v| v.task_id == id) {
                    task.clone()
                } else {
                    return TaskResult::None;
                }
            };

            let waker = task.waker.borrow();
            let mut context = Context::from_waker(waker.as_ref().unwrap());

            // SAFETY:
            // probably not
            let mut future =
                Pin::new_unchecked(Box::from_raw(task.future as *mut dyn Future<Output = O>));

            match future.as_mut().poll(&mut context) {
                Poll::Pending => {
                    // We forget the future here because we don't want to drop it yet
                    std::mem::forget(future);

                    TaskResult::None
                }
                Poll::Ready(data) => {
                    // Remove this task so that subsequent polls will fail and memory will get cleaned up
                    // This won't drop the future but since it's in-scope, it'll be dropped anyways
                    let mut tasks = cell.borrow_mut();
                    tasks.retain(|task| task.task_id != id);

                    TaskResult::Ready(data)
                }
            }
        })
    }
}
