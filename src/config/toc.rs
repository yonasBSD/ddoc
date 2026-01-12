use {
    crate::*,
    std::{
        fmt,
        str::FromStr,
    },
};

/// The settings for the insertion of a table of content in a web page.
#[derive(Debug, Clone, Copy, Default)]
pub struct Toc {
    pub activate_visible_item: bool,
}

impl FromStr for Toc {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s != "toc" {
            return Err("Toc must be 'toc'");
        }
        Ok(Self {
            activate_visible_item: true,
        })
    }
}

impl From<Attributes> for Toc {
    fn from(map: Attributes) -> Self {
        let mut toc_insert = Toc::default();
        if let Some(v) = map
            .get("activate-visible-item")
            .or_else(|| map.get("activate_visible_item"))
        {
            if let Some(b) = v.as_bool() {
                toc_insert.activate_visible_item = b;
            }
        }
        toc_insert
    }
}

impl fmt::Display for Toc {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "toc")
    }
}
impl serde::Serialize for Toc {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for Toc {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
