use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub enum Kind {
    SDTM,
    ADaM,
    TFLs,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum FileType {
    Code,
    Data,
    Xpt,
    Output,
    Log,
    Qc,
}

impl FileType {
    pub fn extention(&self) -> String {
        let extention = match self {
            FileType::Code => "sas",
            FileType::Data => "sas7bdat",
            FileType::Xpt => "xpt",
            FileType::Output => "rtf",
            FileType::Log => "log",
            FileType::Qc => "rtf",
        };
        extention.into()
    }
}
