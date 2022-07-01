#![feature(backtrace)]
extern crate wee_alloc;
pub mod bindings;
pub mod util;
use bindings::*;
use std::panic;
use util::error;
use util::println;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[allow(improper_ctypes)]
extern "C" {
    fn get_game() -> DataModel;
}

/// # Safety
#[no_mangle]
#[allow(unreachable_code)]
pub unsafe fn greet() {
    let x = "wow!".to_string();
    let fnn1 = move || {
        println(&format!("Wow, hello {}!", x));
    };

    fnn1();
    fnn1();
    task_delay(3., fnn1);

    let my_fancy_part = Part::new();
    my_fancy_part.set_anchored(true);
    my_fancy_part.set_color(Color3::from_rgb(255., 0., 0.));
    my_fancy_part.set_parent(Some(get_game().get_child("Workspace")));
    my_fancy_part.set_size(Vector3::new(25., 420., 25.));
    my_fancy_part.set_position(Vector3::new(55., 250., 30.));

    panic::set_hook(Box::new(|info| {
        let msg = info.to_string();
        error(&msg);
    }));
}
