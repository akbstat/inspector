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
use validator::{qc::QcResultValidator, result::ReportResult, sas_log::SasLogValidatior};

pub struct TflInspector {
    investigator: Investigator,
    config: Vec<Config>,
    qc_ignore: Vec<String>,
}

impl TflInspector {
    pub fn new(
        investigator: Investigator,
        config: &[Config],
        qc_ignore: &[String],
    ) -> TflInspector {
        TflInspector {
            investigator,
            config: config.to_vec(),
            qc_ignore: qc_ignore.to_vec(),
        }
    }

    fn validate_log(&self, item: &str, group: &Group) -> Result<Status> {
        let target_file = self.investigator.tfl_log(item, group);
        match target_file {
            Some(file) => {
                let validator = SasLogValidatior::new();
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
        match self.investigator.tfl_qc_result(item) {
            Some(file) => {
                let mut qc = QcResultValidator::new(file.filepath, &self.qc_ignore)
                    .map_err(|_| Error::QcFailed)?;
                let result = qc.validate().map_err(|_| Error::QcFailed)?;
                match result {
                    ReportResult::Pass => Ok(Status::Pass),
                    ReportResult::Unknown => Ok(Status::Failed("Unknown error".into())),
                    ReportResult::Fail(msg) => Ok(Status::Failed(msg)),
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
