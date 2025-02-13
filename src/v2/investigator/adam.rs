use super::{
    investigator::{File, Investigator},
    utils::{file, filename},
};
use crate::v2::category::{FileType, Group};
use std::path::PathBuf;

const CODE_DIR: &str = r"program\adam";
const DATA_DIR: &str = r"dataset\adam";
const QC_DIR: &str = r"qc-result\adam";

impl Investigator {
    pub fn adam_code(&self, item: &str, group: &Group) -> Option<File<PathBuf>> {
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(CODE_DIR)
            .join(filename(item, group, &FileType::Code));
        file(filepath)
    }

    pub fn adam_data(&self, item: &str, group: &Group) -> Option<File<PathBuf>> {
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(DATA_DIR)
            .join(filename(item, group, &FileType::Data));
        file(filepath)
    }

    pub fn adam_xpt(&self, item: &str) -> Option<File<PathBuf>> {
        let filepath = self
            .root()
            .join(Group::Production.group_dir())
            .join(DATA_DIR)
            .join(filename(item, &Group::Production, &FileType::Xpt));
        file(filepath)
    }

    pub fn adam_log(&self, item: &str, group: &Group) -> Option<File<PathBuf>> {
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(CODE_DIR)
            .join(filename(item, group, &FileType::Log));
        file(filepath)
    }

    pub fn adam_qc_result(&self, item: &str) -> Option<File<PathBuf>> {
        let filepath = self
            .root()
            .join(Group::Validation.group_dir())
            .join(QC_DIR)
            .join(filename(item, &Group::Validation, &FileType::Qc));
        file(filepath)
    }
}
