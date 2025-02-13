use super::audit::{auditing, total_status, AuditResult, SequenceAuditor, SequenceResult};
use crate::v2::{
    category::{FileType, Group},
    investigator::{
        investigator::{File, Investigator},
        utils::filename,
    },
};

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SdtmSequenceAuditor {
    item: String,
    supp: bool,
    production: SdtmProductionFile<PathBuf>,
    validation: SdtmValidationFile<PathBuf>,
}

impl SdtmSequenceAuditor {
    pub fn new(item: &str, supp: bool, investigator: &Investigator) -> SdtmSequenceAuditor {
        let production = SdtmProductionFile::<PathBuf>::build(item, investigator);
        let validation = SdtmValidationFile::<PathBuf>::build(item, investigator);
        SdtmSequenceAuditor {
            item: item.into(),
            supp,
            production,
            validation,
        }
    }

    fn audit_production(&self) -> AuditResult {
        let mut sequences = Vec::with_capacity(6);
        sequences.push(self.audit_code(Group::Production));
        sequences.push(self.audit_main_dataset(Group::Production));
        sequences.push(self.audit_main_xpt());
        if self.supp {
            sequences.push(self.audit_supp_dataset(Group::Production));
            sequences.push(self.audit_supp_xpt());
        }
        sequences.push(self.audit_log(Group::Production));
        let status = total_status(&sequences);
        AuditResult { sequences, status }
    }

    fn audit_validation(&self) -> AuditResult {
        let mut sequences = Vec::with_capacity(6);
        sequences.push(self.audit_code(Group::Validation));
        sequences.push(self.audit_main_dataset(Group::Validation));
        sequences.push(self.audit_main_qc());
        if self.supp {
            sequences.push(self.audit_supp_dataset(Group::Validation));
            sequences.push(self.audit_supp_qc());
        }
        sequences.push(self.audit_log(Group::Validation));
        let status = total_status(&sequences);
        AuditResult { sequences, status }
    }

    fn audit_code(&self, group: Group) -> SequenceResult {
        let code = match group {
            Group::Production => self.production.code.as_ref(),
            Group::Validation => self.validation.code.as_ref(),
        };
        let kind = FileType::Code;
        let name = filename(&self.item, &group, &kind);
        SequenceResult {
            name,
            kind,
            status: auditing(code, None, ""),
            group,
            modified_at: code.map(|f| f.modified_at),
        }
    }

    fn audit_main_dataset(&self, group: Group) -> SequenceResult {
        let kind = FileType::Data;
        let name = filename(&self.item, &group, &kind);
        let base = match group {
            Group::Production => self.production.main_data.as_ref(),
            Group::Validation => self.validation.main_data.as_ref(),
        };
        let compare = match group {
            Group::Production => self.production.code.as_ref(),
            Group::Validation => self.validation.code.as_ref(),
        };
        SequenceResult {
            name,
            kind,
            status: auditing(base, compare, "Code later than data"),
            group,
            modified_at: base.as_ref().map(|f| f.modified_at),
        }
    }

    fn audit_supp_dataset(&self, group: Group) -> SequenceResult {
        let kind = FileType::Data;
        let name = filename(&format!("supp{}", &self.item), &group, &kind);
        let base = match group {
            Group::Production => self.production.supp_data.as_ref(),
            Group::Validation => self.validation.supp_data.as_ref(),
        };
        let compare = match group {
            Group::Production => self.production.main_data.as_ref(),
            Group::Validation => self.validation.main_data.as_ref(),
        };
        SequenceResult {
            name,
            kind,
            status: auditing(base, compare, "Main later than supp"),
            group,
            modified_at: self.production.supp_data.as_ref().map(|f| f.modified_at),
        }
    }

    fn audit_main_xpt(&self) -> SequenceResult {
        let kind = FileType::Xpt;
        let name = filename(&self.item, &Group::Production, &kind);
        SequenceResult {
            name,
            kind,
            status: auditing(
                self.production.main_xpt.as_ref(),
                self.production.main_data.as_ref(),
                "Data later than xpt",
            ),
            group: Group::Production,
            modified_at: self.production.main_xpt.as_ref().map(|f| f.modified_at),
        }
    }

    fn audit_supp_xpt(&self) -> SequenceResult {
        let kind = FileType::Xpt;
        let name = filename(&format!("supp{}", &self.item), &Group::Production, &kind);
        SequenceResult {
            name,
            kind,
            status: auditing(
                self.production.supp_xpt.as_ref(),
                self.production.supp_data.as_ref(),
                "Data later than xpt",
            ),
            group: Group::Production,
            modified_at: self.production.supp_xpt.as_ref().map(|f| f.modified_at),
        }
    }

