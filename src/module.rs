use {
    crate::*,
    std::path::{
        Path,
        PathBuf,
    },
};

/// Either the main project or a plugin
pub struct Module {
    pub name: String,
    pub root: PathBuf,
    pub config: Option<Sourced<Config>>,
    pub src_path: PathBuf,
}

impl Module {
    /// Load the module from the given root directory, reading its config file
    /// (which is optional when the module is a plugin (i.e. name is not empty),
    /// but required when it's the main project).
    pub fn load<S: Into<String>>(
        name: S,
        root: &Path,
    ) -> DdResult<Self> {
        let config = Config::in_dir(root)?;
        let src_path = root.join("src");
        let module = Self {
            name: name.into(),
            config,
            root: root.to_owned(),
            src_path,
        };
        Ok(module)
    }

    pub fn is_main(&self) -> bool {
        self.name.is_empty()
    }

    fn list_static_entries_in(
        &self,
        subdir: &str,
        ext_filter: Option<&str>,
        entries: &mut Vec<StaticEntry>,
    ) -> DdResult<()> {
        let static_src = self.src_path.join(subdir);
        let prefix = format!("{}/", subdir);
        //let prefix = if self.name.is_empty() {
        //    format!("{}/", subdir)
        //} else {
        //    format!("{}/{}/", subdir, self.name)
        //};
        StaticEntry::list_in(&static_src, &prefix, ext_filter, entries)
    }

    pub fn list_js(
        &self,
        entries: &mut Vec<StaticEntry>,
    ) -> DdResult<()> {
        self.list_static_entries_in("js", Some(".js"), entries)
    }
    pub fn list_css(
        &self,
        entries: &mut Vec<StaticEntry>,
    ) -> DdResult<()> {
        self.list_static_entries_in("css", Some(".css"), entries)
    }

    pub fn add_watch_targets(
        &self,
        targets: &mut Vec<WatchTarget>,
    ) {
        if let Some(config) = &self.config {
            let config_path = config.src();
            targets.push(WatchTarget::new_file(config_path));
        }
        if self.src_path.exists() {
            targets.push(WatchTarget::new_dir(&self.src_path));
        }
    }
    fn copy_static_into(
        &self,
        dir: &str,
        build_root: &Path, // build root without the name
    ) -> DdResult<()> {
        let static_src = self.src_path.join(dir);
        if !static_src.exists() {
            return Ok(());
        }
        let static_dst = build_root.join(dir); //.join(&self.name);
        copy_normal_recursive(&static_src, &static_dst)?;
        Ok(())
    }
    pub fn copy_all_statics_into(
        &self,
        build_root: &Path, // build root without the name
    ) -> DdResult<()> {
        self.copy_static_into("js", build_root)?;
        self.copy_static_into("css", build_root)?;
        self.copy_static_into("img", build_root)?;
        Ok(())
    }
}
