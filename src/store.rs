use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::model::PersonalRecord;

pub fn save_record(path: impl AsRef<Path>, record: &PersonalRecord) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create data directory {}", parent.display()))?;
    }
    let serialized = serde_json::to_string_pretty(record).context("failed to serialize record")?;
    fs::write(path, serialized).with_context(|| format!("failed to write {}", path.display()))
}

pub fn load_record(path: impl AsRef<Path>) -> Result<PersonalRecord> {
    let path = path.as_ref();
    let raw =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("failed to parse {}", path.display()))
}
