use {
    crate::*,
    std::path::PathBuf,
};

// FIXME trash

pub struct ServedFile {
    /// path relative to the root of the site
    pub served_path: PathBuf,
    /// path to the file on disk
    pub src_path: PathBuf,
}
