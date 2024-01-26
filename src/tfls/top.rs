use anyhow::{Error, Result};
use calamine::{open_workbook, DataType::Empty, Reader, Xlsx};
use std::path::Path;

const TOP: &str = "top";
const OUTPUT_NAME_COL_INDEX: usize = 4;
const TARGET_ROWS_START_INDEX: usize = 1;
const VALIDATION_LEVEL_INDEX: usize = 0;

/// Read topline and get information for each output
pub fn read_top(dir: &Path) -> Result<Vec<(String, bool)>, Error> {
    let mut outputs: Vec<(String, bool)> = vec![];
    let mut workbook: Xlsx<_> = open_workbook(dir)?;

    let range = workbook.worksheet_range(TOP)?;
    for (n, row) in range.rows().into_iter().enumerate() {
        // skipping untarget rows
        if n < TARGET_ROWS_START_INDEX {
            continue;
        }
        let output;
        let mut qc: bool = false;
        if let Some(e) = row.get(OUTPUT_NAME_COL_INDEX) {
            if e.eq(&Empty) {
                break;
            }
            output = e.as_string().unwrap();
        } else {
            break;
        }
        if let Some(e) = row.get(VALIDATION_LEVEL_INDEX) {
            if e.eq(&Empty) {
                break;
            }
            if e.as_string().unwrap().eq("3") {
                qc = true;
            }
        }
        outputs.push((output, qc));
    }
    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_top_test() {
        let p = Path::new(r"D:\projects\rusty\mobius_kit\.mocks\specs\top-ak112-303-CSR.xlsx");
        let result = read_top(p).unwrap();
        assert_eq!(144, result.len());
    }
}
