use crate::core::post::Post;
use crate::util::boxed_error;
use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use std::path::{Path, PathBuf};

pub struct Renderer {
    pub registry: Handlebars,
}

impl Renderer {
    pub fn new(theme_path: &PathBuf) -> Self {
        let mut registry = Handlebars::new();
        registry.set_strict_mode(true);
        let mut renderer = Renderer { registry };
        renderer.register_templates(&theme_path);
        renderer
    }

    pub fn rendered_post(&self, post: Post) -> Result<Post, Box<dyn Error>> {
        self.render_post(&post)
            .map(|contents| Post { contents, ..post })
    }

    pub fn render_post(&self, post: &Post) -> Result<String, Box<dyn Error>> {
        dbg!(&post.name);
        self.registry
            .render(
                "post",
                &json!({
                    "content": post.contents,
                    "title": post.title()
                }),
            )
            .map_err(boxed_error)
    }

    pub fn register_templates(&mut self, theme_path: &PathBuf) {
        self.register_template("page", theme_path);
        self.register_template("post", theme_path);
    }

    fn register_template(&mut self, template: &str, theme_path: &PathBuf) {
        self.registry
            .register_template_file(
                template,
                path_with_component(&theme_path, format!("{}.tpl", template).as_str()),
            )
            .expect(format!("Cannot read template: {}", template).as_str());
    }
}

fn path_with_component(path: &Path, component: &str) -> PathBuf {
    path.join(Path::new(component))
}
