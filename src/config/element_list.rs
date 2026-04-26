use {
    crate::*,
    serde::{
        Deserialize,
        de,
    },
    std::fmt,
    termimad::crossterm::style::Stylize,
};

/// A collection of elements, a vessel for deserializing
#[derive(Debug, Clone, Default)]
pub struct ElementList {
    pub children: Vec<Element>,
}

pub struct ElementListDeserializer {}
impl<'de> de::Visitor<'de> for ElementListDeserializer {
    type Value = ElementList;

    fn expecting(
        &self,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        formatter.write_str("a composite element")
    }
    fn visit_map<M>(
        self,
        mut access: M,
    ) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        #[derive(Debug, Clone, Deserialize)]
        #[serde(untagged)]
        enum DeserContent {
            Composite(ElementList),
            Attributes(Attributes),
        }
        let mut children = Vec::new();
        while let Some((key, value)) = access.next_entry::<ElementKey, DeserContent>()? {
            let ElementKey { etype, classes: _ } = key;
            let content = match (etype, value) {
                (ElementType::HtmlTag(tag), DeserContent::Composite(comp)) => {
                    ElementContent::DomTree {
                        tag,
                        children: comp.children,
                    }
                }
                (ElementType::HtmlTag(tag), DeserContent::Attributes(mut attrs)) => {
                    let text = attrs.shift_remove("text").map(Text::from);
                    let raw_html = attrs.shift_remove("html").map(|v| v.to_string());
                    ElementContent::DomLeaf {
                        tag,
                        text,
                        raw_html,
                        attributes: attrs,
                    }
                }
                (ElementType::Link, DeserContent::Attributes(attrs)) => {
                    let nav_link: NavLink = attrs.into();
                    ElementContent::Link(nav_link)
                }
                (ElementType::Menu, DeserContent::Attributes(attrs)) => {
                    let menu_insert: Menu = attrs.into();
                    ElementContent::Menu(menu_insert)
                }
                (ElementType::Toc, DeserContent::Attributes(attrs)) => {
                    let toc: Toc = attrs.into();
                    ElementContent::Toc(toc)
                }
                (ElementType::Menu, _) => ElementContent::Menu(Menu::default()),
                (ElementType::Toc, _) => ElementContent::Toc(Toc::default()),
                (ElementType::Main, _) => ElementContent::Main,
                (ElementType::PageTitle, _) => ElementContent::PageTitle,
                (etype, value) => {
                    eprintln!(
                        "{}: invalid element type {} for value {:?}",
                        "error".red(),
                        etype.to_string().yellow(),
                        value,
                    );
                    return Err(de::Error::custom(format!(
                        "invalid element type {:?} for value {:?}",
                        etype, value
                    )));
                }
            };
            children.push(Element {
                classes: key.classes,
                content,
            });
        }
        Ok(Self::Value { children })
    }
}
impl<'de> de::Deserialize<'de> for ElementList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(ElementListDeserializer {})
    }
}
