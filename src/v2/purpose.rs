use super::error::Result;
use serde::Serialize;
use std::{fs, path::Path};

#[derive(Debug, Serialize)]
pub struct Purpose {
    id: String,
    name: String,
}

pub(crate) fn list_purposes<P: AsRef<Path>>(parent: &str, root: P) -> Result<Vec<Purpose>> {
    let mut purposes = vec![];
    if !root.as_ref().exists() {
        return Ok(purposes);
    }
    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        let dir = entry.path();
        if dir.is_file() {
            continue;
        }
        purposes.push(fetch_purpose(parent, &dir)?);
    }
    Ok(purposes)
}

fn fetch_purpose<P: AsRef<Path>>(parent: &str, root: P) -> Result<Purpose> {
    let purpose = root
        .as_ref()
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    let id = format!("{}-{}", parent, purpose);
    Ok(Purpose {
        id,
        name: purpose.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_purposes() -> anyhow::Result<()> {
        let root = Path::new(r"D:\Studies\ak101\202\stats");
        let parent = "ak101-202";
        let purposes = list_purposes(parent, root)?;
        assert_eq!(purposes.len(), 4);
        Ok(())
    }
}
