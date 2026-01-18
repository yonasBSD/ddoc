use {
    crate::*,
    std::borrow::Cow,
};

#[derive(Debug, Clone)]
pub enum Text {
    String(String),
    PreviousPageTitle,
    CurrentPageTitle,
    NextPageTitle,
}

impl Text {
    pub fn from_cow(s: Cow<str>) -> Text {
        match s.as_ref() {
            "--previous-page-title" => Text::PreviousPageTitle,
            "--current-page-title" => Text::CurrentPageTitle,
            "--next-page-title" => Text::NextPageTitle,
            _ => Text::String(s.into_owned()),
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Self::String(s) => s.as_str(),
            Self::PreviousPageTitle => "--previous-page-title",
            Self::CurrentPageTitle => "--current-page-title",
            Self::NextPageTitle => "--next-page-title",
        }
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self::from_cow(Cow::Owned(s))
    }
}
impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::from_cow(Cow::Borrowed(s))
    }
}
impl From<&AttributeValue> for Text {
    fn from(value: &AttributeValue) -> Self {
        match value {
            AttributeValue::String(s) => Self::from(s.as_str()),
            AttributeValue::Bool(b) => Self::String(b.to_string()),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Text {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}
impl serde::Serialize for Text {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
