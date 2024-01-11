use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};

enum ConfigFileKind {
    SDTM,
    ADAM,
    TOP,
}

pub struct ProjectDirInfer {
    study: PathBuf,
    project: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InferResult {
    root: String,
    config: Vec<String>,
}

impl ProjectDirInfer {
    pub fn new(p: &Path) -> Result<ProjectDirInfer> {
        let target = r"Studies\";
        let slash = r"\";
        let at_least_size = 4;
        let p = p.to_string_lossy().to_string();
        let mut cutter = 0;
        let mut valid = false;
        for (i, _) in p.as_bytes().iter().enumerate() {
            if i < target.len() {
                continue;
            }
            if p[i - target.len()..i].eq(target) {
                cutter = i;
                valid = true;
                break;
            }
        }
        if !valid {
            return Err(anyhow!("Error: invalid project directory"));
        }
        let prefix = PathBuf::from(&p[..cutter]);
        let tail = PathBuf::from(&p[cutter..])
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect::<Vec<String>>();
        if tail.len() < at_least_size {
            return Err(anyhow!("Error: invalid project directory"));
        }
        let study = PathBuf::from(&tail[..2].join(slash));
        let project = PathBuf::from(&tail[..at_least_size].join(slash));
        let study = prefix.join(study);
        let project = prefix.join(project);
        if !(study.exists() && project.exists()) {
            return Err(anyhow!("Error: invalid directory"));
        }
        Ok(ProjectDirInfer { study, project })
    }
    pub fn sdtm(&self) -> Result<InferResult> {
        Ok(InferResult {
            root: self.root().as_os_str().to_string_lossy().to_string(),
            config: self.filter_files(ConfigFileKind::SDTM)?,
        })
    }
    pub fn adam(&self) -> Result<InferResult> {
        Ok(InferResult {
            root: self.root().as_os_str().to_string_lossy().to_string(),
            config: self.filter_files(ConfigFileKind::ADAM)?,
        })
    }
    pub fn tfl(&self) -> Result<InferResult> {
        Ok(InferResult {
            root: self.root().as_os_str().to_string_lossy().to_string(),
            config: self.filter_files(ConfigFileKind::TOP)?,
        })
    }
    pub fn root(&self) -> PathBuf {
        self.project.clone()
    }
    fn documents(&self) -> PathBuf {
        self.study.join("documents")
    }
    fn specs(&self) -> PathBuf {
        self.documents().join("specs")
    }
    fn utility(&self) -> PathBuf {
        self.root().join("utility")
    }
    fn filter_files(&self, kind: ConfigFileKind) -> Result<Vec<String>> {
        let dir = match kind {
            ConfigFileKind::SDTM | ConfigFileKind::ADAM => self.specs(),
            ConfigFileKind::TOP => self.utility(),
        };
        let filter_str = match kind {
            ConfigFileKind::SDTM => "SDTM",
            ConfigFileKind::ADAM => "ADAM",
            ConfigFileKind::TOP => "TOP",
        };
        let mut targets = vec![];
        for i in fs::read_dir(dir.clone())? {
            let i = i?;
            if i.file_type()?.is_dir() {
                continue;
            }
            let name = i.file_name().to_string_lossy().to_string();
            if name.to_uppercase().contains(filter_str) && name.ends_with(".xlsx") {
                targets.push(dir.join(name).as_os_str().to_string_lossy().to_string());
            }
        }
        Ok(targets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prject_dir_infer_test() {
        let i = ProjectDirInfer::new(Path::new(r"D:\Studies\ak112\303\stats\CSR")).unwrap();
        assert_eq!(
            i.specs(),
            Path::new(r"D:\Studies\ak112\303\documents\specs")
        );
        let sdtm = i.sdtm().unwrap();
        assert_eq!(sdtm.config.len(), 1);
        assert_eq!(r"D:\Studies\ak112\303\stats\CSR", sdtm.root);
        let adam = i.adam().unwrap();
        assert_eq!(adam.config.len(), 1);
        let tops = i.tfl().unwrap();
        assert_eq!(tops.config.len(), 1);
    }
}
