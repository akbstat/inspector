use std::path::{Path, PathBuf};

use super::group::GroupKind;

const DEV_ROOT: &str = r"product";
const QC_ROOT: &str = r"validation";
const RAW: &str = r"rawdata";
const SDTM_PROGRAM_PATH: &str = r"program\sdtm";
const SDTM_DATASET_PATH: &str = r"dataset\sdtm";
const SDTM_QC_RESULT: &str = r"qc-result\sdtm";
const ADAM_PROGRAM_PATH: &str = r"program\adam";
const ADAM_DATASET_PATH: &str = r"dataset\adam";
const ADAM_QC_RESULT: &str = r"qc-result\adam";
const TFL_PROGRAM_PATH: &str = r"program\tfl";
const TFL_DATASET_PATH: &str = r"dataset\tfl";
const TFL_OUTPUT_PATH: &str = r"output";
const TFL_QC_RESULT: &str = r"qc-result\tfl";

#[derive(Debug, Default)]
pub struct Paths {
    raw: PathBuf,
    sdtm: SDTM,
    adam: ADaM,
    tfls: TFLs,
}

impl Paths {
    pub fn new(root: &Path) -> Paths {
        Paths {
            raw: PathBuf::from(root).join(RAW),
            sdtm: SDTM::new(root),
            adam: ADaM::new(root),
            tfls: TFLs::new(root),
        }
    }
    pub fn raw(&self) -> &Path {
        self.raw.as_path()
    }
    pub fn sdtm_code(&self, kind: GroupKind) -> &Path {
        match kind {
            GroupKind::Dev => self.sdtm.program.0.as_path(),
            GroupKind::Qc => self.sdtm.program.1.as_path(),
        }
    }
    pub fn sdtm_dataset(&self, kind: GroupKind) -> &Path {
        match kind {
            GroupKind::Dev => self.sdtm.dataset.0.as_path(),
            GroupKind::Qc => self.sdtm.dataset.1.as_path(),
        }
    }
    // pub fn sdtm_xpt(&self) -> &Path {
    //     self.sdtm.dataset.0.as_path()
    // }
    // pub fn sdtm_log(&self, kind: GroupKind) -> &Path {
    //     self.sdtm_code(kind)
    // }
    pub fn sdtm_qc(&self) -> &Path {
        self.sdtm.qc.as_path()
    }
    pub fn adam_code(&self, kind: GroupKind) -> &Path {
        match kind {
            GroupKind::Dev => self.adam.program.0.as_path(),
            GroupKind::Qc => self.adam.program.1.as_path(),
        }
    }
    pub fn adam_dataset(&self, kind: GroupKind) -> &Path {
        match kind {
            GroupKind::Dev => self.adam.dataset.0.as_path(),
            GroupKind::Qc => self.adam.dataset.1.as_path(),
        }
    }
    // pub fn adam_xpt(&self) -> &Path {
    //     self.adam.dataset.0.as_path()
    // }
    // pub fn adam_log(&self, kind: GroupKind) -> &Path {
    //     self.adam_code(kind)
    // }
    pub fn adam_qc(&self) -> &Path {
        self.adam.qc.as_path()
    }
    pub fn tfls_code(&self, kind: GroupKind) -> &Path {
        match kind {
            GroupKind::Dev => self.tfls.program.0.as_path(),
            GroupKind::Qc => self.tfls.program.1.as_path(),
        }
    }
    pub fn tfls_dataset(&self, kind: GroupKind) -> &Path {
        match kind {
            GroupKind::Dev => self.tfls.dataset.0.as_path(),
            GroupKind::Qc => self.tfls.dataset.1.as_path(),
        }
    }
    // pub fn tfls_log(&self, kind: GroupKind) -> &Path {
    //     self.tfls_code(kind)
    // }
    pub fn tfls_output(&self) -> &Path {
        self.tfls.output.as_path()
    }
    pub fn tfls_qc(&self) -> &Path {
        self.tfls.qc.as_path()
    }
}

