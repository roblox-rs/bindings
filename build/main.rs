mod codegen;
mod config;
mod dump;
mod overrides;
// mod macros;

use codegen::{
    luau, rust,
    structs::{Namespace, NamespaceKind},
};
use config::DATATYPES;
use dump::{transform_dump, types::Dump};
use overrides::{get_datatypes, get_instances, internal_namespace};
use std::{env, fs, path::Path};

fn main() {
    let output = env::var("OUT_DIR").unwrap();
    let dump = &serde_json::from_slice::<Dump>(
        fs::read("./dump.json")
            .expect("Dump could not be read.")
            .as_slice(),
    )
    .expect("could not deserialize dump");

    let mut namespaces = transform_dump(dump);

    let instance_overrides = get_instances();
    for partial in instance_overrides.into_iter() {
        if let Some(namespace) = namespaces.iter_mut().find(|v| v.name == partial.name) {
            for member in partial.members.into_iter() {
                namespace.members.retain(|v| v.api_name != member.api_name);
                namespace.members.push(member);
            }
        }
    }

    let internal_namespace = {
        let partial = internal_namespace();

        Namespace {
            kind: NamespaceKind::Functions,
            name: partial.name,
            extends: None,
            members: partial.members,
        }
    };

    let mut namespaces = namespaces
        .into_iter()
        .chain([internal_namespace])
        .chain(get_datatypes().into_iter().map(|v| Namespace {
            kind: NamespaceKind::DataType,
            name: v.name.clone(),
            extends: None,
            members: v.members,
        }))
        .collect::<Vec<_>>();

    // Generate datatypes even if they're not yet typed
    for datatype in DATATYPES {
        if namespaces.iter().all(|v| v.name != datatype) {
            namespaces.push(Namespace {
                kind: NamespaceKind::DataType,
                name: datatype.to_string(),
                extends: None,
                members: vec![],
            });
        }
    }

    fs::write("./roblox/abi.luau", luau::generate(&namespaces)).unwrap();
    fs::write(
        Path::new(&output).join("generated.rs"),
        rust::generate(&namespaces, &dump.enums),
    )
    .unwrap();
}
