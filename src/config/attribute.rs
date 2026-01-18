use {
    indexmap::IndexMap,
    serde::Deserialize,
};

pub type AttributeKey = String;

/// The value of an attribute in a ddoc element (`ddoc-link`, `ddoc-menu`, etc).
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AttributeValue {
    String(String),
    Bool(bool),
}
impl AttributeValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            Self::Bool(_) => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::String(_) => None,
            Self::Bool(b) => Some(*b),
        }
    }
}

impl std::fmt::Display for AttributeValue {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
        }
    }
}

pub type Attributes = IndexMap<AttributeKey, AttributeValue>;
