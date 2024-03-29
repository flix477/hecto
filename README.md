# hecto
Super simple Markdown blog server.

<img width="400" alt="Hecto default theme" src="https://user-images.githubusercontent.com/6579510/69485997-af376780-0e14-11ea-84aa-6881ff0d3434.png">

## Features
- Transform a folder of Markdown files to a blog in a single command
- (Somewhat buggy) Live reloading on new posts and theme changes
- Renders LaTeX equations (using KaTeX)

## Usage
**From the directory you want to serve**:
`hecto`

**To specify the directory to serve**:
`hecto /path/to/folder/`

By default, those will serve at http://127.0.0.1:7878

### Arguments
- `--hostname HOSTNAME`: Specify a custom hostname (or `-h`)
- `--port PORT`: Specify a custom port (or `-p`)
- `--theme /path/to/theme/`: Specify a custom theme (or `-t`)

## Themes
Hecto comes with a default theme, but you can easily make one yourself!

Themes are made of three [Handlebars](https://handlebarsjs.com) templates:
- `post.hbs`: The template that blog posts are rendered to
- `folder.hbs`: The template that blog folders are rendered to
- `page.hbs`: The template that contains elements common to both posts and folders, like the header

Check out the default theme for more information!
