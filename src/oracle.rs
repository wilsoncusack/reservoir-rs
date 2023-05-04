use ethers::types::Bytes;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Display)]
#[strum(serialize_all = "camelCase")]
pub enum PriceKind {
    Upper,
    Lower,
    Twap,
    Spot,
}

#[derive(Display)]
#[strum(serialize_all = "camelCase")]
enum QueryParam {
    Kind,
    Currency,
    TwapSeconds,
    Collection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OracleResponse {
    pub price: f64,
    pub message: OracleMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OracleMessage {
    pub id: String,
    pub payload: Bytes,
    pub timestamp: u64,
    pub signature: Bytes,
}

impl crate::client::Client {
    pub async fn max_collection_bid(
        &self,
        collection: &str,
        price_kind: PriceKind,
        quote_currency: Option<&str>,
        twap_seconds: Option<u32>,
    ) -> Result<OracleResponse, eyre::Error> {
        let url = "/oracle/collections/top-bid/v2";
        let mut query: Vec<(String, String)> = vec![
            (QueryParam::Collection.to_string(), collection.to_string()),
            (QueryParam::Kind.to_string(), price_kind.to_string()),
        ];
        if let Some(twap_seconds) = twap_seconds {
            query.push((
                QueryParam::TwapSeconds.to_string(),
                twap_seconds.to_string(),
            ))
        }
        if let Some(quote_currency) = quote_currency {
            query.push((QueryParam::Currency.to_string(), quote_currency.to_string()))
        }
        Ok(self.get::<_, OracleResponse>(&url, query).await?)
    }
}
