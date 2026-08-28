use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const DELIM: &str = "+++";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Front {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, toml::Value>,
}

pub fn split(text: &str) -> (Front, &str) {
    let rest = match text.strip_prefix(DELIM) {
        Some(r) => r.strip_prefix('\n').unwrap_or(r),
        None => return (Front::default(), text),
    };
    let close_marker = format!("\n{DELIM}");
    let close = match rest.find(&close_marker) {
        Some(i) => i,
        None => return (Front::default(), text),
    };
    let front_text = &rest[..close];
    let after = &rest[close + close_marker.len()..];
    let after = after.strip_prefix('\n').unwrap_or(after);
    let body = after.strip_prefix('\n').unwrap_or(after);
    let front: Front = toml::from_str(front_text).unwrap_or_default();
    (front, body)
}

pub fn join(front: &Front, body: &str) -> String {
    let front_text = toml::to_string(front).unwrap_or_default();
    if front_text.trim().is_empty() {
        return body.to_string();
    }
    format!("{DELIM}\n{front_text}{DELIM}\n\n{body}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_no_frontmatter() {
        let (front, body) = split("hello world\n");
        assert!(front.tags.is_empty());
        assert_eq!(body, "hello world\n");
    }

    #[test]
    fn split_with_tags() {
        let text = "+++\ntags = [\"work\", \"urgent\"]\n+++\n\nhello\n";
        let (front, body) = split(text);
        assert_eq!(front.tags, vec!["work", "urgent"]);
        assert_eq!(body, "hello\n");
    }

    #[test]
    fn roundtrip_preserves_extra() {
        let text = "+++\ntitle = \"foo\"\ntags = [\"a\"]\n+++\n\nbody\n";
        let (mut front, body) = split(text);
        front.tags.push("b".to_string());
        let out = join(&front, body);
        let (back, _) = split(&out);
        assert_eq!(back.tags, vec!["a", "b"]);
        assert_eq!(
            back.extra.get("title").and_then(|v| v.as_str()),
            Some("foo")
        );
    }
}
