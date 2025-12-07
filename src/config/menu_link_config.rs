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
    #[serde(alias = "href")]
    pub url: Option<String>,
    pub class: Option<String>,
}

impl NavLinksConfig {
    pub fn has_href(&self, href: &str) -> bool {
        for link in &self.before_menu {
            if let Some(url) = &link.url {
                if url == href {
                    return true;
                }
            }
        }
        for link in &self.after_menu {
            if let Some(url) = &link.url {
                if url == href {
                    return true;
                }
            }
        }
        false
    }
}
