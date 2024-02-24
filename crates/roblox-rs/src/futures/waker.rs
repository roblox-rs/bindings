macro_rules! waker {
    ($name:ident $ty:ty) => {
        pub(super) mod $name {
            #[allow(unused_imports)]
            use super::super::{ffi::RustFutureTask, task::Task};
            use std::{
                mem::forget,
                rc::Rc,
                task::{RawWaker, RawWakerVTable},
            };

            unsafe fn clone(ptr: *const ()) -> RawWaker {
                let ptr = Rc::from_raw(ptr as *const $ty);
                let raw_waker = ptr.clone().raw_waker();
                forget(ptr);
                raw_waker
            }

            unsafe fn wake(ptr: *const ()) {
                let ptr = Rc::from_raw(ptr as *const $ty);
                ptr.wake_by_ref();
            }

            unsafe fn wake_by_ref(ptr: *const ()) {
                let ptr = Rc::from_raw(ptr as *const $ty);
                ptr.wake_by_ref();
                forget(ptr);
            }

            unsafe fn drop(ptr: *const ()) {
                Rc::from_raw(ptr as *const $ty);
            }

            pub const WAKER: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
        }
    };
}

waker!(task_waker Task);
waker!(lua_task_waker RustFutureTask);
