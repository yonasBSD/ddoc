use {
    crate::*,
    indexmap::IndexMap,
    termimad::crossterm::style::Stylize,
};

/// A single link in the navigation bar
#[derive(Debug, Clone)]
pub struct NavLink {
    pub href: Option<String>,
    pub target: Option<String>,
    pub content: Vec<NavLinkPart>,
}

#[derive(Debug, Clone)]
pub enum NavLinkPart {
    Label(Text),
    Img { src: String, alt: Option<String> },
    InlineImg { src: String },
}

impl From<Attributes> for NavLink {
    fn from(map: IndexMap<AttributeKey, AttributeValue>) -> Self {
        let mut alt = None;
        let mut content = Vec::new();
        let mut href = None;
        let mut target = None;
        for (key, value) in &map {
            match key.as_str() {
                "alt" => {
                    let mut added = false;
                    for part in &mut content {
                        if let NavLinkPart::Img { alt, .. } = part {
                            *alt = Some(value.to_string());
                            added = true;
                            break;
                        }
                    }
                    if !added {
                        alt = Some(value.to_string());
                    }
                }
                "img" => {
                    if let Some(src) = value.as_str() {
                        let img_part = NavLinkPart::Img {
                            src: src.to_string(),
                            alt: alt.take(),
                        };
                        content.push(img_part);
                    }
                }
                "inline" => {
                    if let Some(src) = value.as_str() {
                        let inline_part = NavLinkPart::InlineImg {
                            src: src.to_string(),
                        };
                        content.push(inline_part);
                    }
                }
                "label" => {
                    let label_part = NavLinkPart::Label(value.into());
                    content.push(label_part);
                }
                "href" | "url" => {
                    // note: "url" is deprecated and not documented
                    href = Some(value.to_string());
                }
                "target" | "link_target" => {
                    target = Some(value.to_string());
                }
                key => {
                    eprintln!(
                        "{}: unknown attribute in nav-link: {}",
                        "warning".yellow().bold(),
                        key.red(),
                    );
                }
            }
        }
        Self {
            href,
            target,
            content,
        }
    }
}
