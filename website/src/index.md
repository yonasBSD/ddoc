
# ddoc

**Warning: ddoc is very (very very) recent and still considered unstable**

## Introduction

**ddoc** is a markdown based static site generator.

**ddoc** is *much* less powerful than other tools (Hugo, Zola, Mkdocs, etc.) and doesn't include templating or plugins systems.

**ddoc** makes sense when you want a simple site, such as this one, with a site wide navigation menu, a table of content on every page, and you want to be confident the style won't be broken at every release of the tool.

## Why NOT use ddoc

* ddoc assumes you want to write or tune, then own, your CSS, not choose among themes
* ddoc has no templating system - it doesn't suit every need
* ddoc has less features than any other static site generator
* ddoc is very new and might need fixes according to feedback

## Project Goals, and features

* Be a reliable static site generator for documentation sites
* Complete and reasonnable navigation (menu, TOC, links, search)
* Avoid breaks among versions - no imported CSS
* Support images, tables, code sections, links, etc.
* Cross-platform and easy to install - a single binary with no dependencies
* Clean URL paths, no history shenanigans, obvious links
* Generated HTML is semantic and easy to style with CSS
* All internal links are relative, ddoc doesn't need to know the base url of the site

## Possible future goals

* Automated "list" pages - to make ddoc suitable for blogs, albums, etc.
* Image processing
* Syntax highlighting in code

## Getting Started

* [Install ddoc](install)
* [Setup your site](setup)
* [Edit your site](edit)
* [Look at examples](examples)

