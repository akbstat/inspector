use super::reader::{Config, ConfigReader};
use crate::v2::error::Result;
use calamine::{open_workbook, Data, DataType, Reader, Xlsx};
use std::path::Path;

const CONTENT: &str = "CONTENT";
const CONTENT_START_ROW: usize = 6;
const DOMAIN_COLUMN: usize = 0;

pub(crate) struct AdamConfigReader;

impl AdamConfigReader {
    pub fn new() -> AdamConfigReader {
        AdamConfigReader
    }
}

impl ConfigReader for AdamConfigReader {
    fn read(&self, file: &Path) -> Result<Vec<Config>> {
        let mut configs = vec![];
        let mut workbook: Xlsx<_> = open_workbook(&file)?;
        let empty = Data::String("".into());
        let range = workbook.worksheet_range(CONTENT)?;
        for (n, row) in range.rows().into_iter().enumerate() {
            // skipping untarget rows
            if n.lt(&CONTENT_START_ROW) {
                continue;
            }
            let mut config = Config::default();
            let domain = row
                .get(DOMAIN_COLUMN)
                .unwrap_or(&empty)
                .as_string()
                .unwrap_or_default();
            if domain.is_empty() {
                break;
            }
            config.name = domain.clone();
            config.order = n;
            configs.push(config);
        }
        Ok(configs)
    }
}
