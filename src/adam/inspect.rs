use anyhow::{anyhow, Result};
use qc_judgement::QcJudge;
use std::{
    cell::Cell,
    collections::HashMap,
    fs::{self, Metadata},
    path::Path,
};

use super::read_spec;
use crate::{
    module::{File, FileKind, Group, GroupKind, GroupStatus, Item, Module, Paths},
    utils::{latest_timestamp, sys_to_unix},
};

#[derive(Debug, Default)]
pub struct Inspector {
    specs: Vec<String>,
    paths: Paths,
    latest_sdtm_data: u64,
    dev_program_files_map: HashMap<String, Metadata>,
    qc_program_files_map: HashMap<String, Metadata>,
    dev_data_files_map: HashMap<String, Metadata>,
    qc_data_files_map: HashMap<String, Metadata>,
    qc_files_map: HashMap<String, Metadata>,
}

impl Inspector {
    pub fn new(spec: &Path, paths: Paths) -> Result<Inspector> {
        let specs = read_spec(spec)?;
        let latest_sdtm_data = latest_timestamp(paths.sdtm_dataset(GroupKind::Dev))?;
        let mut i = Inspector {
            specs,
            paths,
            latest_sdtm_data,
            ..Default::default()
        };
        i.dev_program_files_map = i.program_files_map(GroupKind::Dev)?;
        i.qc_program_files_map = i.program_files_map(GroupKind::Qc)?;
        i.dev_data_files_map = i.datasets_files_map(GroupKind::Dev)?;
        i.qc_data_files_map = i.datasets_files_map(GroupKind::Qc)?;
        i.qc_files_map = i.qc_files_map()?;
        Ok(i)
    }

    pub fn module(&self) -> Result<Module> {
        let m = Module::new();
        for name in self.specs.iter() {
            let item = Item::new(name);
            let dev = Group::new();
            let qc = Group::new();
            let dev_code = self.file(name, FileKind::SasCode, GroupKind::Dev)?;
            let dev_log = self.file(name, FileKind::SasLog, GroupKind::Dev)?;
            let dev_data = self.file(name, FileKind::SasData, GroupKind::Dev)?;
            let dev_xpt = self.file(name, FileKind::Xpt, GroupKind::Dev)?;
            let qc_code = self.file(name, FileKind::SasCode, GroupKind::Qc)?;
            let qc_log = self.file(name, FileKind::SasLog, GroupKind::Qc)?;
            let qc_data = self.file(name, FileKind::SasData, GroupKind::Qc)?;
            let qc_result = self.file(name, FileKind::QcResult, GroupKind::Qc)?;

            dev.set_files(vec![dev_code, dev_log, dev_data, dev_xpt]);
            qc.set_files(vec![qc_code, qc_log, qc_data, qc_result]);

            self.update_dev_status(&dev).update_qc_status(&dev, &qc);
            item.set_dev(dev).set_qc(qc);
            m.set_item(item);
        }
        Ok(m)
    }

    fn update_dev_status(&self, group: &Group) -> &Self {
        let code = group.get_file_copies(FileKind::SasCode).0;
        let data = group.get_file_copies(FileKind::SasData);
        let xpt = group.get_file_copies(FileKind::Xpt);
        let log = group.get_file_copies(FileKind::SasLog).0;

        // let mut missing = false;
        let original = vec![&code, &data.0, &xpt.0, &log];
        let expect = original
            .iter()
            .filter(|f| f.is_required())
            .collect::<Vec<&&File>>();
        let mut actual = expect.clone();
        actual.sort_by(|x, y| {
            // if file is missing, then throw it to the tail of vector
            let mut modified_x = x.modified_at();
            let mut modified_y = y.modified_at();
            if x.is_missing() {
                modified_x = u64::MAX;
            }
            if y.is_missing() {
                modified_y = u64::MAX;
            }
            modified_x.partial_cmp(&modified_y).unwrap()
        });
        let status = self.update_status(&expect, &actual, false);
        group
            .set_status(status)
            .set_files(vec![code, data.0, xpt.0, log]);
        self
    }

