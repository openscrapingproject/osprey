use crate::prelude::*;

use regex::Regex;
use std::collections::HashMap;
use crate::api::Matcher;

/// The RegexMatcherExtractor can act as either a [`Matcher`] to match potential page sets,
/// or a metadata extractor to get information from the request metadata
#[derive(Serialize, Deserialize, Debug)]
pub struct RegexMatcherExtractor {
    #[serde(rename = "type")]
    pub operating_as: Type,
    pub match_mode: Option<MatchMode>,
    pub url: Option<Value>,
    pub version: Option<Value>,
    pub status: Option<Value>,
    pub headers: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Extract {
    pub regex: String,
    pub group: u32,
    pub template: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Value {
    /// ## Matchers
    /// checks whether the value exists
    /// Currently not used because the API garuantees that all metadata is returned
    Exists(bool),
    /// checks whether the value matches the regex
    Regex(String),

    /// ## Extractors
    /// Matches the value against the regex and places it in the template
    RegexExtract(Extract),
}

/// Determines the logical operation to run on more than one matcher
/// Is unsupported for a MetadataExtractor, which should only return one value
#[derive(Serialize, Deserialize, Debug)]
pub enum MatchMode {
    AND,
    OR
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Matcher,
    Extractor
}

impl RegexMatcherExtractor {

    fn match_value(&self, matcher: Option<Value>, input: String, default: bool) -> bool {
        // when no matcher provided, let all through
        if matcher.is_none() {
            return default
        }
        return match matcher.unwrap() {
            Value::Regex(reg) => {
                info!("Trying to match with regex {}", reg);

                let re = Regex::new(reg.as_str());

                match re {
                    Ok(r) => r.is_match(&input.to_string()),
                    Err(e) => {
                        error!("Failed to create regex {}", e);
                        false
                    }
                }
            }
            Value::Exists(base) => {
                true
            }
            Value::RegexExtract(ex) => {
                false
            }
        }
    }
}


#[typetag::serde(name = "regex")]
impl Matcher for RegexMatcherExtractor {
    fn run_match(&self, data: crate::api::MatchData) -> Result<bool> {
        let config = self;

        match config.operating_as {
            Type::Matcher => {
                match config.match_mode.unwrap_or_else(|| Err(anyhow!("no match mode specified")))? {
                    MatchMode::AND => {
                        self.match_value(config.url, data.url.to_string(), true) && self.match_value(config.version, data.version.to_string(), true) && self.match_value(config.status, data.status.to_string(), true) 
                    },
                    MatchMode::OR => {
                        if (config.url.is_none() && config.version.is_none() && config.status.is_none()) {
                            true
                        }
                        self.match_value(config.url, data.url.to_string(), false) || self.match_value(config.version, data.version.to_string(), false) || self.match_value(config.status, data.status.to_string(), false) 
                    }
                }
            }
            Type::Extractor => {
                return Err(anyhow!("configured for wrong mode: metadata extractor"));
            }
        }

        // let reg = config.url.as_str();
        
        // info!("Match for URL {} is {}", data.url, res);
        // TODO match against the headers as well
        Ok(res)
    }
}
