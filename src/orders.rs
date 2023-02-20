use serde::Deserialize;
use strum_macros::Display;

#[derive(Display)]
#[strum(serialize_all = "camelCase")]
enum QueryParam {
    Collection,
    Limit,
    SortBy,
}

#[derive(Deserialize)]
pub struct BidsResponse {
    pub orders: Vec<Order>,
}

#[derive(Deserialize)]
pub struct Order {
    pub price: Price,
    pub criteria: Criteria,
}

#[derive(Deserialize)]
pub struct Price {
    pub amount: Amount,
}

#[derive(Deserialize, PartialEq, PartialOrd)]
pub struct Amount {
    pub usd: f64,
}

#[derive(Deserialize)]
pub struct Criteria {
    pub kind: String,
}

impl crate::client::Client {
    async fn bids(
        &self,
        collection: &str,
        limit: Option<u64>,
    ) -> Result<BidsResponse, eyre::Error> {
        let url = "/orders/bids/v5";
        let mut query: Vec<(String, String)> = vec![
            (
                QueryParam::Collection.to_string(),
                collection.to_string(),
            ),
            (QueryParam::SortBy.to_string(), "price".to_string()),
        ];
        if let Some(limit) = limit {
            query.push((QueryParam::Limit.to_string(), limit.to_string()))
        }
        Ok(self.get::<_, BidsResponse>(&url, query).await?)
    }
}
