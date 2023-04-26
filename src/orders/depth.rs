use serde::Deserialize;
use strum_macros::Display;

#[derive(Display)]
#[strum(serialize_all = "camelCase")]
enum QueryParam {
    Side,
    Collection,
}

#[derive(Display)]
#[strum(serialize_all = "camelCase")]
pub enum DepthSide {
    Buy,
    Sell,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    pub depth: Vec<DepthDetail>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DepthDetail {
    pub price: f64,
    pub quantity: i64,
}

impl crate::client::Client {
    pub async fn depth(&self, side: DepthSide, collection: &str) -> Result<Response, eyre::Error> {
        let url = "/orders/depth/v1";
        let query: Vec<(String, String)> = vec![
            (QueryParam::Collection.to_string(), collection.to_string()),
            (QueryParam::Side.to_string(), side.to_string()),
        ];

        Ok(self.get::<_, Response>(&url, query).await?)
    }
}

mod tests {
    use crate::client;
    use crate::orders::depth;

    #[tokio::test]
    async fn fetches_depth_without_error() {
        let c = client::Client::new(client::Chain::Ethereum, "demo-api-key".to_string());
        c.depth(
            depth::DepthSide::Buy,
            &"0x8d04a8c79ceb0889bdd12acdf3fa9d207ed3ff63",
        )
        .await
        .unwrap();
    }
}
