use std::cell::{Ref, RefCell};

use super::group::Group;

#[derive(Debug, Default)]
pub struct Item {
    name: String,
    dev: RefCell<Group>,
    qc: RefCell<Group>,
}

impl Item {
    pub fn new(name: &str) -> Item {
        Item {
            name: name.into(),
            ..Default::default()
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn set_dev(&self, g: Group) -> &Self {
        *self.dev.borrow_mut() = g;
        self
    }
    pub fn set_qc(&self, g: Group) -> &Self {
        *self.qc.borrow_mut() = g;
        self
    }
    pub fn dev(&self) -> Ref<Group> {
        self.dev.borrow()
    }
    pub fn qc(&self) -> Ref<Group> {
        self.qc.borrow()
    }
}
