
# Create Pages

Each page is backed by a markdown file in the main `/src` directory.

You may organize your files in subdirectories but you don't have to, the tree of files doesn't have to match the menu tree.

Each file, to be served, must be referenced in the `pages` section of the `ddoc.hjson` config file.

Standard features of CommonMark are available, including links, images, tables, etc.

# Add CSS files and JS files

Any file matching `/src/css/*.css` or `/src/js/*.js`, either directly or in a plugin, will be served (with the nuance that when a file is present with the same name in several plugins or in main, only the one of main or the latest in the plugins list, will be served).

Files are included in the HTML's head element in alphabetic order.

The body element holds a page-specific class to help discriminate rules.
For example, on this site the `<h1>` elements of the [examples](../examples) page have a top border, which is defined with a rule on `body.page-examples h1`.

# Include images

Images have to be stored in `/src/img` (in main or in a plugin).

They can be included  in markdown using standard syntax:

```markdown
![Alt text](img/my_image.png)
```

You'll also have references to images in your ddoc.hjson config file, for example in menu
links.

In both cases, ddoc will rewrite the image URL to point to the correct location whatever
the depth of the page including the image. Your reference just has to start with `img/`.

# More than just Markdown

When Markdown isn't enough, you can insert HTML in your md file, add dedicated styling or scripts.

For example, the small strip of images in the [Overview](..) page is a small `<div class=image-strip>...` insert relying on dedicated `css/image-strip.css` and `js/image-strip.js` files.

# Links

## Links to pages

In your markdown or in the ddoc.hjson config file, you can refer to other pages, or locations
in other pages, using either relative or absolute links.

A relative link is like `../other_page#some_section`.

An absolute link is like `/path/to/other_page#some_section`.

The leading `/` in absolute links refers to the root of your documentation site and the URL
will be rewritten to be relative to the current page depending on its depth.

## Expansions

Some special values are dynamically expanded:

- `--previous-page` links to the previous page according to the order defined by the pages list
- `--next-page` links to the next page
- `--page-title` is the title of the page while `--title` is the title of the whole site

Other variable values are taken in the `vars` section of a `ddoc.hjson` (the main one having priority over the plugin ones).

## Javascript call

There's no problem calling a javascript function of one of your scripts from a link.

For example, you can have this in the `body` of your `ddoc.hjson`:

```Hjson
ddoc-link.bipper: {
    label: ping
    href: "javascript:alert('test');"
}
```
