use super::{
    investigator::{File, Investigator},
    utils::filename,
};
use crate::v2::{
    category::{FileType, Group},
    investigator::utils::file,
};
use std::path::PathBuf;

const CODE_DIR: &str = r"program\sdtm";
const DATA_DIR: &str = r"dataset\sdtm";
const QC_DIR: &str = r"qc-result\sdtm";

impl Investigator {
    pub fn sdtm_code_production(&self, item: &str) -> Option<File<PathBuf>> {
        let group = Group::Production;
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(CODE_DIR)
            .join(filename(item, &group, &FileType::Code));
        file(filepath)
    }

    pub fn sdtm_code_validation(&self, item: &str) -> Option<File<PathBuf>> {
        let group = Group::Validation;
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(CODE_DIR)
            .join(filename(item, &group, &FileType::Code));
        file(filepath)
    }

    pub fn sdtm_data_main_production(&self, item: &str) -> Option<File<PathBuf>> {
        let group = Group::Production;
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(DATA_DIR)
            .join(filename(item, &group, &FileType::Data));
        file(filepath)
    }

    pub fn sdtm_data_main_validation(&self, item: &str) -> Option<File<PathBuf>> {
        let group = Group::Validation;
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(DATA_DIR)
            .join(filename(item, &group, &FileType::Data));
        file(filepath)
    }

    pub fn sdtm_data_supp_production(&self, item: &str) -> Option<File<PathBuf>> {
        let group = Group::Production;
        let item = format!("supp{}", item);
        let filepath = self
            .root()
            .join(Group::Production.group_dir())
            .join(DATA_DIR)
            .join(filename(&item, &group, &FileType::Data));
        file(filepath)
    }

    pub fn sdtm_data_supp_validation(&self, item: &str) -> Option<File<PathBuf>> {
        let group = Group::Validation;
        let item = format!("supp{}", item);
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(DATA_DIR)
            .join(filename(&item, &group, &FileType::Data));
        file(filepath)
    }

    pub fn sdtm_xpt_main(&self, item: &str) -> Option<File<PathBuf>> {
        let group = Group::Production;
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(DATA_DIR)
            .join(filename(&item, &group, &FileType::Xpt));
        file(filepath)
    }

    pub fn sdtm_xpt_supp(&self, item: &str) -> Option<File<PathBuf>> {
        let group = Group::Production;
        let item = format!("supp{}", item);
        let filepath = self
            .root()
            .join(Group::Production.group_dir())
            .join(DATA_DIR)
            .join(filename(&item, &group, &FileType::Xpt));
        file(filepath)
    }

    pub fn sdtm_log(&self, item: &str, group: &Group) -> Option<File<PathBuf>> {
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(CODE_DIR)
            .join(filename(&item, &group, &FileType::Log));
        file(filepath)
    }

    pub fn sdtm_qc_main(&self, item: &str) -> Option<File<PathBuf>> {
        let group = Group::Validation;
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(QC_DIR)
            .join(filename(&item, &group, &FileType::Qc));
        file(filepath)
    }

    pub fn sdtm_qc_supp(&self, item: &str) -> Option<File<PathBuf>> {
        let group = Group::Validation;
        let item = format!("supp{}", item);
        let filepath = self
            .root()
            .join(group.group_dir())
            .join(QC_DIR)
            .join(filename(&item, &group, &FileType::Qc));
        file(filepath)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::v2::InvestigatorParam;

    use super::*;
    #[test]
    fn test_investigator_sdtm() -> anyhow::Result<()> {
        let inv = Investigator::new(&InvestigatorParam {
            product: "ak112".into(),
            trial: "303".into(),
            purpose: "CSR".into(),
            root: Path::new(r"D:\Studies"),
        });
        assert_eq!(
            inv.sdtm_code_production("ae").unwrap().filepath,
            Path::new(r"D:\Studies\ak112\303\stats\CSR\product\program\sdtm\ae.sas")
        );
        Ok(())
    }
}
