use {
    crate::*,
    std::{
        borrow::Cow,
        fs,
        path::Path,
    },
};

static TEMPLATE_INIT_HJSON: &str = r#"
# This is a configuration file for the ddoc static site generator.
# For details and instruction, see https://dystroy.org/ddoc/

title: <title>
description: <description>
favicon: null // eg "img/favicon.ico"

// All pages must be listed here
// One of them must be index.md
// You can have submenus, eg:
// pages: {
//     Home: index.md
//     Guide: {
//         "Getting Started": guide/getting_started.md
//         "Advanced Topics": guide/advanced_topics.md
//     }
// }
pages: {
    Home: index.md
}

// Header and footer links can have { img, href, class, label, alt, target}.
// All these fields are optional.
// Hrefs starting with '/' are relative to the site's root (eg '/guide/help.md')
// You can remove any part if you don't want it.
header: {
    before-menu: [ // "before-menu" is the className which will be applied
        // this is a good place for a logo or a link to a wider site
    ]
    middle: menu
    after-menu: [
        {
            img: img/ddoc-left-arrow.svg
            href: --previous
            class: previous-page-link
            alt: Previous Page
        }
        {
            img: img/ddoc-search.svg
            href: --search
            class: search-opener
            alt: Search
        }
        {
            img: img/ddoc-right-arrow.svg
            href: --next
            class: next-page-link
            alt: Next Page
        }
        <github-navlink>
    ]
}
footer: {
}

// UI options
ui: {
    // if true, the generated HTML includes a checkbox which
    // can be styled into a hamburger menu for small screens
    hamburger_checkbox: true
}

"#;
static TEMPLATE_GITHUB_NAVLINK: &str = r#"{
            img: img/github-mark-white.svg
            class: external-nav-link
            alt: GitHub
            href: <url>
        }"#;

/// Initialize a ddoc.hjson file in the specified directory
/// (do nothing if one already exists)
///
/// # Errors
/// Return `DdError::InvalidConfig` if an existing ddoc.hjson
/// cannot be read, or other less likely `DdError` variants on
/// write errors when creating a new ddoc.hjson
pub fn init_hjson_in_dir(
    dir: &Path,
    init_values: &InitValues,
) -> DdResult<Config> {
    let path = dir.join("ddoc.hjson");
    if path.exists() {
        read_file(&path).map_err(|e| {
            error!("Error reading {}: {}", path.display(), e);
            // Return a specific error so that the caller can
            // issue a proper message to the user
            DdError::InvalidConfig
        })
    } else {
        // FIXME we should handle the case of this generation failing
        // to build a parsable hjson (then probably give up with the
        // init values)
        let mut hjson = TEMPLATE_INIT_HJSON.to_owned();
        let title = init_values.title.as_deref().unwrap_or("Unnamed Site");
        let description = init_values.description.as_deref().unwrap_or("");
        let github_navlink = if let Some(github_repo) = &init_values.github_repo {
            TEMPLATE_GITHUB_NAVLINK.replace("<url>", github_repo).into()
        } else {
            Cow::Borrowed("// links here will appear after the menu")
        };

        hjson = hjson
            .replace("<title>", &escape_hjson_string(title))
            .replace("<description>", &escape_hjson_string(description))
            .replace("<github-navlink>", &github_navlink);

        fs::write(&path, hjson)?;
        eprintln!("Created {}", path.display());
        read_file(&path)
    }
}

pub fn escape_hjson_string(s: &str) -> String {
    format!("{:?}", s)
}
