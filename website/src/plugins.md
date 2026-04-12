
The first thing you should decide when starting a new site is whether

- to start from one of the default theme
- to import a theme plugin from elsewhere
- to create your own style in a new theming plugin
- or simply to write CSS files in `src/css`

# What's a plugin

A plugin looks a lot like a whole site: it contains

- a `ddoc.hjson` file
- a `src` folder

The content of a plugin is merged with the main files:

- properties and variables are added
- css and js files are added
- elements of the dom tree are merged

When a property is present both in a plugin and in the main tree, when a css or js file, or an image, is present both in main and in a plugin, the version in main is chosen.

Said otherwise: a file or property of a plugin can be overriden in main (or in a plugin which appears before in the list of enabled plugins).

# Embedded Plugins

Some plugins are embedded in the ddoc binary and are automatically added to any new site (but most of them are not enabled):

- 2 themes: **theme-columns** and **theme-top-menu**
- **search**: brings the search feature
- **toc-activate**: update the Table Of Content on scroll

# Enable a plugin

For a plugin to be enabled

- its must be present in the `plugins` directory
- it must be listed in the `plugins` section of the main `ddoc.hjson` file

For example here the only active plugin is `theme-columns`.

```Hjson
active-plugins: [
    // "theme-top-menu"
    "theme-columns"
]
```

# How to deal with plugins

A plugin is part of your site, you should put it in version control to ensure you'll be able to restore the site exactly as it was.

A theming plugin is usually just a starting point: it's unlikely you want your site to be exactly as the plugin makes it.

## Override a plugin

Most often, when a plugin globally suits you but you want some simple changes, you don't have to change the plugin itself:
- to insert new DOM elements, you may add then in your main `ddoc.hjson` file
- to override a CSS variable, you can just define it in a CSS file in your main `src/css` directory
- to override a style, you may also define a CSS rule - if the selectivity is the same as the plugin's one, your rule has priority (being added later in the `<head>`)
- to completely override a file (an image, a whole CSS file, a JS file), just put a file with same name in the main `src`

## Modify the plugin itself

But you're also free to modify the plugin, this may be the simplest solution if you want to "own" the style, just change it (if you share the change, add a comment inside so that nobody's confused and think it's the original one).

You can modify either the `ddoc.hjson` file or the various files it contains.

If you don't think you'll revert back to the official version, it's better to rename the plugin (the folder and the entry in the `active-plugins` list).

# Plugins as modules

Even if you don't plan to reuse a style, it makes sense to specify the styling of your site in one or more plugins:

- it's easier to switch between plugins to experiment with several styles
- you may decide to have optional parts (eg some blocks defined with both css and js)
- you may organize your code in separate concerns

# Reset a default plugin

Default plugins are the ones which come with the `ddoc` binary.

They're written in the `plugins` directory when you `init` a new site.

They can also be restored to the binary state when you don't want a change you made, or if you use the vanilla plugin but want the version coming with a new version of `ddod`.

The `--init-plugin` directory writes or resets a plugin in the `plugins` directory, but asks for confirmation if the plugin is already present.
