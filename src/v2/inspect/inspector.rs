use super::result::{InspectionResult, LogResult, QcResult, Status};
use crate::v2::{
    category::{Group, Kind},
    error::{Error, Result},
    investigator::investigator::{Investigator, InvestigatorParam},
    read_config,
    sequence::audit::{self, AuditResult},
};
use adam::AdamInspector;
use sdtm::SdtmInspector;
use std::path::Path;
use tfl::TflInspector;
use validator::{qc::QcResultValidator, sas_log::SasLogValidatior};

mod adam;
mod sdtm;
mod tfl;

pub trait Inspector {
    fn inspect(&self) -> Result<Vec<InspectionResult>>;
}

pub fn inspect<P: AsRef<Path>>(
    param: &InvestigatorParam<P>,
    config_file: P,
    kind: &Kind,
    qc_ignore: &[String],
) -> Result<Vec<InspectionResult>> {
    let config = read_config(config_file, &kind)?;
    let investigator = Investigator::new(param);
    let inspector: Box<dyn Inspector> = match kind {
        Kind::SDTM => Box::new(SdtmInspector::new(investigator, &config, qc_ignore)),
        Kind::ADaM => Box::new(AdamInspector::new(investigator, &config, qc_ignore)),
        Kind::TFLs => Box::new(TflInspector::new(investigator, &config, qc_ignore)),
    };
    inspector.inspect()
}

pub fn log_detail<P: AsRef<Path>>(
    param: &InvestigatorParam<P>,
    item: &str,
    kind: &Kind,
    group: &Group,
) -> Result<LogResult> {
    let investigator = Investigator::new(param);
    let validator = SasLogValidatior::new();
    let file = match kind {
        Kind::SDTM => investigator.sdtm_log(item, group),
        Kind::ADaM => investigator.adam_log(item, group),
        Kind::TFLs => investigator.tfl_log(item, group),
    };
    match file {
        Some(file) => {
            let result = validator
                .validate(&file.filepath)
                .map_err(|_| Error::LogFailed(item.to_string()))?;
            Ok(result.into())
        }
        None => Ok(LogResult {
            status: Status::Missing,
            details: vec![],
        }),
    }
}

pub fn qc_detail<P: AsRef<Path>>(
    param: &InvestigatorParam<P>,
    item: &str,
    kind: &Kind,
    ignore: &[String],
) -> Result<Vec<QcResult>> {
    let investigator = Investigator::new(param);
    let mut items = Vec::with_capacity(2);
    match kind {
        Kind::SDTM => {
            items.push(investigator.sdtm_qc_main(item));
            items.push(investigator.sdtm_qc_supp(item));
        }
        Kind::ADaM => items.push(investigator.adam_qc_result(item)),
        Kind::TFLs => items.push(investigator.tfl_qc_result(item)),
    }
    let mut results = Vec::with_capacity(2);
    for (index, item) in items.into_iter().enumerate() {
        let item_type = match index {
            0 => "main".into(),
            _ => "supp".into(),
        };
        match item {
            Some(file) => {
                let mut validator =
                    QcResultValidator::new(file.filepath, ignore).map_err(|_| Error::QcFailed)?;
                let result = validator.validate().map_err(|_| Error::QcFailed)?;
                match result {
                    validator::result::ReportResult::Pass => results.push(QcResult {
                        item_type,
                        status: Status::Pass,
                    }),
                    validator::result::ReportResult::Unknown => results.push(QcResult {
                        item_type,
                        status: Status::Failed("Unknown Reason".into()),
                    }),
                    validator::result::ReportResult::Fail(reason) => results.push(QcResult {
                        item_type,
                        status: Status::Failed(reason),
                    }),
                }
            }
            None => results.push(QcResult {
                item_type,
                status: Status::Missing,
            }),
        }
    }
    Ok(results)
}

pub fn sequence_detail<P: AsRef<Path>>(
    param: &InvestigatorParam<P>,
    item: &str,
    supp: bool,
    kind: &Kind,
) -> Vec<AuditResult> {
    let investigator = Investigator::new(param);
    let auditor = audit::new(item, kind, supp, &investigator);
    auditor.audit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_inspect_sdtm() -> anyhow::Result<()> {
        let kind = Kind::SDTM;
        let config = Path::new(
            r"D:\Studies\ak112\303\documents\specs\AK112-303 SDTM Specification v0.4.xlsx",
        );
        let root = Path::new(r"D:\Studies");
        let param = InvestigatorParam {
            product: "ak112".into(),
            trial: "303".into(),
            purpose: "CSR".into(),
            root,
        };
        let qc_ignore = vec![];
        let result = inspect(&param, &config, &kind, &qc_ignore);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_inspect_adam() -> anyhow::Result<()> {
        let kind = Kind::ADaM;
        let config = Path::new(
            r"D:\Studies\ak112\303\documents\specs\AK112-303 ADaM Specification v0.2.xlsx",
        );
        let root = Path::new(r"D:\Studies");
        let param = InvestigatorParam {
            product: "ak112".into(),
            trial: "303".into(),
            purpose: "CSR".into(),
            root,
        };
        let qc_ignore = vec![];
        let result = inspect(&param, &config, &kind, &qc_ignore);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_inspect_tfl() -> anyhow::Result<()> {
        let kind = Kind::TFLs;
        let config = Path::new(r"D:\Studies\ak112\303\stats\CSR\utility\top-ak112-303-CSR.xlsx");
        let root = Path::new(r"D:\Studies");
        let param = InvestigatorParam {
            product: "ak112".into(),
            trial: "303".into(),
            purpose: "CSR".into(),
            root,
        };
        let qc_ignore = vec![];
        let result = inspect(&param, &config, &kind, &qc_ignore);
        assert!(result.is_ok());
        Ok(())
    }
}
