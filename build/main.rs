mod codegen;
mod config;

use codegen::{
    lang::{luau, rust},
    structs::Dump,
    transform_dump,
};
use std::{env, fs, path::Path};

pub use config::CLASS_BLACKLIST;

fn main() {
    let output = env::var("OUT_DIR").unwrap();
    let dump = transform_dump(
        &serde_json::from_slice::<Dump>(
            fs::read("./dump.json")
                .expect("Dump could not be read.")
                .as_slice(),
        )
        .expect("Could not parse API dump"),
    );

    fs::write(
        Path::new(&output).join("generated.rs"),
        rust::generate(&dump),
    )
    .expect("Could not write generated.rs");

    fs::write("./roblox/abi.luau", luau::generate(&dump)).expect("Could not write abi.luau");
}
