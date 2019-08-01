/// Represents a blog post
#[derive(Clone, Debug, Default)]
pub struct Post {
    pub name: String,
    pub metadata: PostMetadata,
    /// HTML contents of the post
    pub contents: String,
    /// Markdown contents of the post
    pub source: String,
}

impl Post {
    pub fn title(&self) -> String {
        if let Some(title) = &self.metadata.title {
            title
        } else {
            &self.name
        }
        .clone()
    }
}

#[derive(Clone, Debug, Default)]
pub struct PostMetadata {
    pub title: Option<String>,
    pub image: Option<String>,
}
