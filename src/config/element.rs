use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub classes: Vec<ClassName>,
    pub content: ElementContent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementContent {
    DomLeaf {
        tag: String,
        text: Option<Text>,
        raw_html: Option<String>,
        attributes: Attributes,
        // TODO support markdown ?
    },
    DomTree {
        tag: String,
        children: Vec<Element>,
    },
    Link(NavLink),
    Menu(Menu),
    Toc(Toc),
    Main,
    PageTitle, // this one is quite obsolete now with --current-page-title
}

impl Element {
    pub fn is_empty(&self) -> bool {
        match &self.content {
            ElementContent::DomTree { children, .. } => children.is_empty(),
            _ => false,
        }
    }
    pub fn try_merge(
        &mut self,
        other: &Element,
    ) -> bool {
        struct RankedSelectorElement {
            selector: String,
            merged_element: Element,
            rank: usize,
        }
        match (&mut self.content, &other.content) {
            (
                ElementContent::DomTree {
                    children: children1,
                    ..
                },
                ElementContent::DomTree {
                    children: children2,
                    ..
                },
            ) => {
                let mut merged_elements = Vec::new();
                for (i, e1) in children1.drain(..).enumerate() {
                    merged_elements.push(RankedSelectorElement {
                        selector: e1.selector(),
                        merged_element: e1,
                        rank: i,
                    });
                }
                for (i, e2) in children2.iter().enumerate() {
                    let selector = e2.selector();
                    if let Some(e1) = merged_elements.iter_mut().find(|e| e.selector == selector) {
                        if e1.merged_element.try_merge(e2) {
                            e1.rank = i.max(e1.rank);
                            continue;
                        }
                    }
                    merged_elements.push(RankedSelectorElement {
                        selector,
                        merged_element: e2.clone(),
                        rank: i,
                    });
                }
                merged_elements.sort_by_key(|e| e.rank);
                for e in merged_elements {
                    children1.push(e.merged_element);
                }
                true
            }
            _ => false,
        }
    }
    /// The pseudo-tag of this element, as used in configuration
    pub fn tag(&self) -> &str {
        match &self.content {
            ElementContent::DomLeaf { tag, .. } => tag,
            ElementContent::DomTree { tag, .. } => tag,
            ElementContent::Link(_) => "ddoc-link",
            ElementContent::Menu(_) => "ddoc-menu",
            ElementContent::Toc(_) => "ddoc-toc",
            ElementContent::Main => "ddoc-main",
            ElementContent::PageTitle => "ddoc-page-title",
        }
    }
    pub fn children(&self) -> Option<&Vec<Element>> {
        match &self.content {
            ElementContent::DomTree { children, .. } => Some(children),
            _ => None,
        }
    }
    pub fn new_composite(
        key: &str, // tag.class1.class2
        children: Vec<Element>,
    ) -> Self {
        let mut tokens = key.split('.');
        let tag = tokens.next().unwrap_or("div").to_string();
        let classes = tokens.map(|s| s.to_string()).collect();
        Self {
            classes,
            content: ElementContent::DomTree { tag, children },
        }
    }
    pub fn visit<F>(
        &self,
        f: &mut F,
    ) where
        F: FnMut(&Element),
    {
        f(self);
        if let Some(children) = self.children() {
            for child in children {
                child.visit(f);
            }
        }
    }
    pub fn has_href(
        &self,
        href: &str,
    ) -> bool {
        self.has(&mut |element: &Element| {
            if let ElementContent::Link(link) = &element.content {
                if link.href.as_deref() == Some(href) {
                    return true;
                }
            }
            false
        })
    }
    pub fn has<F>(
        &self,
        f: &mut F,
    ) -> bool
    where
        F: FnMut(&Element) -> bool,
    {
        if f(self) {
            return true;
        }
        if let Some(children) = self.children() {
            for child in children {
                if child.has(f) {
                    return true;
                }
            }
        }
        false
    }
    pub fn selector(&self) -> String {
        let mut selector = self.tag().to_string();
        for class in &self.classes {
            selector.push('.');
            selector.push_str(class.as_str());
        }
        selector
    }

    /// Print this element and its children in a human-readable way, for debugging purposes
    pub fn println(
        &self,
        label: &str,
    ) {
        print!("{}: ", label);
        self.print();
        println!();
    }
    /// Print this element and its children in a human-readable way, for debugging purposes
    fn print(&self) {
        print!("{}", self.selector());
        if let Some(children) = self.children() {
            if !children.is_empty() {
                print!("( ");
                for child in children {
                    child.print();
                    print!(" ");
                }
                print!(")");
            }
        }
    }
}

impl Default for Element {
    fn default() -> Self {
        Self {
            classes: vec![],
            content: ElementContent::DomTree {
                tag: "div".to_string(),
                children: vec![],
            },
        }
    }
}

impl From<ElementContent> for Element {
    fn from(content: ElementContent) -> Self {
        Self {
            classes: vec![],
            content,
        }
    }
}

// Check that the order of children is preserved when merging two elements with overlapping
// children (i.e. same selector)
#[test]
fn test_merge_order() {
    let e1 = Element::new_composite(
        "div.container",
        vec![
            Element::new_composite("div.a", vec![]),
            Element::new_composite("div.b", vec![]),
        ],
    );
    let e2 = Element::new_composite(
        "div.container",
        vec![
            Element::new_composite("div.b", vec![]),
            Element::new_composite("div.c", vec![]),
        ],
    );
    let m = Element::new_composite(
        "div.container",
        vec![
            Element::new_composite("div.a", vec![]),
            Element::new_composite("div.b", vec![]),
            Element::new_composite("div.c", vec![]),
        ],
    );
    let mut e12 = e1.clone();
    assert!(e12.try_merge(&e2));
    assert_eq!(e12, m);
    let mut e21 = e2.clone();
    assert!(e21.try_merge(&e1));
    assert_eq!(e21, m);
}
