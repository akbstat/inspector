use super::{
    investigator::{File, Investigator},
    utils::{file, filename},
};
use crate::v2::category::{FileType, Group};
use std::path::PathBuf;

const CODE_DIR: &str = r"program\tfl";
const DATA_DIR: &str = r"dataset\tfl";
const QC_DIR: &str = r"qc-result\tfl";
const OUTPUT_DIR: &str = r"output";

impl Investigator {
    pub fn tfl_code(&self, item: &str, group: &Group) -> Option<File<PathBuf>> {
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(CODE_DIR)
            .join(filename(item, group, &FileType::Code));
        file(filepath)
    }

    pub fn tfl_data(&self, item: &str, group: &Group) -> Option<File<PathBuf>> {
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(DATA_DIR)
            .join(filename(item, group, &FileType::Data));
        file(filepath)
    }

    pub fn tfl_log(&self, item: &str, group: &Group) -> Option<File<PathBuf>> {
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(CODE_DIR)
            .join(filename(item, group, &FileType::Log));
        file(filepath)
    }

    pub fn tfl_qc_result(&self, item: &str) -> Option<File<PathBuf>> {
        let filepath = self
            .root()
            .join(Group::Validation.group_dir())
            .join(QC_DIR)
            .join(filename(item, &Group::Validation, &FileType::Qc));
        file(filepath)
    }

    pub fn tfl_output(&self, item: &str, group: &Group) -> Option<File<PathBuf>> {
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(OUTPUT_DIR)
            .join(filename(item, group, &FileType::Output));
        file(filepath)
    }
}
