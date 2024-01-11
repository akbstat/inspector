use anyhow::Result;
use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

mod infer;

pub use infer::ProjectDirInfer;

/// convert SystemTime struct into unix timestamp
pub fn sys_to_unix(st: SystemTime) -> Result<u64> {
    Ok(st.duration_since(UNIX_EPOCH)?.as_secs())
}

/// get the timestamp of latest file in folder
pub fn latest_timestamp(path: &Path) -> Result<u64> {
    let mut timestamp = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            continue;
        }
        let t = sys_to_unix(entry.metadata()?.modified()?)?;
        if t > timestamp {
            timestamp = t
        }
    }
    Ok(timestamp)
}
