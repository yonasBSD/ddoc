use {
    crate::*,
    indexmap::IndexMap,
    serde::{
        Deserialize,
        Serialize,
    },
};

pub type ClassName = String;

/// Either the header or footer navigation configuration
/// (could be used elsewhere)
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NavDir {
    pub components: IndexMap<ClassName, NavComponent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NavComponent {
    Menu(MenuInsert),
    NavLinks(Vec<NavLink>),
}

impl NavDir {
    /// Check if any of the nav links has the specified href
    pub fn has_href(
        &self,
        href: &str,
    ) -> bool {
        for component in self.components.values() {
            if let NavComponent::NavLinks(links) = component {
                for link in links {
                    if link.href.as_deref() == Some(href) {
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
}

#[test]
fn test_nav_component_deserialize() {
    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct TestConfig {
        header: NavDir,
        footer: NavDir,
    }
    let hjson = r#"
        header: {
            before-menu: [
                {
                    img: img/dystroy-rust-white.svg
                    href: https://dystroy.org
                    alt: dystroy.org homepage
                    class: external-nav-link
                }
                {
                    url: /index.md
                    alt: ddoc homepage
                    label: ddoc
                    class: home-link
                }
            ]
            middle: menu
            after-menu: [
                {
                    img: img/ddoc-left-arrow.svg
                    href: --previous
                    class: previous-page-link
                    alt: Previous Page
                }
                {
                    img: img/ddoc-search.svg
                    href: --search
                    class: search-opener
                    alt: Search
                }
                {
                    img: img/ddoc-right-arrow.svg
                    url: --next
                    class: next-page-link
                    alt: Next Page
                }
                {
                    img: img/github-mark-white.svg
                    class: external-nav-link
                    alt: GitHub
                    href: https://github.com/Canop/ddoc
                }
            ]
        }
        footer: {
            right: [
                {
                    label: made with **ddoc**
                    href: https://dystroy.org/ddoc
                }
            ]
        }
    "#;
    let test_config: TestConfig = deser_hjson::from_str(hjson).unwrap();
    assert_eq!(test_config.header.components.len(), 3);
    assert_eq!(test_config.footer.components.len(), 1);
    assert!(matches!(
        test_config.header.components.get("middle").unwrap(),
        NavComponent::Menu(_)
    ));
    let NavComponent::NavLinks(links) = test_config.header.components.get("after-menu").unwrap()
    else {
        panic!("Expected NavLinks");
    };
    assert_eq!(links[2].href.as_deref(), Some("--next"));
}
