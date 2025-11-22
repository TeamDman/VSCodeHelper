use std::path::PathBuf;
use std::rc::Rc;

use eyre::bail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Uri {
    LocalPath(PathBuf),
    VsCodeRemotePath(Rc<str>),
    Unknown(Rc<str>),
}

impl Uri {
    pub fn as_path(&self) -> eyre::Result<PathBuf> {
        match self {
            Uri::LocalPath(path) => Ok(path.clone()),
            Uri::VsCodeRemotePath(s) => bail!("URI is a remote path: {}", s),
            Uri::Unknown(s) => bail!("URI is unknown: {}", s),
        }
    }

    pub fn protocol(&self) -> &'static str {
        match self {
            Uri::LocalPath(_) => "file",
            Uri::VsCodeRemotePath(_) => "vscode-remote",
            Uri::Unknown(_) => "unknown",
        }
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Uri::LocalPath(path) => {
                // Best effort serialization to VSCode URI format
                let path_str = path.to_string_lossy().replace('\\', "/");
                // Encode special characters if needed, but for now simple replacement
                // VSCode tends to encode ':' as '%3A' for drive letters
                if let Some(colon_idx) = path_str.find(':') {
                    let (drive, rest) = path_str.split_at(colon_idx);
                    // rest starts with ':'
                    write!(f, "file:///{}{}{}", drive, "%3A", &rest[1..])
                } else {
                    write!(f, "file:///{}", path_str)
                }
            }
            Uri::VsCodeRemotePath(s) => write!(f, "{}", s),
            Uri::Unknown(s) => write!(f, "{}", s),
        }
    }
}

impl TryFrom<Uri> for PathBuf {
    type Error = eyre::Error;

    fn try_from(value: Uri) -> Result<Self, Self::Error> {
        value.as_path()
    }
}

impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some(stripped) = s.strip_prefix("file:///") {
            let path_unescaped = percent_encoding::percent_decode(stripped.as_bytes())
                .decode_utf8()
                .map_err(serde::de::Error::custom)?;
            let path = PathBuf::from(path_unescaped.to_string());
            Ok(Uri::LocalPath(path))
        } else if s.starts_with("vscode-remote://") {
            Ok(Uri::VsCodeRemotePath(Rc::from(s)))
        } else {
            Ok(Uri::Unknown(Rc::from(s)))
        }
    }
}
