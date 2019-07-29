/// Represents a blog post
#[derive(Clone, Debug)]
pub struct Post {
    pub name: String,
    /// HTML contents of the post
    pub contents: String,
}