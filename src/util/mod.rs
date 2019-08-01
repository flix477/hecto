use std::error::Error;
use std::path::{Path, Component, PathBuf};
use std::ffi::OsStr;

pub fn boxed_error<T: 'static + Error>(error: T) -> Box<dyn Error> {
    Box::new(error).into()
}
pub fn first_component(path: &Path) -> String {
    component_as_string(
        path
            .components()
            .next()
            .unwrap()
    )
}

pub fn component_as_string(component: Component) -> String {
    os_str_to_string(component.as_os_str())
}

pub fn os_str_to_string(os_str: &OsStr) -> String {
    os_str
        .to_string_lossy()
        .parse()
        .unwrap()
}

pub fn relative_path(path: &Path, root_path: &Path) -> PathBuf {
    path.components().skip(root_path.components().count()).collect::<PathBuf>()
}