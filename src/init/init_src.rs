use {
    crate::*,
    std::{
        fs,
        path::Path,
    },
};

/// Create and fill the `src/` directory in the given `dir`.
///
/// Break nothing, and don't write files if they don't look
/// necessary
pub fn init_src_in_dir(
    dir: &Path,
    init_values: &InitValues,
    _config: &Config,
) -> DdResult<()> {
    // src dir
    let src_dir = dir.join("src");
    if !src_dir.exists() {
        fs::create_dir_all(&src_dir)?;
    }

    // src/index.md
    let index_md_path = src_dir.join("index.md");
    if !index_md_path.exists() {
        if let Some(index_path) = &init_values.index {
            fs::copy(index_path, &index_md_path)?;
            eprintln!(
                "Created {} from {}",
                index_md_path.display(),
                index_path.display()
            );
        } else {
            // create a default index.md
            fs::write(
                &index_md_path,
                "# Welcome\n\n\
                This is the index page. You can edit the `index.md` file to add your own content.\n\n\
                You can also add pages, edit CSS files, etc..\n\n\
                # More\n\n\
                A look at the `ddoc.hjson` file should introduce some more concepts.\n\n\
                Then, you'll find complete information at the [documentation](https://dystroy.org/ddoc/edit/)\n\n",
            )?;
            eprintln!("Created {}", index_md_path.display());
        }
    }

    // src/css/
    let css_dir = src_dir.join("css");
    if !css_dir.exists() {
        fs::create_dir_all(&css_dir)?;
    }
    if !has_css(&css_dir)? {
        fs::write(css_dir.join("main.css"), MAIN_CSS_BYTES)?;
    }

    // src/img/
    let img_dir = src_dir.join("img");
    if !img_dir.exists() {
        fs::create_dir_all(&img_dir)?;
    }

    Ok(())
}

fn has_css(css_dir: &Path) -> DdResult<bool> {
    for entry in fs::read_dir(css_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("css") {
            return Ok(true);
        }
    }
    Ok(false)
}
