use crate::prelude::*;

use regex::Regex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RegexMatcher {
    pub url: String,
    pub headers: HashMap<String, String>,
}

#[typetag::serde(name = "regex")]
impl crate::api::Matcher for RegexMatcher {
    fn run_match(&self, data: crate::api::MatchData) -> Result<bool> {
        let config = self;

        let reg = config.url.as_str();
        info!("Trying to match with regex {}", reg);

        let re = Regex::new(reg)?;

        let res = re.is_match(&data.url.to_string());
        info!("Match for URL {} is {}", data.url, res);
        // TODO match against the headers as well
        Ok(res)
    }
}
