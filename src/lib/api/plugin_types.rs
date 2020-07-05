use super::http;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type HeaderMap = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: http::StatusCode,
    pub version: http::Version,
    pub headers: HeaderMap,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchData {
    pub url: http::URL,
    pub headers: HeaderMap,
    pub status: http::StatusCode,
}
