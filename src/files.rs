use {
    crate::*,
    serde::de::DeserializeOwned,
    std::{
        fs::{
            self,
            File,
        },
        io::BufReader,
        path::Path,
    },
};

/// Deserialize an object from a JSON, TOML, or HJSON file.
///
/// # Errors
/// Return `DdError::Read` on file read errors, `DdError::UnsupportedFileFormat`
/// if the file extension is not supported, and deserialization errors
/// as appropriate.
pub fn read_file<T, P: AsRef<Path>>(path: P) -> DdResult<T>
where
    T: DeserializeOwned,
{
    let path = path.as_ref();
    let file = File::open(path).map_err(|error| DdError::Read {
        path: path.to_owned(),
        error,
    })?;
    let reader = BufReader::new(file);
    let obj = match path.extension().and_then(|s| s.to_str()) {
        Some("hjson") => deser_hjson::from_reader(reader)?,
        Some("json") => serde_json::from_reader(reader)?,
        Some("toml") => {
            let toml = std::io::read_to_string(reader)?;
            toml::from_str(&toml)?
        }
        _ => return Err(DdError::UnsupportedFileFormat(path.to_owned())),
    };
    Ok(obj)
}

/// Search direct subdirectories of `parent` for a ddoc project
/// (a directory containing a valid `ddoc.hjson` file).
pub fn project_subdirectory(parent: &Path) -> Option<std::path::PathBuf> {
    let read_dir = std::fs::read_dir(parent).ok()?;
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() && Project::load(&path).is_ok() {
            return Some(path);
        }
    }
    None
}

/// Copy normal non hidden files from `src_dir` to `dst_dir` recursively
pub fn copy_normal_recursive(
    src_dir: &Path,
    dst_dir: &Path,
) -> DdResult<()> {
    if !dst_dir.exists() {
        fs::create_dir_all(dst_dir)?;
    }
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            let sub_src = entry.path();
            let sub_dst = dst_dir.join(entry.file_name());
            copy_normal_recursive(&sub_src, &sub_dst)?;
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
        let dest_path = dst_dir.join(file_name);
        if dest_path.exists() {
            fs::remove_file(&dest_path)?; // to have it updated
        }
        fs::copy(entry.path(), dest_path)?;
    }
    Ok(())
}
