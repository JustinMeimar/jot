use std::path::PathBuf;
use std::process::ExitStatus;

pub type Result<T> = std::result::Result<T, JotError>;

#[derive(Debug)]
pub enum JotError {
    Io(std::io::Error),
    NoHome,
    NoEditor,
    ConfigParse(toml::de::Error),
    ConfigSerialize(toml::ser::Error),
    ReservedName(String),
    UnknownVariant(String),
    AlreadyExists(String),
    EditorFailed(ExitStatus),
    PathTaken(PathBuf),
}

impl std::fmt::Display for JotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::NoHome => write!(f, "HOME is not set"),
            Self::NoEditor => write!(f, "EDITOR is not set"),
            Self::ConfigParse(e) => write!(f, "config parse: {e}"),
            Self::ConfigSerialize(e) => write!(f, "config serialize: {e}"),
            Self::ReservedName(n) => {
                write!(f, "variant name {n:?} is reserved")
            }
            Self::UnknownVariant(n) => write!(f, "unknown variant {n:?}"),
            Self::AlreadyExists(n) => write!(f, "variant {n:?} already exists"),
            Self::EditorFailed(s) => write!(f, "editor exited with {s}"),
            Self::PathTaken(p) => {
                write!(f, "path already exists: {}", p.display())
            }
        }
    }
}

impl std::error::Error for JotError {}

impl From<std::io::Error> for JotError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<toml::de::Error> for JotError {
    fn from(e: toml::de::Error) -> Self {
        Self::ConfigParse(e)
    }
}

impl From<toml::ser::Error> for JotError {
    fn from(e: toml::ser::Error) -> Self {
        Self::ConfigSerialize(e)
    }
}
