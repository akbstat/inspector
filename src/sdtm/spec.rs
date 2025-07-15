use anyhow::{Error, Result};
use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::path::Path;

const CONTENT: &str = "CONTENT";
const SUPP_PREFIX: &str = "SUPP";
const DOMAIN_COL_INDEX: usize = 0;
const TARGET_ROWS_START_INDEX: usize = 6;
const VAR_BELONG_COL_INDEX: usize = 9;

/// Read sdtm spec and get information for each domain
///
/// eg. `(ae, true)`, means ae domain, has supp domain
///
/// `(co, false)`, means ae domain, does not have supp domain
pub fn read_spec(dir: &Path) -> Result<Vec<(String, bool)>, Error> {
    let mut domains: Vec<(String, bool)> = vec![];
    let mut workbook: Xlsx<_> = open_workbook(dir)?;

    let range = workbook.worksheet_range(CONTENT)?;
    for (n, row) in range.rows().into_iter().enumerate() {
        // skipping untarget rows
        if n < TARGET_ROWS_START_INDEX {
            continue;
        }
        let domain;
        let mut supp = false;
        if let Some(e) = row.get(DOMAIN_COL_INDEX) {
            if e.is_empty() {
                break;
            }
            domain = e.as_string().unwrap();
        } else {
            break;
        }
        if skip_supp(&domain) {
            continue;
        }
        // read domain detail sheet to find out if supp existed
        if let Ok(range) = workbook.worksheet_range(&domain) {
            for row in range.rows().into_iter().rev() {
                if let Some(cell) = row.get(VAR_BELONG_COL_INDEX) {
                    if cell.is_empty() {
                        continue;
                    } else {
                        if cell.as_string().unwrap().eq(SUPP_PREFIX) {
                            supp = true;
                            break;
                        }
                    }
                }
            }
        }
        domains.push((domain.to_lowercase(), supp));
    }
    Ok(domains)
}

/// if read supp domain in content sheet, just skip,
/// because will determine existence of supp in details
/// of main domain
fn skip_supp(domain: &str) -> bool {
    domain.starts_with(SUPP_PREFIX)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_spec_test() {
        let p = Path::new(
            r"D:\projects\rusty\mobius_kit\.mocks\specs\AK112-303 SDTM Specification v0.2.xlsx",
        );
        let result = read_spec(p).unwrap();
        assert_eq!(37, result.len());
        // checkout if ae has supp
        assert!(result.get(5).unwrap().1);
        // checkout if co does not have supp
        assert!(!result.get(36).unwrap().1);
    }

    #[test]
    fn skip_supp_test() {
        assert!(skip_supp("SUPPAE"));
        assert!(!skip_supp("AE"));
    }
}
