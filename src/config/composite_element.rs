use {
    crate::*,
    serde::de,
    std::fmt,
};

#[derive(Debug, Clone, Default)]
pub struct CompositeElement {
    pub entries: Vec<CompositeElementEntry>,
}
#[derive(Debug, Clone)]
pub struct CompositeElementEntry {
    pub key: ElementKey,
    pub value: Element,
}

impl CompositeElement {
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&CompositeElementEntry> {
        self.entries.get(index)
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn visit<F>(
        &self,
        mut f: F,
    ) where
        F: FnMut(&ElementKey, &Element),
    {
        for entry in &self.entries {
            f(&entry.key, &entry.value);
            if let Element::Composite(comp) = &entry.value {
                comp.visit(&mut f);
            }
        }
    }
}

pub struct CompositeElementDeserializer {}
impl<'de> de::Visitor<'de> for CompositeElementDeserializer {
    type Value = CompositeElement;

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
        let mut entries = Vec::new();
        while let Some((key, value)) = access.next_entry::<ElementKey, Element>()? {
            entries.push(CompositeElementEntry { key, value });
        }
        Ok(Self::Value { entries })
    }
}
impl<'de> de::Deserialize<'de> for CompositeElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(CompositeElementDeserializer {})
    }
}

#[test]
fn test_element_deserialization() {
    let hjson = r#"
    {
        header: {
            div.before-menu: {
                ddoc-link: {
                    img: img/dystroy-rust-white.svg
                    href: https://dystroy.org
                    alt: dystroy.org homepage
                    class: external-nav-link
                }
                ddoc-link: {
                    url: /index.md
                    alt: ddoc homepage
                    label: ddoc
                    class: home-link
                }
            }
            ddoc-menu: {
                hamburger-checkbox: true
            }
            div.after-menu: {
                ddoc-link: {
                    img: img/ddoc-left-arrow.svg
                    href: --previous
                    class: previous-page-link
                    alt: Previous Page
                }
                ddoc-link: {
                    img: img/ddoc-search.svg
                    href: --search
                    class: search-opener
                    alt: Search
                }
                ddoc-link: {
                    img: img/ddoc-right-arrow.svg
                    href: --next
                    class: next-page-link
                    alt: Next Page
                }
                ddoc-link: {
                    img: img/github-mark-white.svg
                    class: external-nav-link
                    alt: GitHub
                    href: https://github.com/Canop/ddoc
                }
            }
        }
        article: {
            aside.page-nav: {
                ddoc-toc: {}
            }
            ddoc-main: {}
        }
        footer: {
            div.made-with-ddoc: {
                ddoc-link: {
                    label: made with
                }
                ddoc-link: {
                    label: ddoc
                    href: https://dystroy.org/ddoc
                    class: link-to-ddoc
                }
            }
        }

    }
    "#;
    let element: Element = deser_hjson::from_str(hjson).unwrap();
    let header = element
        .as_composite()
        .unwrap()
        .get(0)
        .unwrap()
        .value
        .as_composite()
        .unwrap();
    assert_eq!(header.len(), 3);
    let after_menu = header.get(2).unwrap();
    assert_eq!(after_menu.key.to_string(), "div.after-menu");
    assert_eq!(after_menu.key.classes[0], "after-menu");
    let after_menu = after_menu.value.as_composite().unwrap();
    assert_eq!(after_menu.len(), 4);
    assert_eq!(after_menu.get(2).unwrap().key.etype, ElementType::Link);
    assert_eq!(
        after_menu
            .get(2)
            .unwrap()
            .value
            .as_attributes()
            .unwrap()
            .get("href")
            .unwrap()
            .as_str()
            .unwrap(),
        "--next"
    );
    let toc = element
        .as_composite()
        .unwrap()
        .get(1)
        .unwrap()
        .value
        .as_composite()
        .unwrap()
        .get(0)
        .unwrap()
        .value
        .as_composite()
        .unwrap()
        .get(0)
        .unwrap();
}
