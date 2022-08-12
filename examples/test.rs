#![feature(backtrace)]
#![feature(ptr_metadata, fn_traits)]
#![feature(unsize)]
// extern crate roblox_rs;
extern crate wee_alloc;
// pub mod bindings;
use roblox_rs::{println, *};

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// # Safety
#[allow(unreachable_code)]
pub fn main() {
    let x = "wow!!";
    let fnn1 = move || {
        println!("Wow, hello {}!", x);
    };

    task::delay(3., fnn1);

    let value = "Workspace".to_string();

    let connection = Workspace::instance().on_instance_changed(|name| {
        println!("On instance changed!!!, {}", name);
    });

    task::delay(4.0, move || {
        connection.disconnect();
    });

    if Workspace::instance().name() != "Workspace" {
        println!("BYPASSED, YOU CAN'T SET WORKSPACE NAME I DO!");
        Workspace::instance().set_name("Workspace");
    }

    println!(
        "wow: {}",
        DataModel::instance()
            .find_first_child(&value, false)
            .expect("NO INSTANCE?!?!")
            .name()
    );

    println!(
        "wow: {:?}",
        DataModel::instance()
            .find_first_child("notWorkspace", false)
            .map(|v| v.name())
    );

    // std::mem::transmute::<_, Instance>(0).set_name("hello god");
    // std::mem::transmute::<_, Instance>(0).name();

    let my_fancy_part = Part::new();
    my_fancy_part.set_anchored(true);
    my_fancy_part.set_color(&Color3::from_rgb(255., 0., 0.));
    my_fancy_part.set_parent(Some(&Workspace::instance().downcast()));
    my_fancy_part.set_size(&Vector3::new(25., 420., 25.));
    my_fancy_part.set_position(&Vector3::new(55., 160., 30.));

    my_fancy_part
        .on_touched(|value| {
            println!("I ACTUALLY GOT A BASE PART?!?!?");
            println!(
                "base part is a: {} and is named: {}",
                &value.class_name(),
                &value.name()
            );
        })
        .leak();

    let value = Vector3Value::new();
    value
        .on_changed(|v: Vector3| {
            println!(
                "vector changed: {} {} {}",
                v.x() as i32,
                v.y() as i32,
                v.z() as i32
            );
        })
        .leak();

    value.set_value(&Vector3::new(1., 0., 1.));
    value.set_value(&Vector3::new(0., 2., 1.));
}
