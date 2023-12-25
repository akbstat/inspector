use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use std::{fs::read_dir, time::SystemTime};

use anyhow::{Ok, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    directory: String,
    name: String,
    extention: String,
    created_at: u64,
    modified_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Inspector {
    files: Vec<File>,
}

impl Inspector {
    pub fn new() -> Inspector {
        let files = vec![];
        Inspector { files }
    }

    /// collect file metadata information
    pub fn collect(&mut self, dir: &Path) -> Result<()> {
        for item in read_dir(dir)? {
            let file = item?;
            if file.file_type()?.is_file() {
                let metadata = file.metadata()?;
                let (name, extention) = split_filename(file.file_name().to_str().unwrap());
                self.files.push(File {
                    directory: dir.to_str().unwrap().into(),
                    name,
                    extention,
                    created_at: sys_to_unix(metadata.created()?)?,
                    modified_at: sys_to_unix(metadata.modified()?)?,
                });
            } else {
                let next = PathBuf::from(dir).join(file.file_name().to_str().unwrap());
                self.collect(next.as_path())?;
            }
        }
        Ok(())
    }

    /// store the directory information into json format
    pub fn status(&mut self) -> Result<String> {
        self.files
            .sort_by(|f1, f2| f2.modified_at.partial_cmp(&f1.modified_at).unwrap());
        Ok(serde_json::to_string(&self)?)
    }
}

/// convert SystemTime struct into unix timestamp
fn sys_to_unix(st: SystemTime) -> Result<u64> {
    Ok(st.duration_since(UNIX_EPOCH)?.as_secs())
}

/// split filename into two parts, name and extention
fn split_filename(name: &str) -> (String, String) {
    let mut result = ("".into(), "".into());
    let split = name.split(".").collect::<Vec<&str>>();
    if split.len() > 1 {
        let tail = split.len() - 1;
        result.0 = split[..tail].join(".");
        result.1 = split[tail].into();
    } else {
        result.0 = name.into()
    }
    result
}

#[cfg(test)]
mod inpector_test {
    use super::*;
    use std::path::Path;
    #[test]
    fn test() {
        let dir = r"D:\网页下载文件\dingtalk\test-lyh";
        let mut r = Inspector::new();
        r.collect(Path::new(dir)).unwrap();
        let result = r.status().unwrap();
        assert!(result.len() > 0)
    }

    #[test]
    fn test_split_filename() {
        let p = r"钉钉办公常用功能小指引-DM-20210930.pdf";
        let result = split_filename(p);
        assert_eq!("pdf", result.1);
        assert_eq!("钉钉办公常用功能小指引-DM-20210930", result.0);
    }
}
