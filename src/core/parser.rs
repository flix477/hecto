use std::path::Path;
use std::error::Error;
use std::fs::Metadata;
use crate::core::folder::{FolderEntry, Folder};
use crate::core::post::Post;
use crate::renderer::Renderer;

fn parse_path(path: &Path, renderer: &Renderer) -> Result<Option<FolderEntry>, Box<dyn Error>> {
    let metadata = std::fs::metadata(path)?;
    if metadata.is_dir() {
        Ok(Some(FolderEntry::Folder(parse_folder(path, renderer)?)))
    } else if is_markdown_file(path, &metadata) {
        Ok(Some(FolderEntry::Post(parse_post(path, renderer)?)))
    } else {
        Ok(None)
    }
}

pub fn parse_folder(path: &Path, renderer: &Renderer) -> Result<Folder, Box<dyn Error>> {
    let entries = std::fs::read_dir(path)?
        .map(|result| result.map_err(|error| Box::new(error).into()))
        .map(|result| result.and_then(|entry| parse_path(&entry.path(), renderer)))
        .flatten()
        .flatten()
        .collect();

    Ok(Folder {
        name: path.file_name().unwrap().to_string_lossy().parse().unwrap(),
        entries
    })
}

fn parse_post(path: &Path, renderer: &Renderer) -> Result<Post, Box<dyn Error>> {
    let contents = read_file(path)
        .and_then(parse_markdown)
        .and_then(|contents| renderer.render_post(contents))?;

    Ok(Post {
        name: path.file_name().unwrap().to_string_lossy().parse().unwrap(),
        contents
    })
}

fn read_file(path: &Path) -> Result<String, Box<dyn Error>> {
    std::fs::read_to_string(path).map_err(|error| Box::new(error).into())
}

fn parse_markdown(contents: String) -> Result<String, Box<dyn Error>> {
    let options = pulldown_cmark::Options::empty();
    let parser = pulldown_cmark::Parser::new_ext(contents.as_str(), options);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    Ok(html)
}

fn is_markdown_file(path: &Path, metadata: &Metadata) -> bool {
    if let Some(extension) = path.extension() {
        extension == "md" && metadata.is_file()
    } else {
        false
    }
}