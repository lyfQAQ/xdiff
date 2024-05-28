use anyhow::Result;
use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;
use url::Url;
/*
    yaml字段结构体的定义
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DiffProfile {
    pub request1: RequestProfile,
    pub request2: RequestProfile,
    pub response: ResponseProfile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    pub url: Url,
    // https://serde.rs/attr-skip-serializing.html
    #[serde(skip_serializing_if = "Option::is_none", default /*不设置函数，使用std中的Default，= "default_params"*/)]
    pub params: Option<serde_json::Value>,
    #[serde(
        with = "http_serde::header_map",
        skip_serializing_if = "HeaderMap::is_empty",
        default
    )]
    pub headers: HeaderMap,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

impl DiffConfig {
    pub async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }
    pub fn from_yaml(content: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }
    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profiles.get(name)
    }
}

impl DiffProfile {
    pub async fn diff(&self, args: DiffArgs) -> Result<String> {
        // let res1 = req1.send(&args).await?;
        // let res2 = req2.send(args).await?;
        // let text1 = res1.filter_text(&self.response).await?;
        // let text2 = res2.filter_text(&self.response).await?;
        todo!()
    }
}
