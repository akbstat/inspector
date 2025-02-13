use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub enum Kind {
    SDTM,
    ADaM,
    TFLs,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Group {
    Production,
    Validation,
}

impl Group {
    pub fn group_dir(&self) -> &str {
        match *self {
            Group::Production => "product",
            Group::Validation => "validation",
        }
    }
}

#[derive(Debug, Serialize)]
pub enum FileType {
    Code,
    Data,
    Xpt,
    Output,
    Log,
    Qc,
}
