use serde::{de::DeserializeOwned, Serialize};

pub enum Chain {
    Ethereum, 
    Goerli, 
    Optimism,
    Polygon
}

pub struct Client {
    client: reqwest::Client,
    base_url: String,
    api_key: String
}

impl Client {
    fn new(chain: Chain, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url_for_chain(chain),
            api_key: api_key
        }
    }
}

fn base_url_for_chain(chain: Chain) -> String {
    let prefix = match chain{
        Chain::Ethereum=>"",
        Chain::Goerli=>"-goerli",
        Chain::Optimism=>"-optimism",
        Chain::Polygon=>"-polygon",
    };
    format!("https://api{}.reservoir.tools", prefix)
}

impl Client {
    pub async fn get<Q: Serialize, D: DeserializeOwned>(
        &self,
        route: &str,
        query: Q,
    ) -> Result<D, eyre::Error> {
        let res = self
            .client
            .get(format!("{}{}", self.base_url, route))
            .query(&query)
            .header("api_key", self.api_key.clone())
            .send()
            .await?
            .json::<D>()
            .await?;
        Ok(res)
    }
}
