use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

pub mod luau;
pub mod rust;
pub mod stream;
pub mod structs;

#[derive(Clone)]
pub struct UniqueIds(Cell<u32>, RefCell<HashMap<String, u32>>);

impl UniqueIds {
    pub fn new() -> Self {
        UniqueIds(Cell::new(0), RefCell::new(HashMap::new()))
    }

    pub fn next_names(&self, inputs: &[&str]) -> String {
        let input = inputs.join("_");
        let mut names = self.1.borrow_mut();

        if let Some(&id) = names.get(&input) {
            names.insert(input.clone(), id + 1);
            format!("{input}_{id}")
        } else {
            names.insert(input.clone(), 1);
            input.clone()
        }
    }
}
