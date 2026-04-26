mod attribute;
mod element;
mod element_key;
mod element_list;
mod menu;
mod nav_link;
mod page_list;
mod text;
mod toc;

pub use {
    attribute::*,
    element::*,
    element_key::*,
    element_list::*,
    menu::*,
    nav_link::*,
    page_list::*,
    text::*,
    toc::*,
};

use {
    crate::{
        before_0_11::NavComponents,
        *,
    },
    indexmap::IndexMap,
    serde::{
        Deserialize,
        Deserializer,
    },
    std::path::Path,
};

pub static CONFIG_FILE_NAME: &str = "ddoc.hjson";

pub type ClassName = String;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub title: Option<String>,
    pub description: Option<String>,
    pub ddoc_version: Option<String>,
    #[serde(default, alias = "active-plugins")]
    pub active_plugins: Vec<String>,
    #[serde(default, alias = "pages", alias = "menu")]
    pub site_map: PageList,
    pub favicon: Option<String>,
    /// for compatibility with ddoc (0.11-), this is loaded but only used
    /// through conversion to the new `body` field
    #[serde(flatten)]
    pub old: NavComponents,
    #[serde(
        default = "default_body_element",
        deserialize_with = "deserialize_body_element"
    )]
    pub body: Element,
    #[serde(default)]
    pub vars: IndexMap<String, String>,
}

impl Config {
    /// Read the ddoc.hjson configuration file at the root of a ddoc module
    ///
    /// Return both the config and the path where it was found
    ///
    /// # Errors
    /// Return `DdError::ConfigNotFound` if no ddoc.hjson is found at the specified path
    /// or other `DdError` variants on read/parse errors
    pub fn in_dir(path: &Path) -> DdResult<Option<Sourced<Self>>> {
        let config_path = path.join(CONFIG_FILE_NAME);
        if !config_path.exists() {
            return Ok(None);
        }
        let config: Config = read_file(&config_path)?;
        let config = Sourced::new(config, config_path);
        Ok(Some(config))
    }
    pub fn title(&self) -> &str {
        self.title.as_deref().unwrap_or("Untitled")
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref().filter(|s| !s.is_empty())
    }
    pub fn favicon(&self) -> Option<&str> {
        self.favicon.as_deref().filter(|s| !s.is_empty())
    }
    pub fn var(
        &self,
        name: &str,
    ) -> Option<String> {
        match name {
            "title" => Some(self.title().to_string()),
            "description" => self.description().map(|s| s.to_string()),
            "favicon" => self.favicon().map(|s| s.to_string()),
            _ => self.vars.get(name).cloned(),
        }
    }

    pub fn has_any_plugin(&self) -> bool {
        !self.active_plugins.is_empty()
    }

    /// Add to this main config element the config of a plugin
    pub fn merge(
        &mut self,
        other: &Config,
    ) {
        if self.title.is_none() {
            self.title = other.title.clone();
        }
        if self.description.is_none() {
            self.description = other.description.clone();
        }
        if self.favicon.is_none() {
            self.favicon = other.favicon.clone();
        }
        for (key, value) in &other.vars {
            if !self.vars.contains_key(key) {
                self.vars.insert(key.clone(), value.clone());
            }
        }
        for plugin in &other.active_plugins {
            if !self.active_plugins.contains(plugin) {
                self.active_plugins.push(plugin.clone());
            }
        }
        if !self.body.try_merge(&other.body) {
            warn!(
                "Plugin config body could not be merged into main config body, plugin body will be ignored"
            );
        }
    }
}

fn default_body_element() -> Element {
    Element {
        classes: vec![],
        content: ElementContent::DomTree {
            tag: "body".to_string(),
            children: vec![],
        },
    }
}
fn deserialize_body_element<'de, D: Deserializer<'de>>(
    deserializer: D
) -> Result<Element, D::Error> {
    let element_list = ElementList::deserialize(deserializer)?;
    let element = Element {
        classes: vec![],
        content: ElementContent::DomTree {
            tag: "body".to_string(),
            children: element_list.children,
        },
    };
    Ok(element)
}
