use super::reader::{Config, ConfigReader};
use crate::v2::error::Result;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::path::Path;

const TOP: &str = "top";
const CONTENT_START_ROW: usize = 1;
const OUTPUT_NAME_COLUMN: usize = 4;
const VALIDATION_LEVEL_COLUMN: usize = 0;
const VALIDATION_FLAG_STRING: &str = "3";
const VALIDATION_FLAG_FLOAT: f64 = 3f64;

pub(crate) struct TflConfigReader;

impl TflConfigReader {
    pub fn new() -> TflConfigReader {
        TflConfigReader
    }
}

impl ConfigReader for TflConfigReader {
    fn read(&self, file: &Path) -> Result<Vec<Config>> {
        let mut configs = vec![];
        let mut workbook: Xlsx<_> = open_workbook(&file)?;
        let empty = DataType::String("".into());
        let range = workbook.worksheet_range(TOP)?;
        for (n, row) in range.rows().into_iter().enumerate() {
            // skipping untarget rows
            if n.lt(&CONTENT_START_ROW) {
                continue;
            }
            let mut config = Config::default();
            let domain = row
                .get(OUTPUT_NAME_COLUMN)
                .unwrap_or(&empty)
                .as_string()
                .unwrap_or_default();
            if domain.is_empty() {
                // why use continue instead of break? because in unoffical top file, contains rows which is totally empty, so we need to read all rows to get the complete output informations
                continue;
            }
            config.name = domain.clone();
            config.order = n;
            config.qc = validation(row.get(VALIDATION_LEVEL_COLUMN));
            configs.push(config);
        }
        Ok(configs)
    }
}

fn validation(cell: Option<&DataType>) -> bool {
    let cell = cell.unwrap_or(&DataType::Empty);
    match cell {
        DataType::String(cell) => {
            if cell.eq(VALIDATION_FLAG_STRING) {
                true
            } else {
                false
            }
        }
        DataType::Float(cell) => {
            if cell.eq(&VALIDATION_FLAG_FLOAT) {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}
