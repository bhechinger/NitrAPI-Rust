//! Get game formats (e.g.: Standard, Modern, Onslaught Block).
#![allow(dead_code)]
use crate::query_builder;
use reqwest::StatusCode;
use serde::Deserialize;

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct Maintenance {
    pub cloud_backend: bool,
    pub domain_backend: bool,
    pub dns_backend: bool,
    pub pmacct_backend: bool
}

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct Data {
    pub maintenance: Maintenance
}

#[doc(hidden)]
#[derive(Clone, Debug, Deserialize)]
pub struct Maint {
    pub status: String,
    pub data: Data,
}

pub async fn all() -> Result<Maint, StatusCode> {
    let maint: Result<Maint, StatusCode> = query_builder::all("maintenance").await;

    match maint {
        Ok(t) => Ok(t),
        Err(e) => Err(e),
    }
}
