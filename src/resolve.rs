use crate::paths;
use chrono::{DateTime, Local};
use std::path::{Path, PathBuf};

pub struct ResolvedEntry {
    pub dir: PathBuf,
    pub file_path: PathBuf,
    pub template_path: PathBuf,
}

pub fn resolve(
    variant_key: &str,
    home: &Path,
    now: DateTime<Local>,
) -> ResolvedEntry {
    let dir = paths::variant_dir(home, variant_key);
    let stamp = now.format("%m-%d-%Y_%H-%M").to_string();
    let file_path = dir.join(format!("{stamp}.md"));
    let template_path = paths::template_path(home, variant_key);
    ResolvedEntry {
        dir,
        file_path,
        template_path,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn resolve_uses_mm_dd_yyyy() {
        let home = PathBuf::from("/tmp/jot");
        let now = Local.with_ymd_and_hms(2026, 8, 24, 14, 32, 0).unwrap();
        let entry = resolve("note", &home, now);
        assert_eq!(entry.dir, PathBuf::from("/tmp/jot/note"));
        assert_eq!(
            entry.file_path,
            PathBuf::from("/tmp/jot/note/08-24-2026_14-32.md")
        );
        assert_eq!(
            entry.template_path,
            PathBuf::from("/tmp/jot/templates/note.template")
        );
    }
}
