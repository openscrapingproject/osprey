use log::{debug, info};

use serde::{Deserialize, Serialize};

use crate::api::Response;
use crate::utils;
use anyhow::Result;

use std::collections::HashMap;

use std::time::Duration;

use async_trait::async_trait;

use std::convert::TryFrom;

async fn convert_response(input: reqwest::Response) -> Result<Response> {
    Ok(Response {
        url: input.url().to_string(),
        status: i32::try_from(input.status().as_u16()).unwrap(),
        version: format!("{:#?}", input.version()),
        headers: {
            // TODO: use version of this in utils.
            let mut hm = HashMap::new();
            for (k, v) in input.headers() {
                hm.insert(k.to_string(), v.to_str()?.to_string());
            }
            hm
        },
        body: input.text().await?,
    })
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BasicRequestor {
    // version: String,
    #[serde(with = "humantime_serde")]
    pub timeout: Option<Duration>,
    pub headers: HashMap<String, String>,
}

#[async_trait]
#[typetag::serde(name = "basic")]
impl crate::api::Requestor for BasicRequestor {
    // type Response = Response;
    async fn make_request(&self, url: &str) -> Result<Response> {
        // TODO: reuse clients, think about pooling?
        info!("in make_request");
        let config = self;

        let builder = reqwest::ClientBuilder::new()
            .timeout(config.timeout.unwrap_or(Duration::from_secs(5)));

        let client = builder.build()?;

        let req = client
            .get(url)
            .headers(utils::hash2headers(&config.headers)?)
            .build()?;

        debug!("Request = {:#?}", req);

        let resp = client.execute(req).await?;
        debug!("Response = {:#?}", resp);
        convert_response(resp).await
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use super::BasicRequestor;
    use super::Duration;
    use crate::api::Requestor;

    // TODO: fix this, so we don't have to copy our utils::map macro
    // in every test
    // use super::map;

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

    use anyhow::Result;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn invalid_urls() -> Result<()> {
        init();

        let r = BasicRequestor {
            timeout: Some(Duration::from_secs(5)),
            headers: map!(String, "Accept" => "text/html"),
        };
        // r.configure(r.get_default_config())?;

        let urls = vec![
            "/product/:ID1",
            "/product/:ID2",
            "each URL generates a new Job",
            "with an ID of collection_name_0 ... collection_name_N",
        ];

        for url in urls {
            let result = Requestor::make_request(&r, url).await;
            match result {
                Err(_) => continue,
                Ok(_) => return Err(Error::msg("these should have failed")),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn valid_urls() -> Result<()> {
        init();

        let r = BasicRequestor {
            timeout: Some(Duration::from_secs(5)),
            headers: map!(String, "Accept" => "text/html"),
        };

        let urls = vec![
            "https://google.com",
            "https://www.cnn.com/",
            "https://www.nytimes.com/",
        ];

        for url in urls {
            let result = r.make_request(url).await;
            match result {
                Err(e) => return Err(e),
                Ok(_) => continue,
            }
        }

        Ok(())
    }
}