#[derive(Debug, Default)]
struct SDTM {
    // 0 stands for dev, 1 stands for qc
    pub program: (PathBuf, PathBuf),
    pub dataset: (PathBuf, PathBuf),
    pub qc: PathBuf,
}

impl SDTM {
    pub fn new(root: &Path) -> SDTM {
        let root = PathBuf::from(root);
        let dev_root = root.join(DEV_ROOT);
        let qc_root = root.join(QC_ROOT);
        SDTM {
            program: (
                dev_root.join(SDTM_PROGRAM_PATH),
                qc_root.join(SDTM_PROGRAM_PATH),
            ),
            dataset: (
                dev_root.join(SDTM_DATASET_PATH),
                qc_root.join(SDTM_DATASET_PATH),
            ),
            qc: qc_root.join(SDTM_QC_RESULT),
        }
    }
}

#[derive(Debug, Default)]
struct ADaM {
    // 0 stands for dev, 1 stands for qc
    pub program: (PathBuf, PathBuf),
    pub dataset: (PathBuf, PathBuf),
    pub qc: PathBuf,
}

impl ADaM {
    pub fn new(root: &Path) -> ADaM {
        let root = PathBuf::from(root);
        let dev_root = root.join(DEV_ROOT);
        let qc_root = root.join(QC_ROOT);
        ADaM {
            program: (
                dev_root.join(ADAM_PROGRAM_PATH),
                qc_root.join(ADAM_PROGRAM_PATH),
            ),
            dataset: (
                dev_root.join(ADAM_DATASET_PATH),
                qc_root.join(ADAM_DATASET_PATH),
            ),
            qc: qc_root.join(ADAM_QC_RESULT),
        }
    }
}

#[derive(Debug, Default)]
struct TFLs {
    // 0 stands for dev, 1 stands for qc
    pub program: (PathBuf, PathBuf),
    pub dataset: (PathBuf, PathBuf),
    pub output: PathBuf,
    pub qc: PathBuf,
}

impl TFLs {
    pub fn new(root: &Path) -> TFLs {
        let root = PathBuf::from(root);
        let dev_root = root.join(DEV_ROOT);
        let qc_root = root.join(QC_ROOT);
        TFLs {
            program: (
                dev_root.join(TFL_PROGRAM_PATH),
                qc_root.join(TFL_PROGRAM_PATH),
            ),
            dataset: (
                dev_root.join(TFL_DATASET_PATH),
                qc_root.join(TFL_DATASET_PATH),
            ),
            output: dev_root.join(TFL_OUTPUT_PATH),
            qc: qc_root.join(TFL_QC_RESULT),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paths_test() {
        let root = r"\\180.0.0.1\Studies\ak112\303\stats\CSR";
        let p = Paths::new(Path::new(root));
        assert_eq!(
            Path::new(r"\\180.0.0.1\Studies\ak112\303\stats\CSR\rawdata"),
            p.raw()
        );
        assert_eq!(
            Path::new(r"\\180.0.0.1\Studies\ak112\303\stats\CSR\product\program\adam"),
            p.adam_code(GroupKind::Dev)
        );
        assert_eq!(
            Path::new(r"\\180.0.0.1\Studies\ak112\303\stats\CSR\product\dataset\sdtm"),
            p.sdtm_dataset(GroupKind::Dev)
        );
        // assert_eq!(
        //     Path::new(r"\\180.0.0.1\Studies\ak112\303\stats\CSR\product\dataset\sdtm"),
        //     p.sdtm_xpt()
        // );
        // assert_eq!(
        //     Path::new(r"\\180.0.0.1\Studies\ak112\303\stats\CSR\validation\program\adam"),
        //     p.adam_log(GroupKind::Qc)
        // );
        assert_eq!(
            Path::new(r"\\180.0.0.1\Studies\ak112\303\stats\CSR\product\output"),
            p.tfls_output()
        );
    }
}
