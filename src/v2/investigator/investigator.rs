use chrono::{DateTime, Local};
use std::path::{Path, PathBuf};

pub struct Investigator {
    product: String,
    trial: String,
    purpose: String,
    root: PathBuf,
}

pub struct InvestigatorParam<P: AsRef<Path>> {
    pub product: String,
    pub trial: String,
    pub purpose: String,
    pub root: P,
}

impl Investigator {
    pub fn new<P: AsRef<Path>>(param: &InvestigatorParam<P>) -> Investigator {
        Investigator {
            product: param.product.clone(),
            trial: param.trial.clone(),
            purpose: param.purpose.clone(),
            root: param.root.as_ref().to_path_buf(),
        }
    }
    pub fn root(&self) -> PathBuf {
        self.root
            .join(&self.product)
            .join(&self.trial)
            .join("stats")
            .join(&self.purpose)
    }
}

#[derive(Debug)]
pub struct File<P: AsRef<Path>> {
    pub name: String,
    pub filepath: P,
    pub created_at: DateTime<Local>,
    pub modified_at: DateTime<Local>,
}
