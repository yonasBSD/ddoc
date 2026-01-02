The ddoc.hjson file, at the root of the website folder, describes the site map and the layout of the pages.

It's a [Hjson](https://hjson.github.io/) file.

# Global Properties

Quite explanatory, they're used to build the `<head>` element.

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

## `ddoc-` Elements

