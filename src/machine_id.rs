use serde::Deserialize;
use serde::Serialize;

#[derive(Debug)]
pub struct MachineId {
    pub inner: String,
}
impl AsRef<str> for MachineId {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}
impl MachineId {
    pub fn new(inner: String) -> Self {
        Self { inner }
    }
}
impl From<String> for MachineId {
    fn from(inner: String) -> Self {
        Self::new(inner)
    }
}
impl From<&str> for MachineId {
    fn from(inner: &str) -> Self {
        Self::new(inner.to_string())
    }
}
impl From<&MachineId> for String {
    fn from(value: &MachineId) -> Self {
        value.inner.clone()
    }
}
impl From<MachineId> for String {
    fn from(value: MachineId) -> Self {
        value.inner
    }
}
impl std::fmt::Display for MachineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
impl std::str::FromStr for MachineId {
    type Err = std::string::FromUtf8Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
impl std::ops::Deref for MachineId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MachineId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl<'de> Deserialize<'de> for MachineId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = String::deserialize(deserializer)?;
        Ok(Self::new(inner))
    }
}
impl Serialize for MachineId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}