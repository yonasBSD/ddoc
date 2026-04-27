use {
    crate::*,
    std::{
        fmt,
        str::FromStr,
    },
};

/// The settings for the insertion of a table of content in a web page.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Toc {
    /// The title of the table of content. If not specified, the default title is the
    /// title of the current page.
    pub title: Option<String>,
    // This is used for compatiblity with ddoc before 0.17
    // (for later version, enable the toc-activate plugin instead, which is more powerful and
    // doesn't require a special script)
    pub activate_visible_item: bool,
}

impl FromStr for Toc {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s != "toc" {
            return Err("Toc must be 'toc'");
        }
        Ok(Self {
            title: None,
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
        if let Some(v) = map.get("title") {
            if let Some(s) = v.as_str() {
                toc_insert.title = Some(s.to_string());
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
