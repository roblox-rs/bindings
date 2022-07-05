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

/// # Safety
#[no_mangle]
#[allow(unreachable_code)]
pub unsafe fn greet() {
    let x = "wow!";
    let fnn1 = move || {
        println(&format!("Wow, hello {}!", x));
    };

    task_delay(3., fnn1);

    let my_fancy_part = Part::new();
    my_fancy_part.set_anchored(true);
    my_fancy_part.set_color(Color3::from_rgb(255., 0., 0.));
    my_fancy_part.set_parent(Some(Workspace::instance().downcast()));
    my_fancy_part.set_size(Vector3::new(25., 420., 25.));
    my_fancy_part.set_position(Vector3::new(55., 160., 30.));

    my_fancy_part
        .on_touched(|value| {
            println(&format!("on_touched called!! {}", value.is_some()));
            if let Some(value) = value {
                println("I ACTUALLY GOT A BASE PART?!?!?");
                println(&format!(
                    "base part is a: {} and is named: {}",
                    &value.class_name(),
                    &value.name()
                ));
            }
        })
        .leak();

    let value = Vector3Value::new();
    value
        .on_changed(|v: Vector3| {
            println(&format!(
                "vector changed: {} {} {}",
                v.x() as i32,
                v.y() as i32,
                v.z() as i32
            ));
        })
        .leak();

    value.set_value(Vector3::new(1., 0., 1.));
    value.set_value(Vector3::new(0., 2., 1.));

    panic::set_hook(Box::new(|info| {
        let msg = info.to_string();
        error(&msg);
    }));
}
