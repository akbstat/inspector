use crate::v2::{
    category::{FileType, Group},
    investigator::{
        investigator::{File, Investigator},
        utils::filename,
    },
    sequence::audit::{auditing, total_status, AuditResult, SequenceAuditor, SequenceResult},
};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct AdamSequenceAuditor {
    item: String,
    production: AdamProductionFile<PathBuf>,
    validation: AdamValidationFile<PathBuf>,
}

impl AdamSequenceAuditor {
    pub fn new(item: &str, investigator: &Investigator) -> AdamSequenceAuditor {
        let production = AdamProductionFile::<PathBuf>::build(item, investigator);
        let validation = AdamValidationFile::<PathBuf>::build(item, investigator);
        AdamSequenceAuditor {
            item: item.into(),
            production,
            validation,
        }
    }

    fn audit_production(&self) -> AuditResult {
        let mut sequences = Vec::with_capacity(4);
        sequences.push(self.audit_code(Group::Production));
        sequences.push(self.audit_dataset(Group::Production));
        sequences.push(self.audit_xpt());
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

    fn audit_dataset(&self, group: Group) -> SequenceResult {
        let dataset = match group {
            Group::Production => self.production.dataset.as_ref(),
            Group::Validation => self.validation.dataset.as_ref(),
        };
        let code = match group {
            Group::Production => self.production.code.as_ref(),
            Group::Validation => self.validation.code.as_ref(),
        };
        let kind = FileType::Data;
        let name = filename(&self.item, &group, &kind);
        SequenceResult {
            name,
            kind,
            status: auditing(dataset, code, "Code later than data"),
            group,
            modified_at: dataset.map(|f| f.modified_at),
        }
    }

    fn audit_xpt(&self) -> SequenceResult {
        let xpt = self.production.xpt.as_ref();
        let dataset = self.production.dataset.as_ref();
        let kind = FileType::Xpt;
        let name = filename(&self.item, &Group::Production, &kind);
        SequenceResult {
            name,
            kind,
            status: auditing(xpt, dataset, "Xpt later than data"),
            // Here we assume the main output always belongs to Production;
            // adjust the group if needed.
            group: Group::Production,
            modified_at: xpt.map(|f| f.modified_at),
        }
    }

    fn audit_qc(&self) -> SequenceResult {
        let base = self.validation.qc.as_ref();
        let prod_dataset = self.production.dataset.as_ref();
        let val_dataset = self.validation.dataset.as_ref();

        let mut status = auditing(base, val_dataset, "Qc later than val dataset");

        if !status.is_pass() {
            status = auditing(base, prod_dataset, "Qc later than prod dataset");
        }
        let kind = FileType::Qc;
        let name = filename(&self.item, &Group::Validation, &kind);
        SequenceResult {
            name,
            kind,
            status,
            group: Group::Validation,
            modified_at: base.as_ref().map(|f| f.modified_at),
        }
    }

    fn audit_log(&self, group: Group) -> SequenceResult {
        let log = match group {
            Group::Production => self.production.log.as_ref(),
            Group::Validation => self.validation.log.as_ref(),
        };
        let product = match group {
            Group::Production => self.production.xpt.as_ref(),
            Group::Validation => self.validation.qc.as_ref(),
        };
        let message = match group {
            Group::Production => "Xpt later than log",
            Group::Validation => "Qc later than log",
        };
        let kind = FileType::Log;
        let name = filename(&self.item, &group, &kind);
        SequenceResult {
            name,
            kind,
            status: auditing(log, product, message),
            group,
            modified_at: log.as_ref().map(|f| f.modified_at),
        }
    }
}

impl SequenceAuditor for AdamSequenceAuditor {
    fn audit(&self) -> Vec<AuditResult> {
        vec![self.audit_production(), self.audit_validation()]
    }
}

#[derive(Debug)]
pub struct AdamProductionFile<P: AsRef<Path>> {
    pub code: Option<File<P>>,
    pub dataset: Option<File<P>>,
    pub xpt: Option<File<P>>,
    pub log: Option<File<P>>,
}

impl AdamProductionFile<PathBuf> {
    pub fn build(item: &str, investigator: &Investigator) -> AdamProductionFile<PathBuf> {
        AdamProductionFile {
            code: investigator.adam_code(item, &Group::Production),
            dataset: investigator.adam_data(item, &Group::Production),
            xpt: investigator.adam_xpt(item),
            log: investigator.adam_log(item, &Group::Production),
        }
    }
}

#[derive(Debug)]
pub struct AdamValidationFile<P: AsRef<Path>> {
    pub code: Option<File<P>>,
    pub dataset: Option<File<P>>,
    pub qc: Option<File<P>>,
    pub log: Option<File<P>>,
}

impl AdamValidationFile<PathBuf> {
    pub fn build(item: &str, investigator: &Investigator) -> AdamValidationFile<PathBuf> {
        AdamValidationFile {
            code: investigator.adam_code(item, &Group::Validation),
            dataset: investigator.adam_data(item, &Group::Validation),
            qc: investigator.adam_qc_result(item),
            log: investigator.adam_log(item, &Group::Validation),
        }
    }
}
