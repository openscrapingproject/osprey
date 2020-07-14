use anyhow::Result;
use log::warn;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;

pub fn headers2hash(
    headers: &HeaderMap<HeaderValue>,
) -> HashMap<String, String> {
    let mut header_hashmap = HashMap::new();
    for (k, v) in headers {
        let k = k.as_str().to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_hashmap.entry(k).or_insert_with(Vec::new).push(v)
    }
    // TODO: can we remove the Vec<String> intermediary? Are there cases where
    // this actually happens?
    let mut hm = HashMap::new();
    for (k, v) in header_hashmap {
        if v.len() > 1 {
            warn!("Your headers have more than one chunk. That's weird")
        }
        hm.insert(k, v.join(""));
    }
    hm
}

pub fn hash2headers(
    hashmap: &HashMap<String, String>,
) -> Result<HeaderMap<HeaderValue>> {
    let mut h = HeaderMap::new();
    for (k, v) in hashmap {
        h.insert(HeaderName::from_bytes(k.as_bytes())?, v.parse().unwrap());
    }
    Ok(h)
}

#[macro_export]
macro_rules! map {
    ($($key: expr => $value:expr);*) => {{
        let mut hm = std::collections::HashMap::new();
        $( hm.insert($key, $value); )*
        hm
    }};
    (String, $($key: expr => $value:expr);*) => {{
        let mut hm = std::collections::HashMap::new();
        $( hm.insert($key.to_string(), $value.to_string()); )*
        hm
    }};
}

#[cfg(test)]
mod tests {
    use super::HashMap;

    #[test]
    fn hash2headers() {
        let tests: Vec<HashMap<String, String>> = vec![
            HashMap::new(),
            map!(String, "Content-Type" => "text/html"; "Content-Length" => "12"),
            // Note: looks like they don't actually validate HeaderNames
            map!(String, "Content-zz" => "text/html"; "Content-Length" => "12"),
            /* TODO: make more generic?
             * {
             *     let mut hm = std::collections::HashMap::new();
             *     hm.insert("heelo".to_string, 12);
             *     hm
             * }, */
        ];
        for test in tests {
            let headers = super::hash2headers(&test);
            println!("{:#?}", headers);
        }
    }
}
