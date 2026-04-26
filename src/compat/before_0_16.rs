use {
    crate::*,
    std::fmt::Write,
};

static SEARCH_PLUGIN: &str = "search";
static SEARCH_JS_FILE: &str = "ddoc-search.js";
static TOC_ACTIVATE_PLUGIN: &str = "toc-activate";
static TOC_ACTIVATE_JS_FILE: &str = "ddoc-toc-activate.js";

fn compat_allowed(config: &Config) -> bool {
    !config.has_any_plugin()
}
// compatibility with old ddoc pre-plugin system
fn config_needs_search_script(config: &Config) -> bool {
    config.body.has_href("--search")
}
// compatibility with old ddoc pre-plugin system
fn config_needs_toc_activate_script(config: &Config) -> bool {
    config.body.has(&mut |element: &Element| {
        if let ElementContent::Toc(toc) = &element.content {
            return toc.activate_visible_item;
        }
        false
    })
}

pub fn write_special_js_files_if_needed(
    config: &Config,
    project: &Project,
) -> DdResult<()> {
    if compat_allowed(config) {
        // there's no plugin, it's probablely an old project (0.16-)
        if config_needs_search_script(config) {
            let bytes = resource_file_bytes(SEARCH_PLUGIN, "js", SEARCH_JS_FILE)?;
            project.add_js_to_build(SEARCH_JS_FILE, &bytes)?;
        }
        if config_needs_toc_activate_script(config) {
            let bytes = resource_file_bytes(TOC_ACTIVATE_PLUGIN, "js", TOC_ACTIVATE_JS_FILE)?;
            project.add_js_to_build(TOC_ACTIVATE_JS_FILE, &bytes)?;
        }
    }
    Ok(())
}
pub fn write_special_js_headers_if_needed(
    page_path: &PagePath,
    config: &Config,
    project: &Project,
    html: &mut String,
) -> DdResult<()> {
    if compat_allowed(config) {
        if config_needs_search_script(config) {
            let url = format!("js/{}", SEARCH_JS_FILE);
            let url = project.static_url(&url, page_path);
            writeln!(html, r#"<script src="{}" defer></script>"#, url)?;
        }
        if config_needs_toc_activate_script(config) {
            let url = format!("js/{}", TOC_ACTIVATE_JS_FILE);
            let url = project.static_url(&url, page_path);
            writeln!(html, r#"<script src="{}" defer></script>"#, url)?;
        }
    }
    Ok(())
}

pub fn expand_special_var(
    var_name: &str,
    config: &Config,
) -> Option<String> {
    if compat_allowed(config) {
        if var_name == "search" {
            return Some("javascript:ddoc_search.open();".to_string());
        }
    }
    None
}
