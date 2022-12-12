// Rust future on Lua side
// pub struct RustFuture<F> {}

use std::{
    cell::RefCell,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use crate::drop_pointer;

#[cfg(not(feature = "multivalue"))]
use collections::HashMap;

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
