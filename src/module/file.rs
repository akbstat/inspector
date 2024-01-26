use std::{
    borrow::Borrow,
    cell::{Cell, Ref, RefCell},
};

use serde::Serialize;

use super::GroupKind;

#[derive(Debug, Default, Clone)]
pub struct File {
    name: String,
    size: Cell<u64>,
    required: Cell<bool>,
    modified_at: Cell<u64>,
    kind: RefCell<FileKind>,
    supp: Cell<bool>,
    status: RefCell<FileStatus>,
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name: name.into(),
            ..Default::default()
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn set_size(&self, size: u64) -> &Self {
        self.size.borrow().set(size);
        self
    }
    // if file size is less than 5KB than regard it as not start
    pub fn start_edit(&self, init_size: u64) -> bool {
        self.size.borrow().get() > init_size
    }
    pub fn require(&self) -> &Self {
        self.required.set(true);
        self
    }
    pub fn not_require(&self) -> &Self {
        self.required.set(false);
        self
    }
    pub fn is_required(&self) -> bool {
        self.required.get()
    }
    pub fn set_kind(&self, kind: FileKind) -> &Self {
        *self.kind.borrow_mut() = kind;
        self
    }
    pub fn kind(&self) -> Ref<FileKind> {
        self.kind.borrow()
    }
    pub fn update_modified_at(&self, timestamp: u64) -> &Self {
        self.modified_at.set(timestamp);
        self
    }
    pub fn modified_at(&self) -> u64 {
        self.modified_at.get()
    }
    pub fn fine(&self) -> &Self {
        *self.status.borrow_mut() = FileStatus::Fine;
        self
    }
    pub fn unexpected(&self) -> &Self {
        *self.status.borrow_mut() = FileStatus::Unexpected;
        self
    }
    pub fn status(&self) -> Ref<FileStatus> {
        self.status.borrow()
    }
    pub fn missing(&self) -> &Self {
        *self.status.borrow_mut() = FileStatus::Missing;
        self
    }
    pub fn is_not_match(&self) -> bool {
        self.status.borrow().eq(&FileStatus::NotMatch)
    }
    pub fn not_match(&self) -> &Self {
        *self.status.borrow_mut() = FileStatus::NotMatch;
        self
    }
    // pub fn is_fine(&self) -> bool {
    //     self.status.borrow().eq(&FileStatus::Fine)
    // }
    // pub fn is_unexpected(&self) -> bool {
    //     self.status.borrow().eq(&FileStatus::Unexpected)
    // }
    pub fn is_missing(&self) -> bool {
        self.status.borrow().eq(&FileStatus::Missing)
    }
    pub fn supp(&self) -> &Self {
        self.supp.set(true);
        self
    }
    pub fn is_supp(&self) -> bool {
        self.supp.get()
    }
    pub fn equal(&self, f: &File) -> bool {
        if self.is_missing() || f.is_missing() {
            return false;
        }
        self.name().eq(f.name()) && self.kind().eq(&f.kind())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum FileKind {
    SasData,
    SasCode,
    SasLog,
    Output,
    QcResult,
    Xpt,
    Input,
    #[default]
    Other,
}

impl FileKind {
    pub fn filename(&self, name: &str, kind: GroupKind) -> String {
        let mut filename = match self {
            FileKind::SasCode => format!("{}.sas", name),
            FileKind::SasData => format!("{}.sas7bdat", name),
            FileKind::SasLog => format!("{}.log", name),
            FileKind::Output => format!("{}.rtf", name),
            FileKind::QcResult => format!("{}.rtf", name),
            FileKind::Xpt => format!("{}.xpt", name),
            _ => name.into(),
        };
        if let GroupKind::Qc = kind {
            match self {
                FileKind::SasData => {
                    filename = format!("v_{}", filename);
                }
                FileKind::SasCode => {
                    filename = format!("v-{}", filename);
                }
                FileKind::SasLog => {
                    filename = format!("v-{}", filename);
                }
                FileKind::Output => {
                    filename = format!("v-{}", filename);
                }
                FileKind::QcResult => {
                    filename = format!("v_{}", filename);
                }
                FileKind::Xpt => {
                    filename = format!("v_{}", filename);
                }
                _ => {}
            }
        }

        filename
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize)]
pub enum FileStatus {
    Fine,
    #[default]
    Missing,
    Unexpected,
    NotMatch,
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn create_file_test() {
        let filename = "ae";
        let f = File::new(filename);
        assert_eq!(filename, f.name());
        assert!(!f.is_required());
        assert!(f.is_missing());
        assert!(f.kind().borrow().eq(&FileKind::Other));
        assert_eq!(0, f.modified_at());
        let timestamp = 1000;
        f.set_kind(FileKind::SasCode)
            .fine()
            .require()
            .update_modified_at(timestamp);
        assert!(f.is_required());
        // assert!(f.is_fine());
        assert!(f.kind().borrow().eq(&FileKind::SasCode));
        assert_eq!(timestamp, f.modified_at());
        assert!(!f.is_supp());
        assert!(f.supp().is_supp());
    }

    #[test]
    fn filename_test() {
        let name = "ae";
        assert_eq!(FileKind::SasCode.filename(name, GroupKind::Dev), "ae.sas");
        assert_eq!(FileKind::SasCode.filename(name, GroupKind::Qc), "v-ae.sas");
        assert_eq!(FileKind::SasLog.filename(name, GroupKind::Qc), "v-ae.log");
        assert_eq!(
            FileKind::SasData.filename(name, GroupKind::Qc),
            "v_ae.sas7bdat"
        );
    }
}
