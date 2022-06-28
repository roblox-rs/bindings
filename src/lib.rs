#![feature(backtrace)]
extern crate wee_alloc;
pub mod datatypes;
pub mod instances;
pub mod util;
use std::convert::TryFrom;

use crate::datatypes::Vector3;
use instances::{BasePart, DataModel, Instance};
use std::panic;
use util::error;
use util::println;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// # Safety
#[no_mangle]
pub unsafe fn greet() {
    // println(&format!(
    //     "vector3: {}",
    //     Vector3::new(5.0, 5.0, 5.0) + Vector3::new(4.0, 5.0, 4.0) / Vector3::new(0.5, 2., 5.)
    // ));
    // option_test("None", None);
    // option_test("Some", Some(25));

    panic::set_hook(Box::new(|info| {
        let msg = info.to_string();

        // msg.push_str("\n\nStack:\n\n");
        // let e = Backtrace::capture();
        // let stack = e.to_string();
        // msg.push_str(&stack);

        // msg.push_str("\n\n");

        error(&msg);
    }));

    let game = DataModel::instance()
        .child("Workspace")
        .unwrap()
        .parent()
        .unwrap();

    // println(&format!("{} + {} = {}", 69f64, 420f64, 69f64 + 420f64));

    if let Ok(part) =
        BasePart::try_from(game.child("Workspace").unwrap().child("Baseplate").unwrap())
    {
        println(&format!("can collide? {}", part.can_collide()));
        part.set_can_collide(!part.can_collide());
        println(&format!("can collide after set? {}", part.can_collide()));

        println(&format!("part pos: {}", part.position()));
        part.set_position(part.position() + Vector3::new(0., 15., 0.));
        println(&format!("part pos after move: {}", part.position()));
    }

    game.child("Workspace")
        .unwrap()
        .child("Baseplate")
        .unwrap()
        .set_parent(game.child("Workspace").unwrap().child("Terrain"));

    match DataModel::try_from(game) {
        Ok(game) => {
            println("Successfully converted to data model!");
            println(
                &game
                    .child("Workspace")
                    .unwrap()
                    .child("Terrain")
                    .unwrap()
                    .child("Baseplate")
                    .unwrap()
                    .name(),
            );
        }
        Err(_) => println("Could not convert to data model :("),
    };
}
