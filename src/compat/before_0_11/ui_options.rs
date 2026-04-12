use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UiOptions {
    /// If true, a hamburger checkbox is added to the UI for toggling
    /// the menu on small screens.
    ///
    /// When it's true (default), CSS becomes necessary
    #[serde(default = "bool_true")]
    pub hamburger_checkbox: bool,
}

impl Default for UiOptions {
    fn default() -> Self {
        Self {
            hamburger_checkbox: true,
        }
    }
}

pub fn bool_true() -> bool {
    true
}
