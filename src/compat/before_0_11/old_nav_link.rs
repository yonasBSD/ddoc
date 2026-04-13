use {
    crate::*,
    serde::{
        Deserialize,
        Serialize,
    },
};

/// A single link in the navigation bar
/// (for the old doc format of ddoc < 0.12)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OldNavLink {
    pub img: Option<String>,
    pub inline: Option<String>,
    pub label: Option<Text>,
    pub alt: Option<String>,
    #[serde(alias = "url")]
    pub href: Option<String>,
    pub class: Option<String>,
    pub target: Option<String>,
}

impl OldNavLink {
    /// Convert to the new `NavLink` structure
    pub fn to_nav_link(&self) -> NavLink {
        let mut content = Vec::new();
        if let Some(img_src) = &self.img {
            content.push(NavLinkPart::Img {
                src: img_src.clone(),
                alt: self.alt.clone(),
            });
        }
        if let Some(inline_src) = &self.inline {
            content.push(NavLinkPart::InlineImg {
                src: inline_src.clone(),
            });
        }
        if let Some(label) = &self.label {
            content.push(NavLinkPart::Label(label.clone()));
        }
        NavLink {
            href: self.href.clone(),
            target: self.target.clone(),
            content,
        }
    }
    pub fn classes(&self) -> Vec<String> {
        if let Some(class_str) = &self.class {
            vec![class_str.to_string()]
        } else {
            Vec::new()
        }
    }
}
