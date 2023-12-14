//! Get game formats (e.g.: Standard, Modern, Onslaught Block).
#![allow(dead_code)]
use crate::query_builder;
use reqwest::StatusCode;
use serde::Deserialize;

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct Version {
    pub status: String,
    pub message: String,
}

pub async fn all() -> Result<Version, StatusCode> {
    let version: Result<Version, StatusCode> = query_builder::all("version").await;

    match version {
        Ok(t) => Ok(t),
        Err(e) => Err(e),
    }
}
