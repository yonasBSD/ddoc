[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/ddoc.svg
[l1]: https://crates.io/crates/ddoc

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/ddoc/badge.svg
[l3]: https://docs.rs/ddoc/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3


# ddoc

<!-- cradoc start -->

**Warning: ddoc isn't ready for use yet - don't even try**

## Introduction

**ddoc** is a powerless static site generator specialized for documentation sites,

Examples:

* [Bacon documentation](https://dystroy.org/bacon/) is rendered from these [sources](https://github.com/Canop/bacon/tree/main/website)

## Good reasons NOT to use ddoc

* ddoc assumes you want to write CSS, not choose among themes
* ddoc has less features than any other static site generator
* this tool is super very new and not battle tested

## Project Goals

* Be a reliable static site generator for documentation sites
* Complete and reasonnable navigation (menu, TOC, links)
* Avoid breaks among versions - no imported CSS or layout related HTML
* Support images, tables, code sections, links, etc.
* Cross-platform and easy to install - a single binary with no dependencies
* Clean URL paths, no history shenanigans, obvious links
* Work without JS (but you can add your own JS if you want to)

## Project Non Goals

* Be as generic as zola, mkdocs, hugo, etc. and try to replace them
* Templating - **ddoc probably can't do what you need**
* Theming system - you provide your own CSS

## Possible future goals

* Search
* Automated "list" pages - to make ddoc suitable for blogs, albums, etc.
* Image processing
* Syntax highlighting in code

## Features

* Generated HTML is semantic and easy to style with CSS
* All internal links are relative, ddoc doesn't need to know the base url of the site
* No hidden CSS or JS is injected, only yours
* No templating - everything is built from your markdown, static files, and the ddoc.hjson config

## Usage

*to be written*

## Include images

Images can be included in markdown using standard syntax:
```markdown
![Alt text](img/my_image.png)
```

You'll also have references to images in your ddoc.hjson config file, for example in menu
links.

In both cases, ddoc will rewrite the image URL to point to the correct location whatever
the depth of the page including the image. Your reference just has to start with `img/`.

## Internal links

In your markdown or in the ddoc.hjson config file, you can refer to other pages, or locations
in other pages, using either relative or absolute links.

A relative link is like `../other_page.md#some_section` (the `.md` part is optional).

An absolute link is like `/path/to/other_page#some_section`.

The leading `/` in absolute links refers to the root of your documentation site and the URL
will be rewritten to be relative to the current page depending on its depth.
<!-- cradoc end -->




