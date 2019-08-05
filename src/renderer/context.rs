use serde::Serialize;

#[derive(Serialize)]
pub struct Context<T: Serialize> {
    pub site_name: String,
    pub data: T
}