use super::reader::{Config, ConfigReader};
use crate::v2::error::Result;
use calamine::{open_workbook, Data, DataType, Range, Reader, Xlsx};
use std::path::Path;

const CONTENT: &str = "CONTENT";
const SUPP_PREFIX: &str = "SUPP";
const DOMAIN_COLUMN: usize = 0;
const CONTENT_START_ROW: usize = 6;
const DOMAIN_START_ROW: usize = 13;
// const ALLOCATION_COLUMN: usize = 9;
const ALLOCATION_HEADER: &str = "变量归属";

pub(crate) struct SdtmConfigReader;

impl SdtmConfigReader {
    pub fn new() -> SdtmConfigReader {
        SdtmConfigReader
    }
}

impl ConfigReader for SdtmConfigReader {
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
            // skip supp domain first, then will detect then in domain detail sheets
            if domain.starts_with(SUPP_PREFIX) {
                continue;
            }
            config.name = domain.clone();
            config.order = n;
            config.supp = detect_supp_domain(&workbook.worksheet_range(&domain.to_uppercase())?);
            configs.push(config);
        }
        Ok(configs)
    }
}

fn detect_supp_domain(worksheet: &Range<Data>) -> bool {
    let mut allocation_column = None;
    for (index, row) in worksheet.rows().enumerate() {
        if index.lt(&DOMAIN_START_ROW) {
            if (index + 1).eq(&DOMAIN_START_ROW) {
                for (col, cell) in row.iter().enumerate() {
                    let cell = cell.as_string().unwrap_or_default();
                    if cell.trim().eq(ALLOCATION_HEADER) && allocation_column.is_none() {
                        allocation_column = Some(col);
                        break;
                    }
                }
            }
            continue;
        }
        if let Some(allocation_column) = allocation_column {
            if let Some(cell) = row.get(allocation_column) {
                let cell = cell.as_string().unwrap_or_default();
                let cell = cell.trim();
                if cell.is_empty() {
                    break;
                }
                if cell.eq(SUPP_PREFIX) {
                    return true;
                }
            }
        }
    }
    false
}
