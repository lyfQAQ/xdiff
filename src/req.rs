use std::{process::Output, str::FromStr};

use crate::{ExtraArgs, ResponseProfile};
use anyhow::{anyhow, Result};
use http::{header, HeaderValue};
use reqwest::{
    header::{HeaderMap, HeaderName},
    Client, Method, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

#[derive(Debug)]
pub struct ResponseExt(Response);

impl ResponseExt {
    pub async fn filter_text(self, profile: &ResponseProfile) -> Result<String> {
        let mut output = String::new();
        let headers = self.0.headers().clone();
        output.push_str(&format!("{:?} {}\r", self.0.version(), self.0.status()));
        for header in headers.iter() {
            if !profile.skip_headers.contains(&header.0.to_string()) {
                output.push_str(&format!("{}: {}\r", header.0, header.1.to_str()?))
            }
        }

        output.push('\n');
        let text = self.0.text().await?;
        let content_type = get_content_type(&headers);

        match content_type {
            Some("application/json") => {
                let text = Self::filter_json(&text, &profile.skip_body)?;
                output.push_str(&text);
            }
            _ => {
                output.push_str(&text);
            }
        }

        Ok(output)
    }

    fn filter_json(text: &str, skip: &[String]) -> Result<String> {
        let mut json: serde_json::Value = serde_json::from_str(text)?;

        match json {
            serde_json::Value::Object(ref mut obj) => {
                for k in skip {
                    obj.remove(k);
                }
            }
            _ => todo!(),
        }
        Ok(serde_json::to_string_pretty(&json)?)
    }
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

impl RequestProfile {
    pub async fn send(&self, args: &ExtraArgs) -> Result<ResponseExt> {
        let (headers, query, body) = self.generate(args)?;
        let client = Client::new();
        let req = client
            .request(self.method.clone(), self.url.clone())
            .query(&query)
            .body(body)
            .headers(headers)
            .build()?;

        let res = client.execute(req).await?;
        Ok(ResponseExt(res))
    }

    fn generate(&self, args: &ExtraArgs) -> Result<(HeaderMap, serde_json::Value, String)> {
        let mut headers = self.headers.clone();
        let mut query = self.params.clone().unwrap_or_else(|| json!({}));
        let mut body = self.params.clone().unwrap_or_else(|| json!({}));

        for (k, v) in &args.headers {
            headers.insert(HeaderName::from_str(k.as_str())?, v.parse()?);
        }

        if !headers.contains_key(header::CONTENT_TYPE) {
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
        }

        for (k, v) in &args.query {
            query[k] = v.parse()?;
        }

        for (k, v) in &args.body {
            body[k] = v.parse()?;
        }

        let content_type = get_content_type(&headers);

        match content_type {
            Some("application/json") => {
                let body = serde_json::to_string(&body)?;
                Ok((headers, query, body))
            }
            Some("application/x-www-form-urlencoded" | "multipart/form-data") => {
                let body = serde_urlencoded::to_string(&body)?;
                Ok((headers, query, body))
            }
            _ => Err(anyhow!("unsupported content-type")),
        }
    }
}

fn get_content_type(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().split(":").next())
        .flatten()
}
