use serde::Serialize;

use crate::module::{FileKind, FileStatus, GroupStatus};

#[derive(Debug, Serialize)]
pub struct Module {
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize)]
pub struct Item {
    pub name: String,
    pub timeline: Vec<File>,
    pub groups: Vec<Group>,
}

#[derive(Debug, Serialize)]
pub struct Group {
    pub status: GroupStatus,
    pub files: Vec<File>,
}

#[derive(Debug, Serialize, Clone)]
pub struct File {
    pub status: FileStatus,
    pub name: String,
    pub kind: FileKind,
    pub modified_at: u64,
}
