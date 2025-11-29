use {
    crate::*,
    serde::de::DeserializeOwned,
    std::{
        fs::File,
        io::BufReader,
        path::Path,
    },
};

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
        _ => return Err(DdError::UnsupportedFileFormat(path.to_owned())),
    };
    Ok(obj)
}
