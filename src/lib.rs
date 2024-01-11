mod adam;
mod api;
mod module;
mod sdtm;
mod tfls;
mod utils;

pub use api::adam::inspect_adam;
pub use api::sdtm::inspect_sdtm;
pub use api::tfls::inspect_tfls;
pub use utils::ProjectDirInfer;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};
    #[test]
    fn sdtm_test() {
        let spec = Path::new(
            r"D:\projects\rusty\mobius_kit\.mocks\specs\AK112-303 SDTM Specification v0.2.xlsx",
        );
        let root = Path::new(r"D:\网页下载文件\dingtalk\rtfs\202-113\inspector\CSR");
        let result = inspect_sdtm(spec, root).unwrap();
        let s = serde_json::to_string(&result).unwrap();
        fs::write(
            r"D:\网页下载文件\dingtalk\rtfs\202-113\inspector\CSR\sdtm.json",
            s,
        )
        .unwrap();
    }
    #[test]
    fn tfls_test() {
        let top = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\specs\top-ak112-303-CSR.xlsx");
        let root = Path::new(r"D:\网页下载文件\dingtalk\rtfs\202-113\inspector\CSR");
        let result = inspect_tfls(top, root).unwrap();
        let s = serde_json::to_string(&result).unwrap();
        fs::write(
            r"D:\网页下载文件\dingtalk\rtfs\202-113\inspector\CSR\tfls.json",
            s,
        )
        .unwrap();
    }
}
