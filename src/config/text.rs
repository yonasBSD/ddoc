use {
    crate::*,
    std::{
        borrow::Cow,
        fmt,
    },
};

#[derive(Debug, Clone)]
pub enum Text {
    String(String),
    PreviousPageTitle,
    CurrentPageTitle,
    NextPageTitle,
    Var(String),
}

impl Text {
    pub fn from_cow(s: Cow<str>) -> Text {
        match s.as_ref() {
            "--previous-page-title" => Text::PreviousPageTitle,
            "--current-page-title" => Text::CurrentPageTitle,
            "--next-page-title" => Text::NextPageTitle,
            _ => {
                if let Some(var_name) = s.strip_prefix("--") {
                    Text::Var(var_name.to_string())
                } else {
                    Text::String(s.into_owned())
                }
            }
        }
    }
}

impl fmt::Display for Text {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::PreviousPageTitle => write!(f, "--previous-page-title"),
            Self::CurrentPageTitle => write!(f, "--current-page-title"),
            Self::NextPageTitle => write!(f, "--next-page-title"),
            Self::Var(var_name) => write!(f, "--{}", var_name),
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
impl From<AttributeValue> for Text {
    fn from(value: AttributeValue) -> Self {
        match value {
            AttributeValue::String(s) => Self::from(s),
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
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}
