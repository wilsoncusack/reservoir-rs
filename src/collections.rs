use serde::Deserialize;
use strum_macros::Display;

#[derive(Display)]
#[strum(serialize_all = "camelCase")]
enum QueryParam {
    Id,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub collections: Vec<Collection>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: String,
    pub created_at: String,
    pub name: String,
    pub image: String,
    pub token_count: String,
    pub on_sale_count: String,
    pub volume: VolumeStats,
}

#[derive(Deserialize, Debug)]
pub struct VolumeStats {
    #[serde(rename = "1day")]
    pub one_day: f64,
    #[serde(rename = "7day")]
    pub seven_day: f64,
    #[serde(rename = "30day")]
    pub thirty_day: f64,
    #[serde(rename = "allTime")]
    pub all_time: f64,
}

impl crate::client::Client {
    pub async fn collections(&self, id: &str) -> Result<Response, eyre::Error> {
        let url = "/collections/v5";
        let query: Vec<(String, String)> = vec![(QueryParam::Id.to_string(), id.to_string())];
        Ok(self.get::<_, Response>(&url, query).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::client;
    #[tokio::test]
    async fn price_in_atomic_units_computes_correctly() {
        let c = client::Client::new(client::Chain::Ethereum, "demo-api-key".to_string());
        let r = c
            .collections(&"0x8d04a8c79ceb0889bdd12acdf3fa9d207ed3ff63")
            .await
            .unwrap();
        assert_eq!(r.collections.len(), 1);
        assert_eq!(r.collections.first().unwrap().name, "Blitmap");
    }
}
