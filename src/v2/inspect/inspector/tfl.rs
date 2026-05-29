use std::collections::HashSet;

use super::{InspectionResult, Inspector};
use crate::v2::{
    category::Group,
    config::reader::Config,
    error::{Error, Result},
    inspect::result::{IndividualResult, Status},
    investigator::investigator::Investigator,
    sequence::audit,
    Kind,
};
use lazy_static::lazy_static;
use validator::{
    qc::{evaluator::QcResultEvaluator, html_report::QcResultHtmlParser, QcResultValidator},
    result::ReportResult,
    sas_log::{ExternalLogPattern, SasLogValidatior},
};

lazy_static! {
    static ref QC_EVALUATOR: QcResultEvaluator = QcResultEvaluator::new();
}

pub struct TflInspector {
    investigator: Investigator,
    config: Vec<Config>,
    qc_ignore: Vec<String>,
    external_log_patterns: Option<ExternalLogPattern>,
}

impl TflInspector {
    pub fn new(
        investigator: Investigator,
        config: &[Config],
        qc_ignore: &[String],
        external_log_patterns: Option<ExternalLogPattern>,
    ) -> TflInspector {
        TflInspector {
            investigator,
            config: config.to_vec(),
            qc_ignore: qc_ignore.to_vec(),
            external_log_patterns,
        }
    }

    fn validate_log(&self, item: &str, group: &Group) -> Result<Status> {
        let target_file = self.investigator.tfl_log(item, group);
        match target_file {
            Some(file) => {
                let validator = SasLogValidatior::new(self.external_log_patterns.clone());
                let result = validator
                    .validate(file.filepath)
                    .map_err(|_| Error::LogFailed(item.to_string()))?;
                match result.status {
                    ReportResult::Pass => Ok(Status::Pass),
                    ReportResult::Unknown => Ok(Status::Failed("Unknown error".into())),
                    ReportResult::Fail(msg) => Ok(Status::Failed(msg)),
                }
            }
            None => Ok(Status::Missing),
        }
    }

    fn sequence(&self, item: &str) -> (Status, Status) {
        let sequencer = audit::new(item, &Kind::TFLs, false, &self.investigator);
        let result = sequencer.audit();
        (result[0].status.clone(), result[1].status.clone())
    }

    fn qc(&self, item: &str) -> Result<Status> {
        let target_file = self.investigator.tfl_qc_result(item);
        match target_file {
            Some(file) => {
                if file.filepath.to_string_lossy().ends_with(".html") {
                    let qc_result = QcResultHtmlParser::new().parse(&file.filepath);
                    let result = QC_EVALUATOR.evaluate(&qc_result);
                    match result.status {
                        validator::qc::evaluator::QcStatus::Pass => Ok(Status::Pass),
                        validator::qc::evaluator::QcStatus::Unknown => {
                            Ok(Status::Failed("Unknown error".into()))
                        }
                        validator::qc::evaluator::QcStatus::Failed => {
                            let mut errors =
                                result.error_log.into_iter().collect::<HashSet<String>>();
                            for ignore in self.qc_ignore.iter() {
                                errors.remove(ignore);
                            }
                            Ok(if errors.is_empty() {
                                Status::Pass
                            } else {
                                if errors.len().gt(&1) {
                                    Status::Failed("Multiple".to_string())
                                } else {
                                    let errors = errors.drain().collect::<Vec<String>>();
                                    Status::Failed(errors.get(0).unwrap().clone())
                                }
                            })
                        }
                    }
                } else {
                    let mut qc = QcResultValidator::new(file.filepath, &self.qc_ignore)
                        .map_err(|_| Error::QcFailed)?;
                    let result = qc.validate().map_err(|_| Error::QcFailed)?;
                    match result {
                        ReportResult::Pass => Ok(Status::Pass),
                        ReportResult::Unknown => Ok(Status::Failed("Unknown error".into())),
                        ReportResult::Fail(msg) => Ok(Status::Failed(msg)),
                    }
                }
            }
            None => Ok(Status::Missing),
        }
    }

    fn start_coding(&self, item: &str, group: &Group) -> bool {
        let code_file = match group {
            Group::Production => self.investigator.tfl_code(item, &Group::Production),
            Group::Validation => self.investigator.tfl_code(item, &Group::Validation),
        };
        match code_file {
            Some(file) => file.modified_at.ne(&file.created_at),
            None => false,
        }
    }
}

impl Inspector for TflInspector {
    fn inspect(&self) -> Result<Vec<InspectionResult>> {
        let mut results = Vec::with_capacity(self.config.len());
        for item in self.config.iter() {
            let sequence = self.sequence(&item.name);
            let result = InspectionResult {
                item: item.name.clone(),
                qc: self.qc(&item.name)?,
                qc_supp: None,
                production_result: IndividualResult {
                    start_coding: self.start_coding(&item.name, &Group::Production),
                    log: self.validate_log(&item.name, &Group::Production)?,
                    sequence: sequence.0,
                },
                validation_result: IndividualResult {
                    start_coding: self.start_coding(&item.name, &Group::Validation),
                    log: self.validate_log(&item.name, &Group::Validation)?,
                    sequence: sequence.1,
                },
            };
            results.push(result);
        }
        Ok(results)
    }
}
