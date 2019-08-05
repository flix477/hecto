use crate::core::posts::Post;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct PostView {
    pub title: String,
    pub contents: String,
    pub creation_date: String
}

impl PostView {
    pub fn new(post: &Post) -> Self {
        let creation_date: DateTime<Utc> = post.creation_date.into();
        let creation_date = creation_date.format("%A %B %d %Y, %H:%M").to_string();

        Self {
            title: post.title(),
            contents: post.contents.clone(),
            creation_date
        }
    }
}