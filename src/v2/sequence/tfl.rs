use super::audit::{auditing, SequenceAuditor, SequenceResult};
use crate::v2::{
    category::{FileType, Group},
    investigator::{
        investigator::{File, Investigator},
        utils::filename,
    },
    sequence::audit::{total_status, AuditResult},
};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct TflSequenceAuditor {
    item: String,
    production: TflProductionFile<PathBuf>,
    validation: TflValidationFile<PathBuf>,
}

impl TflSequenceAuditor {
    pub fn new(item: &str, investigator: &Investigator) -> TflSequenceAuditor {
        let production = TflProductionFile::<PathBuf>::build(item, investigator);
        let validation = TflValidationFile::<PathBuf>::build(item, investigator);
        TflSequenceAuditor {
            item: item.into(),
            production,
            validation,
        }
    }

    fn audit_production(&self) -> AuditResult {
        let mut sequences = Vec::with_capacity(4);
        sequences.push(self.audit_code(Group::Production));
        sequences.push(self.audit_dataset(Group::Production));
        sequences.push(self.audit_output());
        sequences.push(self.audit_log(Group::Production));
        let status = total_status(&sequences);
        AuditResult { sequences, status }
    }

    fn audit_validation(&self) -> AuditResult {
        let mut sequences = Vec::with_capacity(4);
        sequences.push(self.audit_code(Group::Validation));
        sequences.push(self.audit_dataset(Group::Validation));
        sequences.push(self.audit_qc());
        sequences.push(self.audit_log(Group::Validation));
        let status = total_status(&sequences);
        AuditResult { sequences, status }
    }

    fn audit_code(&self, group: Group) -> SequenceResult {
        let kind = FileType::Code;
        let code = match group {
            Group::Production => self.production.code.as_ref(),
            Group::Validation => self.validation.code.as_ref(),
        };
        SequenceResult {
            name: filename(&self.item, &group, &kind),
            kind,
            status: auditing(code, None, ""),
            group,
            modified_at: code.map(|f| f.modified_at),
        }
    }

    fn audit_dataset(&self, group: Group) -> SequenceResult {
        let kind = FileType::Data;
        let base = match group {
            Group::Production => self.production.dataset.as_ref(),
            Group::Validation => self.validation.dataset.as_ref(),
        };
        let compare = match group {
            Group::Production => self.production.code.as_ref(),
            Group::Validation => self.validation.code.as_ref(),
        };

        SequenceResult {
            name: filename(&self.item, &group, &kind),
            kind,
            status: auditing(base, compare, "Code later than data"),
            group,
            modified_at: base.as_ref().map(|f| f.modified_at),
        }
    }

    fn audit_output(&self) -> SequenceResult {
        let kind = FileType::Output;
        let base = self.production.output.as_ref();
        let compare = self.production.code.as_ref();

        SequenceResult {
            name: filename(&self.item, &Group::Production, &kind),
            kind,
            status: auditing(base, compare, "Code later than output"),
            group: Group::Production,
            modified_at: base.as_ref().map(|f| f.modified_at),
        }
    }

    fn audit_qc(&self) -> SequenceResult {
        let kind = FileType::Qc;
        let base = self.validation.qc.as_ref();
        let prod_dataset = self.production.dataset.as_ref();
        let val_dataset = self.validation.dataset.as_ref();

        let mut status = auditing(base, val_dataset, "Qc later than val dataset");

        if !status.is_pass() {
            status = auditing(base, prod_dataset, "Qc later than prod dataset");
        }

        SequenceResult {
            name: filename(&self.item, &Group::Validation, &kind),
            kind,
            status,
            group: Group::Validation,
            modified_at: base.as_ref().map(|f| f.modified_at),
        }
    }

    fn audit_log(&self, group: Group) -> SequenceResult {
        let kind = FileType::Log;
        let base = match group {
            Group::Production => self.production.log.as_ref(),
            Group::Validation => self.validation.log.as_ref(),
        };
        let compare = match group {
            Group::Production => self.production.output.as_ref(),
            Group::Validation => self.validation.qc.as_ref(),
        };
        let message = match group {
            Group::Production => "Output later than log",
            Group::Validation => "Qc later than log",
        };
        SequenceResult {
            name: filename(&self.item, &group, &kind),
            kind,
            status: auditing(base, compare, message),
            group,
            modified_at: base.as_ref().map(|f| f.modified_at),
        }
    }
}

impl SequenceAuditor for TflSequenceAuditor {
    fn audit(&self) -> Vec<AuditResult> {
        let production = self.audit_production();
        let validation = self.audit_validation();
        vec![production, validation]
    }
}

#[derive(Debug)]
pub struct TflProductionFile<P: AsRef<Path>> {
    pub code: Option<File<P>>,
    pub dataset: Option<File<P>>,
    pub output: Option<File<P>>,
    pub log: Option<File<P>>,
}

impl TflProductionFile<PathBuf> {
    pub fn build(item: &str, investigator: &Investigator) -> TflProductionFile<PathBuf> {
        TflProductionFile {
            code: investigator.tfl_code(item, &Group::Production),
            dataset: investigator.tfl_data(item, &Group::Production),
            output: investigator.tfl_output(item, &Group::Production),
            log: investigator.tfl_log(item, &Group::Production),
        }
    }
}

#[derive(Debug)]
pub struct TflValidationFile<P: AsRef<Path>> {
    pub code: Option<File<P>>,
    pub dataset: Option<File<P>>,
    pub log: Option<File<P>>,
    pub qc: Option<File<P>>,
}

impl TflValidationFile<PathBuf> {
    pub fn build(item: &str, investigator: &Investigator) -> TflValidationFile<PathBuf> {
        TflValidationFile {
            code: investigator.tfl_code(item, &Group::Validation),
            dataset: investigator.tfl_data(item, &Group::Validation),
            log: investigator.tfl_log(item, &Group::Validation),
            qc: investigator.tfl_qc_result(item),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::{inspect::result::Status, InvestigatorParam};
    #[test]
    fn test_tfls_sequeuce() -> anyhow::Result<()> {
        let invest = Investigator::new(&InvestigatorParam {
            product: "ak112".into(),
            trial: "303".into(),
            purpose: "CSR".into(),
            root: Path::new(r"D:\Studies"),
        });
        let sequencer = TflSequenceAuditor::new("f-14-02-02-03-os-for-fas", &invest);
        let result = sequencer.audit();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].sequences.len(), 4);
        assert_eq!(result[1].sequences.len(), 4);
        assert_eq!(result[0].status, Status::Failed("".into()));
        assert_eq!(result[1].status, Status::Failed("".into()));
        Ok(())
    }
}
