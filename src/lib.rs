pub mod cli;
mod config;
mod req;
pub mod utils;
pub use config::{DiffConfig, DiffProfile, ResponseProfile};
pub use req::RequestProfile;

#[derive(Debug, Clone)]
pub struct ExtraArgs {
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}
