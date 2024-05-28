use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

use crate::ExtraArgs;

/// Diff two http requests and compare the difference of the responses
#[derive(Debug, Parser)]
#[clap(version, author, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
#[non_exhaustive]
pub enum Action {
    /// Diff two API responses based on given profile
    Run(RunArgs),
}

#[derive(Debug, Parser)]
pub struct RunArgs {
    /// profile name
    #[clap(short, long, value_parser)]
    pub profile: String,
    /// Overrides args. Could be used to override the query, headers and body of the request
    /// For query params, use `-e key=val`
    /// For headers, use `-e %key=val`
    /// For body, use `-e @key=val`
    #[clap(short, long, value_parser = parse_key_val, number_of_values = 1)]
    pub extra_params: Vec<KeyVal>,

    /// Configuration to use
    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}

#[derive(Debug, Clone)]
pub enum KeyValType {
    Query,
    Header,
    Body,
}

#[derive(Debug, Clone)]
pub struct KeyVal {
    key_type: KeyValType,
    key: String,
    value: String,
}

/// 解析命令行输入的 key value
fn parse_key_val(s: &str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, '=');
    let retrieve = |v: Option<&str>| -> Result<String> {
        Ok(v.ok_or_else(|| anyhow!("Invalid key value pair: {}", s))?
            .trim()
            .to_string())
    };
    let key = retrieve(parts.next())?;
    let value = retrieve(parts.next())?;

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, key.chars().skip(1).collect::<String>()),
        Some('@') => (KeyValType::Body, key.chars().skip(1).collect::<String>()),
        Some(v) if v.is_ascii_alphabetic() => (KeyValType::Query, key),
        _ => return Err(anyhow!("Invalid key value pair: {}", s)),
    };

    Ok(KeyVal {
        key_type,
        key,
        value,
    })
}

impl From<Vec<KeyVal>> for ExtraArgs {
    fn from(args: Vec<KeyVal>) -> Self {
        let mut headers = vec![];
        let mut query = vec![];
        let mut body = vec![];
        for arg in args {
            match arg.key_type {
                KeyValType::Body => body.push((arg.key, arg.value)),
                KeyValType::Header => headers.push((arg.key, arg.value)),
                KeyValType::Query => query.push((arg.key, arg.value)),
            }
        }
        Self {
            headers,
            query,
            body,
        }
    }
}
