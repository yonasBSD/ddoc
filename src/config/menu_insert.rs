use std::{
    fmt,
    str::FromStr,
};

/// A placeholder to insert the menu at this position in the nav bar
///
/// Rigth now there's no configuration but it could be extended in the future
#[derive(Debug, Clone, Copy)]
pub struct MenuInsert;

impl FromStr for MenuInsert {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s != "menu" {
            return Err("MenuInsert must be 'menu'");
        }
        Ok(Self)
    }
}
impl fmt::Display for MenuInsert {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "menu")
    }
}
impl serde::Serialize for MenuInsert {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for MenuInsert {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
