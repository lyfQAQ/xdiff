use crate::{ExtraArgs, RequestProfile};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;
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
    pub async fn diff(&self, _args: ExtraArgs) -> Result<String> {
        let res1 = self.request1.send(&_args).await?;
        let res2 = self.request2.send(&_args).await?;

        let text1 = res2.filter_text(&self.response).await?;
        // let text2 = res2.filter_text(&self.response).await?;
        println!("{}", text1);
        Ok("".to_string())
    }
}
