use handlebars::Handlebars;
use std::path::{Path, PathBuf};
use serde_json::json;
use std::error::Error;

pub struct Renderer {
    pub registry: Handlebars
}

impl Renderer {
    pub fn new(theme_path: &PathBuf) -> Self {
        let registry = Handlebars::new();
        let mut renderer = Renderer { registry };
        renderer.register_template("post", theme_path);
        renderer
    }

    fn register_template(&mut self, template: &str, theme_path: &PathBuf) {
        dbg!(path_with_component(&theme_path, template));
        self.registry.register_template_file(
            template,
            path_with_component(&theme_path, format!("{}.tpl", template).as_str())
        ).expect(format!("Cannot read template: {}", template).as_str());
    }

    pub fn render_post(&self, post_html: String) -> Result<String, Box<dyn Error>> {
        self.registry.render("post", &json!({"body": post_html}))
            .map_err(|error| Box::new(error).into())
    }
}

fn path_with_component(path: &Path, component: &str) -> PathBuf {
    path.join(Path::new(component))
}