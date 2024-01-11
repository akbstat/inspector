use std::cell::{Ref, RefCell};

use serde::Serialize;

use super::file::{File, FileKind};

#[derive(Debug, Default, Serialize, Clone, Copy)]
pub enum GroupStatus {
    Building,
    Unexpected,
    Changed,
    Ready,
    NotMatch,
    Pass,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum GroupKind {
    Dev,
    Qc,
}

#[derive(Debug, Default)]
pub struct Group {
    status: RefCell<GroupStatus>,
    code: RefCell<File>,
    dataset: RefCell<(File, File)>,
    xpt: RefCell<(File, File)>,
    log: RefCell<File>,
    output: RefCell<File>,
    qc: RefCell<(File, File)>,
}

impl Group {
    pub fn new() -> Group {
        Group::default()
    }
    pub fn set_status(&self, status: GroupStatus) -> &Self {
        *self.status.borrow_mut() = status;
        self
    }
    pub fn status(&self) -> Ref<GroupStatus> {
        self.status.borrow()
    }
    pub fn set_file(&self, f: File) -> &Self {
        let kind = *f.kind();
        match kind {
            FileKind::SasData => {
                if f.is_supp() {
                    (*self.dataset.borrow_mut()).1 = f;
                } else {
                    (*self.dataset.borrow_mut()).0 = f;
                }
            }
            FileKind::SasCode => {
                *self.code.borrow_mut() = f;
            }
            FileKind::SasLog => {
                *self.log.borrow_mut() = f;
            }
            FileKind::Output => {
                *self.output.borrow_mut() = f;
            }
            FileKind::QcResult => {
                if f.is_supp() {
                    (*self.qc.borrow_mut()).1 = f;
                } else {
                    (*self.qc.borrow_mut()).0 = f;
                }
            }
            FileKind::Xpt => {
                if f.is_supp() {
                    (*self.xpt.borrow_mut()).1 = f;
                } else {
                    (*self.xpt.borrow_mut()).0 = f;
                }
            }
            _ => {}
        }
        self
    }
    pub fn set_files(&self, files: Vec<File>) -> &Self {
        for f in files {
            self.set_file(f);
        }
        self
    }
    /// get the copies of files accriding kind, operations on copies will not
    /// effect the files in item
    pub fn get_file_copies(&self, kind: FileKind) -> (File, File) {
        let empty = RefCell::new(File::new(""));
        let result = match kind {
            FileKind::SasData => {
                let dataset = self.dataset.borrow();
                (dataset.0.clone(), dataset.1.clone())
            }
            FileKind::SasCode => (self.code.borrow().clone(), empty.borrow().clone()),
            FileKind::SasLog => (self.log.borrow().clone(), empty.borrow().clone()),
            FileKind::Output => (self.output.borrow().clone(), empty.borrow().clone()),
            FileKind::QcResult => {
                let qc = self.qc.borrow();
                return (qc.0.clone(), qc.1.clone());
            }
            FileKind::Xpt => {
                let xpt = self.xpt.borrow();
                return (xpt.0.clone(), xpt.1.clone());
            }
            _ => (empty.borrow().clone(), empty.borrow().clone()),
        };
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_item_test() {
        let name = "ae";
        let i = Group::new();
        let data = File::new(name);
        let code = File::new(name);
        code.set_kind(FileKind::SasCode);
        data.set_kind(FileKind::SasData);
        i.set_files(vec![code, data]);
        let code = i.get_file_copies(FileKind::SasCode).0;
        code.fine();
        // should not effect the code file inside item
        // assert!(!i.get_file_copies(FileKind::SasCode).0.is_fine());
        i.set_file(code);
        // assert!(i.get_file_copies(FileKind::SasCode).0.is_fine());
        let data = i.get_file_copies(FileKind::SasData).0;
        data.fine();
        // should not effect the data file inside item
        // assert!(!i.get_file_copies(FileKind::SasData).0.is_fine());
        i.set_file(data);
        // assert!(i.get_file_copies(FileKind::SasData).0.is_fine());
    }
}
