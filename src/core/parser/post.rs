use crate::core::posts::{Post, PostMetadata};
use crate::util::{boxed_error, os_str_to_string};
use pulldown_cmark::{Event, Parser, Tag};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::fs::Metadata;

pub fn parse_post(path: &Path, metadata: &Metadata) -> Result<Post, Box<dyn Error>> {
    initial_post(path, metadata).map(parse_markdown)
}

fn initial_post(path: &Path, metadata: &Metadata) -> Result<Post, Box<dyn Error>> {
    Ok(Post {
        name: os_str_to_string(path.file_name().unwrap()),
        creation_date: metadata.created()?,
        contents: fs::read_to_string(path).map_err(boxed_error)?,
        ..Post::default()
    })
}

fn parse_markdown(post: Post) -> Post {
    let parser = get_parser(post.contents.as_str());
    let contents = parse_markdown_html(parser.clone());
    let metadata = parse_metadata(parser);

    Post {
        contents,
        metadata,
        ..post
    }
}

fn get_parser(contents: &str) -> Parser {
    let options = pulldown_cmark::Options::empty();
    pulldown_cmark::Parser::new_ext(contents, options)
}

fn parse_markdown_html(parser: Parser) -> String {
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}

fn parse_metadata(parser: Parser) -> PostMetadata {
    let mut metadata = PostMetadata {
        title: None,
        image: None,
    };

    let mut title = String::new();
    let mut title_started = false;

    parser.for_each(|event| match event {
        Event::Start(Tag::Header(1)) => {
            if metadata.title.is_none() {
                title_started = true;
            }
        }
        Event::Text(text) => {
            if title_started {
                title.push_str(&text)
            }
        }
        Event::End(Tag::Header(1)) => {
            if title_started {
                title_started = false;

                if !title.is_empty() {
                    metadata.title = Some(title.clone());
                }
            }
        }
        _ => {}
    });

    metadata
}
