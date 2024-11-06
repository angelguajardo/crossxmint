use crate::api::ApiClient;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

pub struct Polyanet {
    row: u32,
    column: u32,
}

impl Polyanet {
    pub fn new(row: u32, column: u32) -> Self {
        Self { row, column }
    }

    pub fn to_json(&self, candidate_id: &str) -> serde_json::Value {
        json!({
            "candidateId": candidate_id,
            "row": self.row,
            "column": self.column,
        })
    }

    pub async fn create(&self, api_client: &ApiClient) -> Result<(), reqwest::Error> {
        // Log the JSON payload for verification
        let json_payload = self.to_json(api_client.get_candidate_id());
        println!("Creating Polyanet with payload: {:?}", json_payload);

        // Send POST request
        let result = api_client.post("polyanets", json_payload).await;
        
        // Optional: Add a delay to avoid rate-limiting
        sleep(Duration::from_secs(2)).await;

        result
    }

    // pub async fn delete(&self, api_client: &ApiClient) -> Result<(), reqwest::Error> {
    //     let json_payload = self.to_json(api_client.get_candidate_id());
    //     println!("Deleting Polyanet with payload: {:?}", json_payload);
    //     api_client.delete("polyanets", json_payload).await
    // }
}
