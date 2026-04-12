use {
    crate::*,
    rust_embed::Embed,
    std::{
        collections::HashSet,
        path::Path,
    },
    termimad::crossterm::style::Stylize,
};

#[derive(Embed)]
#[folder = "resources/plugins"]
struct EmbeddedPlugins;

#[derive(Debug, Clone)]
pub struct EmbeddedPlugin {
    name: String,
}

impl EmbeddedPlugins {
    fn list_plugins() -> Vec<String> {
        let mut dirs = HashSet::new();

        for file_path in Self::iter() {
            let path = Path::new(file_path.as_ref());
            // If the path has a parent that isn't empty, it's in a subdirectory
            if let Some(first_segment) = path.components().next() {
                // Check if there are more components (meaning it's a directory, not a file at root)
                if path.components().count() > 1 {
                    dirs.insert(first_segment.as_os_str().to_string_lossy().to_string());
                }
            }
        }
        dirs.into_iter().collect()
    }
}

/// Get the bytes of a file from the plugin name and the path starting from its
/// src/ directory (e.g. "css/main.css")
///
/// This function is intended for verified paths, so the returned Error is
/// an 'Internal' one.
pub fn resource_file_bytes(
    plugin: &'static str,
    file_kind: &'static str,
    file_name: &'static str,
) -> DdResult<Vec<u8>> {
    let path = format!("{}/src/{}/{}", plugin, file_kind, file_name);
    EmbeddedPlugins::get(&path)
        .ok_or_else(|| {
            DdError::internal(format!("File '{}' not found in embedded resources", path))
        })
        .map(|file| file.data.to_vec())
}

pub fn plugin_is_known(plugin: &str) -> bool {
    EmbeddedPlugins::list_plugins().contains(&plugin.to_string())
}

impl EmbeddedPlugin {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn all() -> Vec<Self> {
        EmbeddedPlugins::list_plugins()
            .into_iter()
            .map(|name| Self { name })
            .collect()
    }
    pub fn get(name: &str) -> Option<Self> {
        if EmbeddedPlugins::list_plugins().contains(&name.to_string()) {
            Some(Self {
                name: name.to_string(),
            })
        } else {
            None
        }
    }
    pub fn print_list(_project_opt: Option<&Project>) {
        let plugins = Self::all();
        if plugins.is_empty() {
            println!("No plugins found");
        } else {
            println!("Known plugins:");
            for plugin in plugins {
                println!("- {}", plugin.name());
            }
        }
    }
    /// Extract the plugin's directory from the embedded resources to the specified destination
    /// directory (which should be the `plugins/` directory of a ddoc project)
    fn extract(
        &self,
        dst_dir: &Path,
    ) -> DdResult<()> {
        let plugin_path = format!("{}/", self.name);
        for file_path in EmbeddedPlugins::iter() {
            if file_path.as_ref().starts_with(&plugin_path) {
                let dst_file_path = dst_dir.join(file_path.as_ref());
                if let Some(parent) = dst_file_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(
                    dst_file_path,
                    EmbeddedPlugins::get(file_path.as_ref()).unwrap().data,
                )?;
            }
        }
        Ok(())
    }
    pub fn init(
        &self,
        project_path: &Path, // root of the project
    ) -> DdResult<bool> {
        let project_plugins_dir = project_path.join("plugins");
        if !project_plugins_dir.exists() {
            eprintln!(
                "Directory {} does not exist.",
                project_plugins_dir.to_string_lossy().red().bold(),
            );
            eprintln!("Please check that the project exists and is valid.");
            return Ok(false);
        }
        let dst_plugin_dir = project_plugins_dir.join(self.name());
        if dst_plugin_dir.exists() {
            eprintln!(
                "Plugin {} already exists in the project.",
                self.name().yellow().bold(),
            );
            let skin = termimad::MadSkin::default();
            termimad::ask!(
            &skin,
            "Do you want to overwrite it with the embedded version?", ('n') {
                ('y', "Yes, remove the existing plugin") => {
                    info!("Confirming old plugin removal");
                }
                ('n', "No, abort the operation") => {
                    info!("Aborting plugin initialization.");
                    return Ok(false);
                }
            });
            std::fs::remove_dir_all(&dst_plugin_dir)?;
        }
        self.extract(&project_plugins_dir)?;
        eprintln!(
            "Initialized plugin {} in the project",
            self.name().yellow().bold(),
        );
        Ok(true)
    }
}

#[test]
fn check_default_plugin_exists() {
    let default_plugin = EmbeddedPlugin::default();
    assert!(
        EmbeddedPlugin::get(default_plugin.name()).is_some(),
        "Default plugin '{}' should exist in embedded plugins",
        default_plugin.name()
    );
}
