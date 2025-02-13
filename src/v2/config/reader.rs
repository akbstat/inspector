use super::{adam::AdamConfigReader, sdtm::SdtmConfigReader, tfl::TflConfigReader};
use crate::v2::{category::Kind, error::Result};
use std::path::Path;

pub trait ConfigReader {
    fn read(&self, file: &Path) -> Result<Vec<Config>>;
}

#[derive(Debug, Clone)]
pub struct Config {
    pub name: String,
    pub supp: bool,
    pub qc: bool,
    pub order: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: String::new(),
            supp: false,
            qc: true,
            order: 0,
        }
    }
}

pub fn read_config<P: AsRef<Path>>(file: P, kind: &Kind) -> Result<Vec<Config>> {
    let reader: Box<dyn ConfigReader> = match kind {
        Kind::SDTM => Box::new(SdtmConfigReader::new()),
        Kind::ADaM => Box::new(AdamConfigReader::new()),
        Kind::TFLs => Box::new(TflConfigReader::new()),
    };
    reader.read(file.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_config() -> anyhow::Result<()> {
        let kind = Kind::SDTM;
        let file = Path::new(
            r"D:\Studies\ak112\303\documents\specs\AK112-303 SDTM Specification v0.4.xlsx",
        );
        let config = read_config(file, &kind)?;
        assert_eq!(config.len(), 37);

        let kind = Kind::ADaM;
        let file = Path::new(
            r"D:\Studies\ak112\303\documents\specs\AK112-303 ADaM Specification v0.2.xlsx",
        );
        let config = read_config(file, &kind)?;
        assert_eq!(config.len(), 17);

        let kind = Kind::TFLs;
        let file = Path::new(r"D:\Studies\ak112\303\stats\CSR\utility\top-ak112-303-CSR.xlsx");
        let config = read_config(file, &kind)?;
        assert_eq!(config.len(), 152);
        Ok(())
    }
}
