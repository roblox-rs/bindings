mod types;

pub use types::*;

pub fn get_dump() -> Dump {
    serde_json::from_str::<Dump>(include_str!("../dump.json")).expect("could not deserialize dump")
}
