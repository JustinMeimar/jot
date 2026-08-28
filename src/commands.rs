use crate::config::{self, Config};
use crate::error::{JotError, Result};
use crate::frontmatter;
use crate::io as jot_io;
use crate::paths;
use crate::resolve;
use chrono::Local;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub fn init(home: &Path) -> Result<()> {
    std::fs::create_dir_all(home)?;
    std::fs::create_dir_all(home.join("templates"))?;
    let cfg = paths::config_path(home);
    if !cfg.exists() {
        std::fs::write(&cfg, "")?;
    }
    println!("initialized {}", home.display());
    Ok(())
}

pub fn list(config: &Config) -> Result<()> {
    if config.jot.is_empty() {
        println!("no variants configured");
        return Ok(());
    }
    for (key, variant) in &config.jot {
        let desc = variant.description.as_deref().unwrap_or("");
        println!(
            "  {:<12} [alias: {}]  {}",
            key, variant.subcommand, desc
        );
    }
    Ok(())
}

pub fn rename(home: &Path, from: &str, to: &str) -> Result<()> {
    if crate::cli::RESERVED_NAMES.contains(&to) {
        return Err(JotError::ReservedName(to.to_string()));
    }
    let mut config = config::load(home)?;
    let variant = config
        .jot
        .remove(from)
        .ok_or_else(|| JotError::UnknownVariant(from.to_string()))?;
    if config.jot.contains_key(to) {
        return Err(JotError::AlreadyExists(to.to_string()));
    }
    let to_dir = paths::variant_dir(home, to);
    if to_dir.exists() {
        return Err(JotError::PathTaken(to_dir));
    }
    let to_tmpl = paths::template_path(home, to);
    if to_tmpl.exists() {
        return Err(JotError::PathTaken(to_tmpl));
    }
    let from_dir = paths::variant_dir(home, from);
    if from_dir.exists() {
        std::fs::rename(&from_dir, &to_dir)?;
    }
    let from_tmpl = paths::template_path(home, from);
    if from_tmpl.exists() {
        std::fs::rename(&from_tmpl, &to_tmpl)?;
    }
    config.jot.insert(to.to_string(), variant);
    config::save(home, &config)?;
    println!("renamed {from} to {to}");
    Ok(())
}

pub fn remove(home: &Path, key: &str) -> Result<()> {
    let mut config = config::load(home)?;
    if config.jot.remove(key).is_none() {
        return Err(JotError::UnknownVariant(key.to_string()));
    }
    let tmpl = paths::template_path(home, key);
    if tmpl.exists() {
        std::fs::remove_file(&tmpl)?;
    }
    config::save(home, &config)?;
    let dir = paths::variant_dir(home, key);
    if dir.exists() {
        println!("removed variant {key}");
        println!("note: entries kept at {}", dir.display());
        println!("      remove manually with: rm -rf {}", dir.display());
    } else {
        println!("removed variant {key}");
    }
    Ok(())
}

pub fn create_entry(
    home: &Path,
    variant_key: &str,
    tags: Vec<String>,
) -> Result<()> {
    let entry = resolve::resolve(variant_key, home, Local::now());
    jot_io::ensure_dirs(&entry)?;
    jot_io::seed_from_template(&entry)?;
    if !tags.is_empty() {
        apply_tags(&entry.file_path, tags)?;
    }
    jot_io::open_editor(&entry.file_path)?;
    println!("{}", entry.file_path.display());
    Ok(())
}

fn apply_tags(path: &Path, new_tags: Vec<String>) -> Result<()> {
    let text = std::fs::read_to_string(path)?;
    let (mut front, body) = frontmatter::split(&text);
    for t in new_tags {
        if !front.tags.contains(&t) {
            front.tags.push(t);
        }
    }
    let combined = frontmatter::join(&front, body);
    std::fs::write(path, combined)?;
    Ok(())
}

struct Hit {
    path: PathBuf,
    mtime: SystemTime,
}

pub fn search(home: &Path, key: &str) -> Result<()> {
    let needle = key.to_lowercase();
    let mut tag_hits: Vec<Hit> = Vec::new();
    let mut body_hits: Vec<Hit> = Vec::new();

    if !home.exists() {
        println!("no matches");
        return Ok(());
    }
    for entry in std::fs::read_dir(home)? {
        let entry = entry?;
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }
        if dir.file_name().and_then(|n| n.to_str()) == Some("templates") {
            continue;
        }
        scan_dir(&dir, &needle, &mut tag_hits, &mut body_hits)?;
    }

    tag_hits.sort_by(|a, b| b.mtime.cmp(&a.mtime));
    body_hits.sort_by(|a, b| b.mtime.cmp(&a.mtime));

    if tag_hits.is_empty() && body_hits.is_empty() {
        println!("no matches");
        return Ok(());
    }
    for h in &tag_hits {
        println!("[tag] {}", h.path.display());
    }
    for h in &body_hits {
        println!("      {}", h.path.display());
    }
    Ok(())
}

fn scan_dir(
    dir: &Path,
    needle: &str,
    tag_hits: &mut Vec<Hit>,
    body_hits: &mut Vec<Hit>,
) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let text = std::fs::read_to_string(&path)?;
        let (front, body) = frontmatter::split(&text);
        let mtime = entry
            .metadata()?
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH);

        let tag_match =
            front.tags.iter().any(|t| t.to_lowercase().contains(needle));
        if tag_match {
            tag_hits.push(Hit { path, mtime });
            continue;
        }
        let name_match = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.to_lowercase().contains(needle))
            .unwrap_or(false);
        if name_match || body.to_lowercase().contains(needle) {
            body_hits.push(Hit { path, mtime });
        }
    }
    Ok(())
}
