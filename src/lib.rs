pub(crate) mod cli;
mod config;
pub use config::{DiffArgs, DiffConfig, DiffProfile, RequestProfile, ResponseProfile};

#[derive(Debug, Clone)]
pub struct ExtraArgs {
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}
