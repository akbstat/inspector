use crate::module::{FileKind, FileStatus};
use crate::tfls::Inspector;
use anyhow::Result;
use std::path::Path;

use crate::module::Paths;

use super::module::{File, Group, Item, Module};

pub fn inspect_tfls(top: &Path, root: &Path) -> Result<Module> {
    let paths = Paths::new(root);
    let inspector = Inspector::new(top, paths)?;
    let m = inspector.module()?;

    let mut module = Module { items: vec![] };
    for i in m.items().iter() {
        let mut item = Item {
            name: i.name().into(),
            timeline: vec![File {
                name: "adam".into(),
                status: FileStatus::Fine,
                modified_at: inspector.latest_adam_data(),
                kind: FileKind::Input,
            }],
            groups: vec![],
        };

        let (dev, qc) = (i.dev(), i.qc());
        let dev_code = dev.get_file_copies(FileKind::SasCode).0;
        let dev_data = dev.get_file_copies(FileKind::SasData).0;
        let dev_output = dev.get_file_copies(FileKind::Output).0;
        let dev_log = dev.get_file_copies(FileKind::SasLog).0;
        let qc_code = qc.get_file_copies(FileKind::SasCode).0;
        let qc_data = qc.get_file_copies(FileKind::SasData).0;
        let qc_log = qc.get_file_copies(FileKind::SasLog).0;
        let qc_result = qc.get_file_copies(FileKind::QcResult).0;

        let mut dev_group = Group {
            status: dev.status().clone(),
            files: vec![],
        };
        [dev_code, dev_data, dev_output, dev_log]
            .iter()
            .filter(|f| f.is_required())
            .for_each(|f| {
                let file = File {
                    name: split_filename(f.name()).0,
                    status: f.status().clone(),
                    kind: f.kind().clone(),
                    modified_at: f.modified_at(),
                };
                if !f.is_missing() {
                    item.timeline.push(file.clone());
                }
                dev_group.files.push(file);
            });
        let mut qc_group = Group {
            status: qc.status().clone(),
            files: vec![],
        };
        [qc_code, qc_data, qc_result, qc_log]
            .iter()
            .filter(|f| f.is_required())
            .for_each(|f| {
                let file = File {
                    name: split_filename(f.name()).0,
                    status: f.status().clone(),
                    kind: f.kind().clone(),
                    modified_at: f.modified_at(),
                };
                if !f.is_missing() {
                    item.timeline.push(file.clone());
                }
                qc_group.files.push(file);
            });

        item.timeline
            .sort_by(|x, y| x.modified_at.partial_cmp(&y.modified_at).unwrap());
        item.groups.push(dev_group);
        item.groups.push(qc_group);
        module.items.push(item);
    }
    Ok(module)
}

// ascii code for char "."
const POINT: u8 = 46;
/// split filename with name and extention, for example:
///
/// ae.xpt => (ae, xpt)
fn split_filename(filename: &str) -> (String, String) {
    let bytes = filename.as_bytes();
    let mut break_pos = bytes.len() - 1;
    while break_pos > 0 && bytes.get(break_pos).unwrap().ne(&POINT) {
        break_pos -= 1;
    }

    if break_pos > 0 {
        (
            filename[..break_pos].into(),
            filename[break_pos + 1..].into(),
        )
    } else {
        (filename.into(), "".into())
    }
}
