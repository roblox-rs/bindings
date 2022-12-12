use std::{
    mem::forget,
    rc::Rc,
    task::{RawWaker, RawWakerVTable},
};

use super::task::Task;

unsafe fn clone(ptr: *const ()) -> RawWaker {
    let ptr = Rc::from_raw(ptr as *const Task);
    let raw_waker = ptr.clone().raw_waker();
    forget(ptr);
    raw_waker
}

unsafe fn wake(ptr: *const ()) {
    let ptr = Rc::from_raw(ptr as *const Task);
    ptr.wake_by_ref();
}

unsafe fn wake_by_ref(ptr: *const ()) {
    let ptr = Rc::from_raw(ptr as *const Task);
    ptr.wake_by_ref();
    forget(ptr);
}

unsafe fn drop(ptr: *const ()) {
    Rc::from_raw(ptr as *const Task);
}

pub(super) const WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
