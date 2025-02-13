use super::{
    error::Result,
    purpose::{list_purposes, Purpose},
};
use serde::Serialize;
use std::{fs, path::Path};

#[derive(Debug, Serialize)]
pub struct Trial {
    id: String,
    name: String,
    purposes: Vec<Purpose>,
}

pub(crate) fn list_trials<P: AsRef<Path>>(parent: &str, root: P) -> Result<Vec<Trial>> {
    let mut trials = vec![];
    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        let dir = entry.path();
        if dir.is_file() {
            continue;
        }
        trials.push(fetch_trial(parent, &dir)?);
    }
    Ok(trials)
}

fn fetch_trial<P: AsRef<Path>>(parent: &str, root: P) -> Result<Trial> {
    let trial = root
        .as_ref()
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    let id = format!("{}-{}", parent, trial);
    Ok(Trial {
        id: id.clone(),
        name: trial.into(),
        purposes: list_purposes(&id, root.as_ref().join("stats"))?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_trials() -> anyhow::Result<()> {
        let root = Path::new(r"D:\Studies\ak101");
        let parent = "ak101";
        let purposes = list_trials(parent, root)?;
        assert_eq!(purposes.len(), 2);
        Ok(())
    }
}
