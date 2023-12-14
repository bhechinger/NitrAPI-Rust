//! Get game formats (e.g.: Standard, Modern, Onslaught Block).
#![allow(dead_code)]
use crate::query_builder;
use reqwest::StatusCode;
use serde::Deserialize;

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct Ping {
    pub status: String,
    pub message: String,
}

pub async fn all() -> Result<String, StatusCode> {
    let ping: Result<Ping, StatusCode> = query_builder::all("ping").await;

    match ping {
        Ok(t) => Ok(format!("{}: {}", t.status, t.message)),
        Err(e) => Err(e),
    }
}
