use crate::error::{JotError, Result};
use crate::resolve::ResolvedEntry;
use std::path::Path;
use std::process::Command;

pub fn ensure_dirs(entry: &ResolvedEntry) -> Result<()> {
    std::fs::create_dir_all(&entry.dir)?;
    Ok(())
}

pub fn seed_from_template(entry: &ResolvedEntry) -> Result<()> {
    if entry.file_path.exists() {
        return Ok(());
    }
    if entry.template_path.exists() {
        std::fs::copy(&entry.template_path, &entry.file_path)?;
    } else {
        std::fs::File::create(&entry.file_path)?;
    }
    Ok(())
}

pub fn open_editor(path: &Path) -> Result<()> {
    let editor = std::env::var("EDITOR").map_err(|_| JotError::NoEditor)?;
    let status = Command::new(editor).arg(path).status()?;
    if !status.success() {
        return Err(JotError::EditorFailed(status));
    }
    Ok(())
}
