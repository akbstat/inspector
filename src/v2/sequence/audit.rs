use super::{adam::AdamSequenceAuditor, sdtm::SdtmSequenceAuditor, tfl::TflSequenceAuditor};
use crate::v2::{
    category::FileType,
    inspect::result::Status,
    investigator::{investigator::File, investigator::Investigator},
    Group, Kind,
};
use chrono::{DateTime, Local};
use serde::Serialize;
use std::path::Path;

pub trait SequenceAuditor {
    fn audit(&self) -> Vec<AuditResult>;
}

#[derive(Debug, Serialize)]
pub struct SequenceResult {
    pub name: String,
    pub kind: FileType,
    pub status: Status,
    pub group: Group,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize)]
pub struct AuditResult {
    pub sequences: Vec<SequenceResult>,
    pub status: Status,
}

pub fn new(
    name: &str,
    kind: &Kind,
    supp: bool,
    investigator: &Investigator,
) -> Box<dyn SequenceAuditor> {
    match kind {
        Kind::SDTM => Box::new(SdtmSequenceAuditor::new(name, supp, investigator)),
        Kind::ADaM => Box::new(AdamSequenceAuditor::new(name, investigator)),
        Kind::TFLs => Box::new(TflSequenceAuditor::new(name, investigator)),
    }
}

pub fn total_status(results: &[SequenceResult]) -> Status {
    let failures = results
        .iter()
        .filter(|result| match result.status {
            Status::Pass => false,
            _ => true,
        })
        .collect::<Vec<_>>();
    match failures.len() {
        0 => Status::Pass,
        _ => Status::Failed("".into()),
    }
}

pub fn auditing<P: AsRef<Path>>(
    base: Option<&File<P>>,
    compare: Option<&File<P>>,
    failed_message: &str,
) -> Status {
    match base {
        Some(base) => {
            if let Some(compare) = compare {
                if compare.modified_at.ge(&base.modified_at) {
                    Status::Failed(failed_message.into())
                } else {
                    Status::Pass
                }
            } else {
                Status::Pass
            }
        }
        None => Status::Missing,
    }
}
