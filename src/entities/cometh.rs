use crate::api::ApiClient;
use serde_json::json;

pub struct Cometh {
    row: u32,
    column: u32,
    direction: Direction,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Cometh {
    pub fn new(row: u32, column: u32, direction: Direction) -> Self {
        Self { row, column, direction }
    }

    pub fn to_json(&self, candidate_id: &str) -> serde_json::Value {
        json!({
            "candidateId": candidate_id,
            "row": self.row,
            "column": self.column,
            "direction": format!("{:?}", self.direction).to_lowercase(),
        })
    }

    pub async fn create(&self, api_client: &ApiClient) -> Result<(), reqwest::Error> {
        api_client.post("comeths", self.to_json(api_client.get_candidate_id())).await
    }

    // pub async fn delete(&self, api_client: &ApiClient) -> Result<(), reqwest::Error> {
    //     api_client.delete("comeths", self.to_json(api_client.get_candidate_id())).await
    // }
}
