use reqwest::{Client, Error};
use serde_json::Value;
use anyhow::{anyhow, Result};


pub struct ApiClient {
    client: Client,
    base_url: String,
    pub candidate_id: String,
}

impl ApiClient {
    pub fn new(candidate_id: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://challenge.crossmint.io/api".to_string(),
            candidate_id: candidate_id.to_string(),
        }
    }

    pub fn get_candidate_id(&self) -> &str {
        &self.candidate_id
    }
    // pub async fn get(&self, endpoint: &str) -> Result<Value, reqwest::Error> {
    //     let url = format!("{}/{}", self.base_url, endpoint);
    //     let response = self.client.get(&url).send().await?;
    //     response.json().await
    // }

    pub async fn get_goal_map(&self) -> Result<Value> {
        let url = format!("{}/map/{}/goal", self.base_url, self.candidate_id);
        let response = self.client.get(&url).send().await?;
    
        if response.status().is_success() {
            let goal_map = response.json::<Value>().await?;
            Ok(goal_map)
        } else {
            let status = response.status();
            let error_text = response.text().await?;
            println!("GET goal map from {} failed: {}", url, error_text);
            Err(anyhow!("Request failed with status: {} - {}", status, error_text))
        }
    }

    

    pub async fn post(&self, endpoint: &str, body: serde_json::Value) -> Result<(), Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(&body).send().await?;

        if response.status().is_success() {
            println!("POST to {} succeeded.", url);
        } else {
            println!("POST to {} failed: {:?}", url, response.text().await?);
        }

        Ok(())
    }

    // pub async fn delete(&self, endpoint: &str, body: serde_json::Value) -> Result<(), Error> {
    //     let url = format!("{}/{}", self.base_url, endpoint);
    //     let response = self.client.delete(&url).json(&body).send().await?;

    //     if response.status().is_success() {
    //         println!("DELETE to {} succeeded.", url);
    //     } else {
    //         println!("DELETE to {} failed: {:?}", url, response.text().await?);
    //     }

    //     Ok(())
    // }
}
