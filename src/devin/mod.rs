use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

const DEVIN_API_BASE_URL: &str = "https://api.devin.ai/v1";

#[derive(Debug, Deserialize, Serialize)]
pub struct Knowledge {
    pub id: String,
    pub name: String,
    pub body: String,
    pub trigger_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Folder {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListKnowledgeResponse {
    pub knowledge: Vec<Knowledge>,
    #[serde(default)]
    pub folders: Vec<Folder>,
}

pub struct DevinClient {
    client: Client,
    api_key: String,
}

impl DevinClient {
    pub fn new() -> Result<Self, DevinError> {
        let api_key = env::var("DEVIN_API_KEY").map_err(|_| {
            DevinError::Config("DEVIN_API_KEY environment variable not set".to_string())
        })?;

        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    pub async fn list_knowledge(&self) -> Result<ListKnowledgeResponse, DevinError> {
        let url = format!("{}/knowledge", DEVIN_API_BASE_URL);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(DevinError::Api(format!(
                "API error ({}): {}",
                status, error_text
            )));
        }

        let knowledge_response = response.json::<ListKnowledgeResponse>().await?;

        Ok(knowledge_response)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DevinError {
    #[error("API error: {0}")]
    Api(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
