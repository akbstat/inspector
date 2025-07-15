use anyhow::{Error, Result};
use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::path::Path;

const CONTENT: &str = "CONTENT";
const DOMAIN_COL_INDEX: usize = 0;
const TARGET_ROWS_START_INDEX: usize = 6;

/// Read adam spec and get information for each domain
pub fn read_spec(dir: &Path) -> Result<Vec<String>, Error> {
    let mut domains: Vec<String> = vec![];
    let mut workbook: Xlsx<_> = open_workbook(dir)?;

    let range = workbook.worksheet_range(CONTENT)?;
    for (n, row) in range.rows().into_iter().enumerate() {
        // skipping untarget rows
        if n < TARGET_ROWS_START_INDEX {
            continue;
        }
        let domain;
        if let Some(e) = row.get(DOMAIN_COL_INDEX) {
            if e.is_empty() {
                break;
            }
            domain = e.as_string().unwrap();
        } else {
            break;
        }
        domains.push(domain.to_lowercase());
    }
    Ok(domains)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_spec_test() {
        let p = Path::new(
            r"D:\projects\rusty\mobius_kit\.mocks\specs\AK112-303 ADaM Specification v0.2.xlsx",
        );
        let result = read_spec(p).unwrap();
        assert_eq!(17, result.len());
    }
}
