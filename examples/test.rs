extern crate wee_alloc;
use std::convert::TryFrom;

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

    CollectionService::instance().add_tag(&Workspace::instance().downcast(), "god1");
    CollectionService::instance().add_tag(&Workspace::instance().downcast(), "god2");
    CollectionService::instance().add_tag(&Workspace::instance().downcast(), "god3");

    for tag in CollectionService::instance().get_tags(&Workspace::instance().downcast()) {
        println!("workspace has tag: {tag}");
    }

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

    let my_remote = RemoteEvent::new();
    my_remote.set_name("GOD REMOTE");
    my_remote.set_parent(Some(&Workspace::instance().downcast()));

    let my_func = RemoteFunction::new();
    my_func.set_name("GOD FUNC");
    my_func.set_parent(Some(&Workspace::instance().downcast()));

    my_func.on_server_invoke(move |player, values| {
        println!(
            "Wow, {} sent me messages that I will respond to:",
            player.name()
        );
        for (i, message) in values.into_iter().enumerate() {
            if let Ok(str) = String::try_from(message) {
                println!("got string: {str}");
            } else {
                println!("index {i} is not a string");
            }
        }
        println!("Responding with: 1, Rust, 2");
        vec![1.into(), "Rust".into(), 2.into()]
    });

    my_remote
        .on_server_event(move |player, values| {
            println!("Wow, {} sent me messages:", player.name());
            for (i, message) in values.into_iter().enumerate() {
                if let Ok(str) = String::try_from(message) {
                    println!("got string: {str}");
                } else {
                    println!("index {i} is not a string");
                }
            }
        })
        .leak();

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
