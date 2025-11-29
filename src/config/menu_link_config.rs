use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NavLinksConfig {
    #[serde(default)]
    pub before_menu: Vec<MenuLinkConfig>,
    #[serde(default)]
    pub after_menu: Vec<MenuLinkConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MenuLinkConfig {
    pub img: Option<String>,
    pub label: Option<String>,
    pub alt: Option<String>,
    pub url: Option<String>,
    pub class: Option<String>,
}
