use serde::Serialize;
use validator::result::ReportResult;

#[derive(Debug, Serialize)]
pub struct InspectionResult {
    pub item: String,
    pub qc: Status,
    #[serde(rename = "qcSupp")]
    pub qc_supp: Option<Status>,
    #[serde(rename = "productionResult")]
    pub production_result: IndividualResult,
    #[serde(rename = "validationResult")]
    pub validation_result: IndividualResult,
}

#[derive(Debug, Serialize)]
pub struct IndividualResult {
    #[serde(rename = "startCoding")]
    pub start_coding: bool,
    pub log: Status,
    pub sequence: Status,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Status {
    Pass,
    Failed(String),
    Missing,
    NotStart,
}

impl Status {
    pub fn is_pass(&self) -> bool {
        match *self {
            Status::Pass => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LogResult {
    pub status: Status,
    pub details: Vec<LogRow>,
}

#[derive(Debug, Serialize)]
pub struct LogRow {
    pub row: usize,
    pub content: String,
    pub pass: bool,
}

#[derive(Debug, Serialize)]
pub struct QcResult {
    #[serde(rename = "itemType")]
    pub item_type: String,
    pub status: Status,
}

impl From<validator::sas_log::LogResult> for LogResult {
    fn from(value: validator::sas_log::LogResult) -> Self {
        let status = match value.status {
            ReportResult::Pass => Status::Pass,
            ReportResult::Unknown => Status::Failed("Unknown error".into()),
            ReportResult::Fail(msg) => Status::Failed(msg),
        };
        LogResult {
            status,
            details: value.details.into_iter().map(|row| row.into()).collect(),
        }
    }
}

impl From<validator::sas_log::FailedRow> for LogRow {
    fn from(value: validator::sas_log::FailedRow) -> Self {
        LogRow {
            row: value.row,
            content: value.content,
            pass: value.pass,
        }
    }
}
