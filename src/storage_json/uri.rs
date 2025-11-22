use std::path::PathBuf;
use std::rc::Rc;

use eyre::bail;
use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;
use percent_encoding::utf8_percent_encode;
use serde::Deserialize;
use serde::Serialize;

// Define characters that need to be encoded in file URIs
// Based on RFC 3986, encodes special characters that have meaning in URIs
// Forward slashes (/) are preserved as path separators
const PATH_SEGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}')
    .add(b'%')
    .add(b':');

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_uri_round_trip_with_spaces() {
        let original = Uri::LocalPath(PathBuf::from("C:/Program Files/VSCode/test.txt"));
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: Uri = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
        assert!(serialized.contains("%20")); // Space should be encoded
    }

    #[test]
    fn test_uri_round_trip_with_hash() {
        let original = Uri::LocalPath(PathBuf::from("C:/path#with#hash/file.txt"));
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: Uri = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
        assert!(serialized.contains("%23")); // Hash should be encoded
    }

    #[test]
    fn test_uri_round_trip_with_question() {
        let original = Uri::LocalPath(PathBuf::from("C:/path?with?question/file.txt"));
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: Uri = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
        assert!(serialized.contains("%3F")); // Question mark should be encoded
    }

    #[test]
    fn test_uri_round_trip_with_colon() {
        let original = Uri::LocalPath(PathBuf::from("C:/normal/path/file.txt"));
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: Uri = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
        assert!(serialized.contains("%3A")); // Colon should be encoded
    }

    #[test]
    fn test_uri_round_trip_unix_path() {
        let original = Uri::LocalPath(PathBuf::from("/home/user/with spaces/file.txt"));
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: Uri = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
        assert!(serialized.contains("%20")); // Space should be encoded
    }
}
