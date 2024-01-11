use std::cell::{Ref, RefCell};

use super::item::Item;

#[derive(Debug, Default)]
pub struct Module {
    // latest_rawdata: u64,
    items: RefCell<Vec<Item>>,
}

impl Module {
    pub fn new() -> Module {
        let m = Module {
            // latest_rawdata,
            ..Default::default()
        };
        m
    }
    // pub fn latest_rawdata(&self) -> u64 {
    //     self.latest_rawdata
    // }
    pub fn set_item(&self, item: Item) -> &Self {
        self.items.borrow_mut().push(item);
        self
    }
    pub fn items(&self) -> Ref<Vec<Item>> {
        self.items.borrow()
    }
}
