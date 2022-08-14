use convert_case::{Case, Casing};

use super::structs::{Class, ClassEventMember, ClassFunctionMember, ClassPropertyMember};

pub mod luau;
pub mod rust;

pub fn to_snake(name: &str) -> String {
    name.from_case(Case::Pascal).to_case(Case::Snake)
}

pub fn to_pascal(name: &str) -> String {
    name.from_case(Case::Camel).to_case(Case::Pascal)
}

pub fn raw_name(name: &str) -> String {
    let name = to_snake(name);
    if name == "loop" {
        format!("r#{}", name)
    } else {
        name
    }
}

pub fn is_camel_case(name: &str) -> bool {
    name.chars().next().unwrap().is_lowercase()
}

pub fn get_prop_extern_name(class: &Class, property: &ClassPropertyMember) -> String {
    format!(
        "prop_get_{}_{}",
        to_snake(&class.name),
        to_snake(&property.name)
    )
}

pub fn set_prop_extern_name(class: &Class, property: &ClassPropertyMember) -> String {
    format!(
        "prop_set_{}_{}",
        to_snake(&class.name),
        to_snake(&property.name)
    )
}

pub fn dyn_fn_extern_name(class: &Class, function: &ClassFunctionMember) -> String {
    format!(
        "dyn_fn_{}_{}",
        to_snake(&class.name),
        to_snake(&function.name)
    )
}

pub fn event_extern_name(class: &Class, event: &ClassEventMember) -> String {
    format!(
        "connect_{}_{}",
        to_snake(&class.name),
        to_snake(&event.name)
    )
}
