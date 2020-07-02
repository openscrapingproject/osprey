pub mod regex;


pub struct MatchData {
    pub url: reqwest::Url,
    pub headers: http::HeaderMap,
}
