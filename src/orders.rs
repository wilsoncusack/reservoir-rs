use serde::Deserialize;
use strum_macros::Display;

#[derive(Display)]
#[strum(serialize_all = "camelCase")]
enum QueryParam {
    Collection,
    Limit,
    SortBy,
    Continuation,
}

#[derive(Display)]
#[strum(serialize_all = "camelCase")]
pub enum SortOption {
    Price,
    CreatedAt,
}

#[derive(Deserialize)]
pub struct BidsResponse {
    pub orders: Vec<Order>,
    pub continuation: Option<String>,
}

#[derive(Deserialize)]
pub struct Order {
    pub id: String,
    pub kind: String,
    pub price: Price,
    pub criteria: Criteria,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub amount: Amount,
    pub net_amount: NetAmount,
    pub currency: Currency,
}

#[derive(Deserialize)]
pub struct Amount {
    pub usd: f64,
    pub native: f64,
}

#[derive(Deserialize)]
pub struct NetAmount {
    pub raw: String,
    pub decimal: f64,
    pub native: f64,
}

#[derive(Deserialize)]
pub struct Currency {
    pub contract: String,
    pub decimals: u8,
}

#[derive(Deserialize)]
pub struct Criteria {
    pub kind: String,
}

impl crate::client::Client {
    pub async fn bids(
        &self,
        collection: &str,
        sort_by: Option<SortOption>,
        limit: Option<u64>,
        continuation: Option<String>,
    ) -> Result<BidsResponse, eyre::Error> {
        let url = "/orders/bids/v5";
        let mut query: Vec<(String, String)> =
            vec![(QueryParam::Collection.to_string(), collection.to_string())];
        if let Some(limit) = limit {
            query.push((QueryParam::Limit.to_string(), limit.to_string()))
        }
        if let Some(sort_by) = sort_by {
            query.push((QueryParam::SortBy.to_string(), sort_by.to_string()));
        }
        if let Some(continuation) = continuation {
            println!("in here {}", continuation);
            query.push((QueryParam::Continuation.to_string(), continuation));
        }
        Ok(self.get::<_, BidsResponse>(&url, query).await?)
    }
}

mod tests {
    use crate::client;
    #[tokio::test]
    async fn price_in_atomic_units_computes_correctly() {
        let c = client::Client::new(client::Chain::Ethereum, "demo-api-key".to_string());
        c.bids(
            &"0x8d04a8c79ceb0889bdd12acdf3fa9d207ed3ff63",
            None,
            None,
            None,
        )
        .await
        .unwrap();
    }
}
