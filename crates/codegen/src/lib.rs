mod config;
pub mod luau;
mod overrides;
pub mod rust;
mod structs;
mod transform_dump;

use config::DATATYPES;
use overrides::{get_datatypes, get_instances, internal_namespace};
use roblox_dump::Dump;
use structs::{Namespace, NamespaceKind};
use transform_dump::transform_dump;

pub fn get_namespaces(dump: &Dump) -> Vec<Namespace> {
    let mut namespaces = transform_dump(dump);

    let instance_overrides = get_instances();
    for partial in instance_overrides.into_iter() {
        if let Some(namespace) = namespaces.iter_mut().find(|v| v.name == partial.name) {
            for member in partial.members.iter() {
                namespace.members.retain(|v| v.api_name != member.api_name);
            }

            for member in partial.members.into_iter() {
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

    namespaces
}