    fn audit_main_qc(&self) -> SequenceResult {
        let kind = FileType::Qc;
        let name = filename(&self.item, &Group::Validation, &kind);
        SequenceResult {
            name,
            kind,
            status: auditing(
                self.validation.main_qc.as_ref(),
                self.validation.main_data.as_ref(),
                "Data later than QcResult",
            ),
            group: Group::Validation,
            modified_at: self.validation.main_qc.as_ref().map(|f| f.modified_at),
        }
    }

    fn audit_supp_qc(&self) -> SequenceResult {
        let kind = FileType::Qc;
        let name = filename(&format!("supp{}", &self.item), &Group::Validation, &kind);
        SequenceResult {
            name,
            kind,
            status: auditing(
                self.validation.supp_qc.as_ref(),
                self.validation.main_data.as_ref(),
                "Data later than QcResult",
            ),
            group: Group::Validation,
            modified_at: self.validation.supp_qc.as_ref().map(|f| f.modified_at),
        }
    }

    fn audit_log(&self, group: Group) -> SequenceResult {
        let kind = FileType::Log;
        let name = filename(&self.item, &group, &kind);
        let base = match group {
            Group::Production => self.production.log.as_ref(),
            Group::Validation => self.validation.log.as_ref(),
        };
        let compare = match group {
            Group::Production => match self.supp {
                true => self.production.supp_data.as_ref(),
                false => self.production.main_data.as_ref(),
            },
            Group::Validation => match self.supp {
                true => self.validation.supp_qc.as_ref(),
                false => self.validation.main_qc.as_ref(),
            },
        };
        let message = match group {
            Group::Production => "Data later than log",
            Group::Validation => "QcResult later than log",
        };
        SequenceResult {
            name,
            kind,
            status: auditing(base, compare, message),
            group,
            modified_at: base.map(|f| f.modified_at),
        }
    }
}

impl SequenceAuditor for SdtmSequenceAuditor {
    fn audit(&self) -> Vec<AuditResult> {
        let production = self.audit_production();
        let validation = self.audit_validation();
        vec![production, validation]
    }
}

#[derive(Debug)]
pub(crate) struct SdtmProductionFile<P: AsRef<Path>> {
    pub(crate) code: Option<File<P>>,
    pub(crate) log: Option<File<P>>,
    pub(crate) main_data: Option<File<P>>,
    pub(crate) supp_data: Option<File<P>>,
    pub(crate) main_xpt: Option<File<P>>,
    pub(crate) supp_xpt: Option<File<P>>,
}

impl<P: AsRef<Path>> SdtmProductionFile<P> {
    pub fn build(item: &str, investigator: &Investigator) -> SdtmProductionFile<PathBuf> {
        let code = investigator.sdtm_code_production(item);
        let log = investigator.sdtm_log(item, &Group::Production);
        let main_data = investigator.sdtm_data_main_production(item);
        let supp_data = investigator.sdtm_data_supp_production(item);
        let main_xpt = investigator.sdtm_xpt_main(item);
        let supp_xpt = investigator.sdtm_xpt_supp(item);
        SdtmProductionFile {
            code,
            log,
            main_data,
            supp_data,
            main_xpt,
            supp_xpt,
        }
    }
}

#[derive(Debug)]
pub(crate) struct SdtmValidationFile<P: AsRef<Path>> {
    pub(crate) code: Option<File<P>>,
    pub(crate) log: Option<File<P>>,
    pub(crate) main_data: Option<File<P>>,
    pub(crate) supp_data: Option<File<P>>,
    pub(crate) main_qc: Option<File<P>>,
    pub(crate) supp_qc: Option<File<P>>,
}

impl<P: AsRef<Path>> SdtmValidationFile<P> {
    pub fn build(item: &str, investigator: &Investigator) -> SdtmValidationFile<PathBuf> {
        let code = investigator.sdtm_code_validation(item);
        let log = investigator.sdtm_log(item, &Group::Validation);
        let main_data = investigator.sdtm_data_main_validation(item);
        let supp_data = investigator.sdtm_data_supp_validation(item);
        let main_qc = investigator.sdtm_qc_main(item);
        let supp_qc = investigator.sdtm_qc_supp(item);
        SdtmValidationFile {
            code,
            log,
            main_data,
            supp_data,
            main_qc,
            supp_qc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::{inspect::result::Status, InvestigatorParam};
    #[test]
    fn test_sdtm_sequeuce() -> anyhow::Result<()> {
        let invest = Investigator::new(&InvestigatorParam {
            product: "ak112".into(),
            trial: "303".into(),
            purpose: "CSR".into(),
            root: Path::new(r"D:\Studies"),
        });
        let sequencer = SdtmSequenceAuditor::new("ae", true, &invest);
        let result = sequencer.audit();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].sequences.len(), 6);
        assert_eq!(result[1].sequences.len(), 6);
        assert_eq!(result[0].status, Status::Failed("".into()));
        assert_eq!(result[1].status, Status::Failed("".into()));
        Ok(())
    }
}
