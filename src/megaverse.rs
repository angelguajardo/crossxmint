use crate::api::ApiClient;
use crate::entities::{Polyanet, Soloon, Cometh, Color, Direction};
use tokio::time::{sleep, Duration};
use serde_json::Value;
use anyhow::{Result, anyhow};

pub struct Megaverse {
    api_client: ApiClient,
}

impl Megaverse {
    pub fn new(api_client: ApiClient) -> Self {
        Self { api_client }
    }

    // Function to fetch the goal map
    async fn get_goal_map(&self) -> Result<Value> {
        self.api_client.get_goal_map().await
    }

    // Main function to process the goal map and populate entities
    pub async fn populate_from_goal(&self) -> Result<()> {
        // Retrieve the map and validate
        let goal_map = self.get_goal_map().await?;
        let map_data = goal_map["goal"].as_array().ok_or_else(|| anyhow!("Map format is incorrect"))?;

        // Process each cell in the map grid
        for (row, row_elements) in map_data.iter().enumerate() {
            if let Some(column_data) = row_elements.as_array() {
                self.process_row(row as u32, column_data).await?;
            }
        }

        Ok(())
    }

    async fn process_row(&self, row: u32, column_data: &[Value]) -> Result<()> {
        for (col, cell) in column_data.iter().enumerate() {
            if let Some(entity) = cell.as_str() {
                self.handle_entity(row, col as u32, entity).await?;
            }
        }
        Ok(())
    }

    async fn handle_entity(&self, row: u32, col: u32, entity: &str) -> Result<()> {
        match entity.to_lowercase().as_str() {
            "polyanet" => {
                let polyanet = Polyanet::new(row, col);
                polyanet.create(&self.api_client).await?;
            },
            entity_type if entity_type.starts_with("soloon") => {
                let color_str = entity_type.split('_').nth(1).unwrap_or("unknown");
                let color = match color_str {
                    "blue" => Color::Blue,
                    "red" => Color::Red,
                    "purple" => Color::Purple,
                    "white" => Color::White,
                    _ => return Ok(()), // Ignore if the color is unrecognized
                };
                let soloon = Soloon::new(row, col, color);
                soloon.create(&self.api_client).await?;
            },
            entity_type if entity_type.starts_with("cometh") => {
                let direction_str = entity_type.split('_').nth(1).unwrap_or("unknown");
                let direction = match direction_str {
                    "up" => Direction::Up,
                    "down" => Direction::Down,
                    "left" => Direction::Left,
                    "right" => Direction::Right,
                    _ => return Ok(()), // Ignore if the direction is unrecognized
                };
                let cometh = Cometh::new(row, col, direction);
                cometh.create(&self.api_client).await?;
            },
            _ => {} // Ignore "space" and any unknown entities
        }
        sleep(Duration::from_millis(4000)).await; // Rate-limit control
        Ok(())
    }

    // Existing populate function
    pub async fn populate(&self) -> Result<()> {
        for i in 0..11 {
            if i != 5 && i != 0 && i != 1 && i != 10 && i != 9 {
                let polyanet = Polyanet::new(i, i);
                polyanet.create(&self.api_client).await?;
                sleep(Duration::from_millis(300)).await;
                let polyanet = Polyanet::new(i, 11 - i - 1);
                polyanet.create(&self.api_client).await?;
                sleep(Duration::from_millis(300)).await;
            } else if i == 5 {
                let polyanet = Polyanet::new(i, i);
                polyanet.create(&self.api_client).await?;
                sleep(Duration::from_millis(300)).await;
            } else {
                sleep(Duration::from_millis(300)).await;
            }
        }
        Ok(())
    }
}