    fn update_qc_status(&self, dev_group: &Group, qc_group: &Group) -> &Self {
        let code = qc_group.get_file_copies(FileKind::SasCode).0;
        let data = qc_group.get_file_copies(FileKind::SasData);
        let log = qc_group.get_file_copies(FileKind::SasLog).0;
        let qc = qc_group.get_file_copies(FileKind::QcResult);
        let dev_data = dev_group.get_file_copies(FileKind::SasData).0;

        // stage 0, compare code, data and log
        let original = vec![&code, &data.0, &log];
        let expect = original
            .iter()
            .filter(|f| f.is_required())
            .collect::<Vec<&&File>>();
        let mut actual = expect.clone();
        actual.sort_by(|x, y| {
            // if file is missing, then throw it to the tail of vector
            let mut modified_x = x.modified_at();
            let mut modified_y = y.modified_at();
            if x.is_missing() {
                modified_x = u64::MAX;
            }
            if y.is_missing() {
                modified_y = u64::MAX;
            }
            modified_x.partial_cmp(&modified_y).unwrap()
        });
        let mut status = self.update_status(&expect, &actual, true);
        // stage 1, handle qc result
        if status.ne(&GroupStatus::Ready) {
            qc.0.unexpected();
            if qc.1.is_required() {
                qc.1.unexpected();
            }
        } else {
            // qc result for main domain
            let previous = {
                let x = data.0.modified_at();
                let y = dev_data.modified_at();
                if x.gt(&y) {
                    x
                } else {
                    y
                }
            };

            if previous.gt(&qc.0.modified_at()) {
                status = GroupStatus::Unexpected;
                qc.0.unexpected();
            } else if qc.0.is_not_match() {
                status = GroupStatus::NotMatch;
            } else {
                status = GroupStatus::Pass;
            }
        }

        // let mut missing = false;
        // let original = vec![&code, &data.0, &qc.0, &log];
        // let expect = original
        //     .iter()
        //     .filter(|f| f.is_required())
        //     .collect::<Vec<&&File>>();
        // let mut actual = expect.clone();
        // actual.sort_by(|x, y| {
        //     // if file is missing, then throw it to the tail of vector
        //     let mut modified_x = x.modified_at();
        //     let mut modified_y = y.modified_at();
        //     if x.is_missing() {
        //         modified_x = u64::MAX;
        //     }
        //     if y.is_missing() {
        //         modified_y = u64::MAX;
        //     }
        //     modified_x.partial_cmp(&modified_y).unwrap()
        // });
        // let status = self.update_status(&expect, &actual, true);
        qc_group
            .set_status(status)
            .set_files(vec![code, data.0, qc.0, log]);
        self
    }
    /// get the hash map for all files in directory of adam program file, eg
    ///
    /// ```
    /// self.program_files_map(PathKind::dev)
    /// ```
    /// means get the files in dev group of directory of adam programs
    fn program_files_map(&self, kind: GroupKind) -> Result<HashMap<String, Metadata>> {
        let mut code_map = HashMap::new();
        for entry in fs::read_dir(self.paths.adam_code(kind))? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                continue;
            }
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.ends_with(".sas") || filename.ends_with(".log") {
                let meta = entry.metadata()?;
                code_map.insert(filename, meta);
            }
        }
        Ok(code_map)
    }

    /// get the hash map for all files in directory of adam datasets file, eg
    ///
    /// ```
    /// self.datasets_files_map(PathKind::dev)
    /// ```
    /// means get the files in dev group of directory of adam programs
    fn datasets_files_map(&self, kind: GroupKind) -> Result<HashMap<String, Metadata>> {
        let mut data_map = HashMap::new();
        for entry in fs::read_dir(self.paths.adam_dataset(kind))? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                continue;
            }
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.ends_with(".sas7bdat") || filename.ends_with(".xpt") {
                let meta = entry.metadata()?;
                data_map.insert(filename, meta);
            }
        }
        Ok(data_map)
    }

    /// get the hash map for all files in directory of adam datasets file
    fn qc_files_map(&self) -> Result<HashMap<String, Metadata>> {
        let mut qc_map = HashMap::new();
        for entry in fs::read_dir(self.paths.adam_qc())? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                continue;
            }
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.ends_with(".rtf") {
                let meta = entry.metadata()?;
                qc_map.insert(filename, meta);
            }
        }
        Ok(qc_map)
    }

    fn file(&self, domain: &str, file_kind: FileKind, group_kind: GroupKind) -> Result<File> {
        let domain = String::from(domain);
        let file_map = match group_kind {
            GroupKind::Dev => match file_kind {
                FileKind::SasData => &self.dev_data_files_map,
                FileKind::SasCode => &self.dev_program_files_map,
                FileKind::SasLog => &self.dev_program_files_map,
                FileKind::Xpt => &self.dev_data_files_map,
                _ => return Err(anyhow!("Error: invalid filetype for dev group")),
            },
            GroupKind::Qc => match file_kind {
                FileKind::SasData => &self.qc_data_files_map,
                FileKind::SasCode => &self.qc_program_files_map,
                FileKind::SasLog => &self.qc_program_files_map,
                FileKind::QcResult => &self.qc_files_map,
                FileKind::Xpt => &self.qc_data_files_map,
                _ => return Err(anyhow!("Error: invalid filetype for qc group")),
            },
        };
        let filename = file_kind.filename(&domain, group_kind);
        // domain = file_kind.filename(&domain, group_kind);
        let f = File::new(&filename);
        f.require().set_kind(file_kind);
        if let Some(meta) = file_map.get(&filename) {
            f.update_modified_at(sys_to_unix(meta.modified()?)?)
                .set_size(meta.len());
            if f.kind().eq(&FileKind::QcResult) {
                let p = self.paths.adam_qc().join(f.name());
                match QcJudge::new(p.as_path()) {
                    Ok(judge) => {
                        if !judge.judge() {
                            f.not_match();
                        } else {
                            f.fine();
                        }
                    }
                    Err(_) => {
                        f.is_not_match();
                    }
                };
            } else {
                f.fine();
            }
        } else {
            f.missing();
        }
        Ok(f)
    }

    /// update status of files and caculate a group status
    fn update_status(&self, expect: &[&&File], actual: &[&&File], is_qc: bool) -> GroupStatus {
        let mut status = GroupStatus::Ready;
        let missing = Cell::new(false);
        let set_rest_to_unexpected = |i| {
            actual[i..].iter().for_each(|f| {
                if !f.is_missing() {
                    f.unexpected();
                } else {
                    missing.set(true)
                }
            });
        };
        for (i, f) in actual.iter().enumerate() {
            if f.is_missing() {
                status = GroupStatus::Building;
                set_rest_to_unexpected(i);
                break;
            }
            if f.kind().eq(&FileKind::SasCode) && !f.start_edit(init_size(is_qc)) {
                status = GroupStatus::NotStart;
                set_rest_to_unexpected(i + 1);
                break;
            }
            if f.kind().ne(&FileKind::SasCode) && f.modified_at().lt(&self.latest_sdtm_data) {
                status = GroupStatus::Changed;
                set_rest_to_unexpected(i);
                break;
            }
            if !f.equal(expect.get(i).unwrap()) {
                status = GroupStatus::Unexpected;
                set_rest_to_unexpected(i);
                break;
            }
            // if FileKind::QcResult.eq(&f.kind()) {
            //     if f.is_not_match() {
            //         status = GroupStatus::NotMatch;
            //     } else {
            //         if GroupStatus::NotMatch.ne(&status) {
            //             status = GroupStatus::Pass;
            //         }
            //     }
            // }
        }
        if missing.get() && status.ne(&GroupStatus::NotStart) {
            status = GroupStatus::Building;
        }
        status
    }

    pub fn latest_sdtm_data(&self) -> u64 {
        self.latest_sdtm_data
    }
}

fn init_size(is_qc: bool) -> u64 {
    if is_qc {
        return 2900;
    }
    2750
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inspect_test() {
        let spec = Path::new(
            r"D:\Studies\ak112\303\documents\specs\AK112-303 ADaM Specification v0.2.xlsx",
        );
        let root = Path::new(r"D:\Studies\ak112\303\stats\CSR");
        let paths = Paths::new(root);
        let i = Inspector::new(spec, paths).unwrap();
        let m = i.module().unwrap();
        assert_eq!(i.specs.len(), m.items().len());
    }
}
