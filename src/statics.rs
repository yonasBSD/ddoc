use {
    crate::*,
    std::{
        fs,
        path::Path,
        time::SystemTime,
    },
};

/// Implicit reference to one of the files in `/src/js` or `/src/css`
pub struct StaticEntry {
    pub filename: String,
    pub mtime: u64, // secs since UNIX_EPOCH
}

impl StaticEntry {
    pub fn list_in(
        dir: &Path,
        ext_filter: Option<&str>,
    ) -> DdResult<Vec<Self>> {
        let mut entries = Vec::new();
        if dir.exists() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let file_type = entry.file_type()?;
                if !file_type.is_file() {
                    continue;
                }
                let file_name = entry.file_name();
                let Some(file_name) = file_name.to_str() else {
                    continue;
                };
                if file_name.starts_with(".") {
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
                entries.push(Self {
                    filename: file_name.to_string(),
                    mtime,
                });
            }
            entries.sort_by(|a, b| a.filename.cmp(&b.filename));
        }
        Ok(entries)
    }
}
