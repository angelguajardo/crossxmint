use crate::api::ApiClient;
use serde_json::json;

pub struct Soloon {
    row: u32,
    column: u32,
    color: Color,
}

#[derive(Debug)]
pub enum Color {
    Blue,
    Red,
    Purple,
    White,
}

impl Soloon {
    pub fn new(row: u32, column: u32, color: Color) -> Self {
        Self { row, column, color }
    }

    pub fn to_json(&self, candidate_id: &str) -> serde_json::Value {
        json!({
            "candidateId": candidate_id,
            "row": self.row,
            "column": self.column,
            "color": format!("{:?}", self.color).to_lowercase(),
        })
    }

    pub async fn create(&self, api_client: &ApiClient) -> Result<(), reqwest::Error> {
        api_client.post("soloons", self.to_json(api_client.get_candidate_id())).await
    }

    // pub async fn delete(&self, api_client: &ApiClient) -> Result<(), reqwest::Error> {
    //     api_client.delete("soloons", self.to_json(api_client.get_candidate_id())).await
    // }
}
