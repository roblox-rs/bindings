use codegen::{get_namespaces, luau, rust};
use roblox_dump::get_dump;
use std::{env, fs, path::Path};

fn main() {
    let output = env::var("OUT_DIR").unwrap();
    let dump = get_dump();
    let namespaces = get_namespaces(&dump);

    fs::write("../../roblox/abi.luau", luau::generate(&namespaces)).unwrap();
    fs::write(
        Path::new(&output).join("generated.rs"),
        rust::generate(&namespaces, &dump.enums),
    )
    .unwrap();
}
