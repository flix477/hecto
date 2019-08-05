use std::time::SystemTime;

pub mod folder;
pub mod folder_entry;
pub mod folders;

/// Represents a blog post
#[derive(Clone, Debug)]
pub struct Post {
    /// Filename of the post
    pub name: String,
    /// File creation date
    pub creation_date: SystemTime,
    /// Data mined from the markdown contents
    pub metadata: PostMetadata,
    /// Markdown contents of the post
    pub contents: String,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            name: String::default(),
            creation_date: SystemTime::now(),
            metadata: PostMetadata::default(),
            contents: String::default()
        }
    }
}

impl Post {
    pub fn title(&self) -> String {
        if let Some(ref title) = self.metadata.title {
            title
        } else {
            &self.name
        }.clone()
    }
}

#[derive(Clone, Debug, Default)]
pub struct PostMetadata {
    pub title: Option<String>,
    pub image: Option<String>,
    pub preview: String
}
