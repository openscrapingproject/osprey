use super::http;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type HeaderMap = HashMap<String, String>;

/// Represents an HTTP response in a somewhat standard format
/// that can be serialized.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub url: http::URL,
    pub status: http::StatusCode,
    pub version: http::Version,
    pub headers: HeaderMap,
    pub body: String,
}

/// Exactly the same as [Response], without the body.
/// This could allow for an optimization: do a HEAD request first,
/// then run matcher. However, we assume that most URLs
/// that the user provides to us will match at least one matcher.
/// Thus for now that optimization is eh.
#[derive(Debug, Serialize, Deserialize)]
pub struct MatchData {
    // TODO: make these all Into traits so we can pass an &String, etc
    pub url: http::URL,
    pub status: http::StatusCode,
    pub version: http::Version,
    pub headers: HeaderMap,
}
