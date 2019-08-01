use crate::core::folder::{Folder, FolderEntry};
use crate::core::parser::post::parse_post;
use crate::core::post::Post;
use crate::renderer::Renderer;
use crate::util::{boxed_error, os_str_to_string};
use std::error::Error;
use std::fs::Metadata;
use std::path::Path;

pub mod post;

fn parse_path(
    path: &Path,
    renderer: &Renderer,
) -> Result<Option<FolderEntry<Post, Folder>>, Box<dyn Error>> {
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
        .map(|result| result.map_err(boxed_error))
        .map(|result| result.and_then(|entry| parse_path(&entry.path(), renderer)))
        .flatten()
        .flatten()
        .collect();

    Ok(Folder {
        name: os_str_to_string(path.file_name().unwrap()),
        entries,
    })
}

fn is_markdown_file(path: &Path, metadata: &Metadata) -> bool {
    if let Some(extension) = path.extension() {
        extension == "md" && metadata.is_file()
    } else {
        false
    }
}
