use super::investigator::File;
use crate::v2::{category::FileType, Group};
use chrono::{DateTime, Local};
use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub const CODE_FILE_EXTENTION: &str = "sas";
pub const OUTPUT_EXTENTION: &str = "rtf";
pub const DATA_FILE_EXTENTION: &str = "sas7bdat";
pub const XPT_FILE_EXTENTION: &str = "xpt";
pub const LOG_FILE_EXTENTION: &str = "log";
pub const QC_FILE_EXTENTION: &str = "rtf";

fn system_time_to_chrono(source: &SystemTime) -> DateTime<Local> {
    let duration_since_epoch = source
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let seconds = duration_since_epoch.as_secs() as i64;
    let nanoseconds = duration_since_epoch.subsec_nanos();
    let naive = DateTime::from_timestamp(seconds, nanoseconds);
    naive.unwrap_or_default().with_timezone(&Local)
}

pub fn file<P: AsRef<Path>>(filepath: P) -> Option<File<P>> {
    let name = filepath.as_ref().file_name()?.to_string_lossy().to_string();
    match fs::metadata(filepath.as_ref().to_path_buf()) {
        Ok(metadata) => Some(File {
            name,
            filepath,
            created_at: system_time_to_chrono(&metadata.created().ok()?),
            modified_at: system_time_to_chrono(&metadata.modified().ok()?),
        }),
        Err(_) => None,
    }
}

pub fn filename(item: &str, group: &Group, kind: &FileType) -> String {
    let item = item.to_lowercase();
    let extention = match kind {
        FileType::Code => CODE_FILE_EXTENTION,
        FileType::Data => DATA_FILE_EXTENTION,
        FileType::Xpt => XPT_FILE_EXTENTION,
        FileType::Output => OUTPUT_EXTENTION,
        FileType::Log => LOG_FILE_EXTENTION,
        FileType::Qc => QC_FILE_EXTENTION,
    };
    let filename = match group {
        Group::Production => format!("{}.{}", item, extention),
        Group::Validation => format!("v-{}.{}", item, extention),
    };
    match kind {
        FileType::Data | FileType::Xpt => filename.replace("-", "_"),
        _ => filename,
    }
}
