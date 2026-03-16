use {
    crate::*,
    std::{
        fs,
        path::{
            Path,
            PathBuf,
        },
        time::SystemTime,
    },
};

/// Implicit reference to one of the files in `/src/js` or `/src/css`
pub struct StaticEntry {
    /// path relative to the root of the site, as seen by the browser
    pub served_path: String,

    /// path to the file on disk
    pub src_path: PathBuf,

    /// secs since UNIX_EPOCH
    pub mtime: u64,
}

impl StaticEntry {
    /// Recursively list all the matching files in the given directory,
    /// returning a `StaticEntry` for each one.
    ///
    /// Hidden files and directories (those starting with `.`) are ignored.
    pub fn list_in(
        dir: &Path,
        serving_prefix: &str,
        ext_filter: Option<&str>,
        entries: &mut Vec<StaticEntry>,
    ) -> DdResult<()> {
        if !dir.exists() {
            return Ok(());
        }
        let mut dirs = vec![dir.to_path_buf()];
        while let Some(current_dir) = dirs.pop() {
            for entry in fs::read_dir(&current_dir)? {
                let entry = entry?;
                let src_path = entry.path();
                let file_type = entry.file_type()?;
                if file_type.is_dir() {
                    let file_name = entry.file_name();
                    let Some(file_name) = file_name.to_str() else {
                        continue;
                    };
                    if !file_name.starts_with('.') {
                        dirs.push(src_path);
                    }
                    continue;
                }
                if !file_type.is_file() {
                    continue;
                }
                let file_name = entry.file_name();
                let Some(file_name) = file_name.to_str() else {
                    continue;
                };
                if file_name.starts_with('.') {
                    continue;
                }
                if let Some(ext) = ext_filter {
                    if !file_name.ends_with(ext) {
                        continue;
                    }
                }
                let Some(mtime) = entry.metadata().and_then(|m| m.modified()).ok() else {
                    continue;
                };
                let mtime = mtime
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let relative = src_path.strip_prefix(dir).unwrap_or(&src_path);
                let served_path = format!("{}{}", serving_prefix, relative.to_string_lossy());
                entries.push(Self {
                    served_path,
                    mtime,
                    src_path,
                });
            }
        }
        entries.sort_by(|a, b| a.served_path.cmp(&b.served_path));
        Ok(())
    }
}
