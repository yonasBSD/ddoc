mod attribute;
mod before_0_11;
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
    crate::*,
    before_0_11::NavComponents,
    serde::Deserialize,
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
    old: NavComponents,
    #[serde(default)]
    pub body: ElementList,
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
    pub fn needs_search_script(&self) -> bool {
        self.body.has_href("--search")
    }
    pub fn needs_toc_activate_script(&self) -> bool {
        self.body.has(|element: &Element| {
            if let ElementContent::Toc(toc) = &element.content {
                return toc.activate_visible_item;
            }
            false
        })
    }
    /// For support of old ddoc versions (<= 0.11), convert old nav components
    /// if the new `body` field is empty
    pub fn fix_old(&mut self) {
        if self.body.children.is_empty() {
            self.body = self.old.to_body_composite();
        }
    }

    /// Add to this main config element the config of a plugin
    pub fn merge(
        &mut self,
        other: &Config,
    ) {
        eprintln!(
            "Merging config: main title {:?}, plugin title {:?}",
            self.title, other.title
        );
        if self.title.is_none() {
            self.title = other.title.clone();
        }
        if self.description.is_none() {
            self.description = other.description.clone();
        }
        if self.favicon.is_none() {
            self.favicon = other.favicon.clone();
        }
        for plugin in &other.active_plugins {
            if !self.active_plugins.contains(plugin) {
                self.active_plugins.push(plugin.clone());
            }
        }
        self.body.merge(&other.body);
    }
}
