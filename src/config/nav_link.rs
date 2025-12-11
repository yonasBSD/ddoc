use serde::{
    Deserialize,
    Serialize,
};

/// A single link in the navigation bar
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NavLink {
    pub img: Option<String>,
    pub label: Option<String>,
    pub alt: Option<String>,
    #[serde(alias = "url")]
    pub href: Option<String>,
    pub class: Option<String>,
    pub target: Option<String>,
}
