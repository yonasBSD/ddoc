The ddoc.hjson file, at the root of the website folder, describes the site map and the layout of the pages.

It's a [Hjson](https://hjson.github.io/) file.

# Global Properties

Those simple properties used to build the `<head>` element:

```Hjson
title: "ddoc"
description: "Markdown based static site generator"
favicon: img/favicon.ico
```

As for other paths, the one to the favicon is relative to the `src` directory.

# Site Map

This tree lists all the pages of the site, as they'll be listed in the site navigation menu.

Paths and names to the Markdown files will define the path parts of the URL.

For example, here's the site map of the [bacon](https://dystroy.org/bacon) site:

```Hjson
site-map: {
    Overview: index.md
    Config: config.md
    Analyzers: analyzers.md
    Cookbook: cookbook.md
    Community: {
        "Bacon Dev": community/bacon-dev.md
        FAQ: community/FAQ.md
        bacon-ls: community/bacon-ls.md
        nvim-bacon: community/nvim-bacon.md
    }
}
```

And here's a URL to the "Bacon Dev" page: `https://<site-root>/community/bacon-dev/`

The hierarchy of the menu and sub-menus doesn't have to match the one of the paths.

For example all pages of the ddoc documentation site are at the same depth URL-wise but are still grouped in the menus you see at the top.

# Body layout

The `body` configuration part defines the HTML layout.

Its structure is a map of `tag.classes` or `ddoc-element.classes`.

For example, given the following `body` configuration part:

```Hjson
body: {
    header: {
        ddoc-link.search-opener: {
            img: img/search.png
            href: --search
            alt: Search
        }
    }
    div.center: {
        ddoc-main: {}
    }
}
```

The two elements starting with `ddoc-` are special generated parts, while the `body`, `header`, and `div.center` are standard HTML elements.

The generated `<body>` of a page will look like this:

```HTML
<body>
    <header>
        <a href="javascript:ddoc_search.open();" class="nav-link  search-opener">
            <img src="img/search.png" alt="Search">
        </a>
    </header>
    <div class="center">
        <main>
        Here the HTML built from the content of the page's Markdown file
        </main>
    </div>
</body>
```

The `body` of the default ddoc.hjson file contains more parts than this example, so that pages have a menu, a table of content, etc.

## text/html content of an element

If you don't put sub elements in an element, you can put text or HTML instead, eg

```Hjson
p.help-hint: {
    text: hit 'h' to display the help
}
```

or

```Hjson
p.help-hint: {
    html: "hit <span style=\"color: red;\">h</span> to display the help"
}
```

## Special text expansions

Just like some `--` prefixed strings can be used in any link (see [link expansions](/edit#expansions)), there are some expansions for text:

* `--previous-page-title`: title of the previous page according to the menu
* `--next-page-title`: title of the next page
* `--current-page-title`: title of the current page

Example:

```Hjson
div.page-title: {
    text: --current-page-title
}
```


# `ddoc-` Elements

Those are special generated elements that you can define in the `body` configuration part.

## ddoc-link

This generates a `<a>` element with either a label, an image, etc. according to attributes:

| Attribute | Meaning |
|:-:|:-
|img|the src of an image to display in the `<a>` element
|alt|the alt text displayed on hovering the image, if any|
|inline| link to a SVG file to inline (inlining your svg instead of using a `<img>` element allows for greater CSS or JS control)|
|label|the label (if you provide both an image and a label, the order of attributes decides which comes first)
|href|content of the `href` attribute of the `<a>` element (recomputed depending on the page if a link to another page or a served resource - see [links](/edit#links))
|target|target attribute to set on the `<a>` element

## ddoc-menu

This includes the `<nav.site-nav>` element allowing site navigation, displayed as a menu, foldable or not, depending on your stylesheets.

To ease presentation as a no-javascript hamburger menu, a checkbox can be optionally included:

```Hjson
body: {
    header: {
        ddoc-menu: {
            hamburger-checkbox: true
        }
    }
    ddoc-main: {}
}
```

## ddoc-toc

The Table-Of-Content starts with the page title then contains, in a `<ul>` list, `<li><a>` links to `<h1>` to `<h4>` titles.

Example:

```Hjson
article: {
    aside.page-nav: {
        ddoc-toc: {
            activate-visible-item: true
        }
    }
    ddoc-main: {}
}
```

### Active TOC item

When `activate-visible-item` is `true`, a script is injected in the page so that the TOC follows scroll and selection.

If you don't want this feature, or if you want to use your own script for that, set this property to `false`.

### TOC depth

You usually don't want to show all levels in your TOC, or not on all pages. CSS should be used to define what's shown.

For example to hide levels `<h3>` and `<h4>` on the `index` page, add this in your CSS:

```CSS
body.page-index nav.page-toc .h3, body.page-index nav.page-toc .h4 {
	display: none;
}
```

Note: the default CSS has those `display:none` for `<h3>` and `<h4>`, you may remove those lines if you want to show all levels in the TOC.

## ddoc-main

This is the HTML generated from the Markdown's file of the page.


