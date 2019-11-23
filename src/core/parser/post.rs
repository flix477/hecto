use crate::core::posts::{Post, PostMetadata};
use crate::util::{boxed_error, os_str_to_string};
use pulldown_cmark::{Event, Parser, Tag};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::fs::Metadata;
use std::cmp::min;

pub fn parse_post(path: &Path, metadata: &Metadata) -> Result<Post, Box<dyn Error>> {
    let name = os_str_to_string(path.file_name().unwrap());
    let creation_date = metadata.created()?;
    let text_contents = fs::read_to_string(path).map_err(boxed_error)?;
    let parser = get_parser(&text_contents);
    let contents = parse_markdown_html(parser.clone());
    let metadata = parse_metadata(parser);

    Ok(Post {
        name,
        creation_date,
        contents,
        metadata
    })
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

// TODO: refactor
fn parse_metadata(parser: Parser) -> PostMetadata {
    let mut metadata = PostMetadata::default();

    let mut title = String::new();
    let mut title_started = false;
    let preview_length = 250;
    let mut text_contents = String::new();

    parser.for_each(|event| match event {
        Event::Start(Tag::Header(1)) => {
            if metadata.title.is_none() {
                title_started = true;
            }
        }
        Event::Text(text) => {
            if title_started {
                title.push_str(&text)
            } else if !title.is_empty() && metadata.preview.len() <= preview_length {
                metadata.preview.push_str(&text[0..min(preview_length, text.len())])
            }

            text_contents.push_str(&text);
            text_contents.push_str(" ");
        }
        Event::End(Tag::Header(1)) => {
            if title_started {
                title_started = false;

                if !title.is_empty() {
                    metadata.title = Some(title.clone());
                }
            }
        },
        Event::Start(Tag::Image(_, url, _)) => {
            if metadata.image.is_none() {
                metadata.image = Some(url.to_string());
            }
        },
        _ => {}
    });

    metadata.reading_time = calculate_reading_time(&text_contents);

    metadata
}

const READING_SPEED: usize = 200;

fn calculate_reading_time(contents: &str) -> usize {
    let words = contents.split(" ").count();
    let reading_time = words as f32 / READING_SPEED as f32;
    reading_time.round() as usize
}