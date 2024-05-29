use crate::chat::{Chat, Sender};
use log::{debug, error, info, warn};
use openai_api_rs::v1::api::Client as OpenAIClient;
use openai_api_rs::v1::chat_completion::{
    ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Const
const ANSWER_ERROR: &str = "It seems like the model is unavailable or something went wrong.";

// API and Models.

#[derive(Serialize, Deserialize)]
pub struct APIClient {
    pub endpoint: String,
    pub api_key: String,
    pub models: Vec<String>,
}

impl APIClient {
    pub fn new(endpoint: String, api_key: String, models: Vec<String>) -> Self {
        info!("Creating new APIClient");
        APIClient {
            endpoint,
            api_key,
            models,
        }
    }

    pub fn store(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("endpoint".to_owned(), self.endpoint.clone());
        map.insert("api_key".to_owned(), self.api_key.clone());
        map.insert("models".to_owned(), self.models.join(","));
        debug!("Storing APIClient data: {:?}", map);
        map
    }

    pub fn import(client_data: &HashMap<String, String>) -> Result<APIClient, String> {
        let endpoint = client_data
            .get("endpoint")
            .ok_or_else(|| {
                error!("Missing endpoint");
                "Missing endpoint".to_owned()
            })?
            .to_owned();
        let api_key = client_data
            .get("api_key")
            .ok_or_else(|| {
                error!("Missing API key");
                "Missing API key".to_owned()
            })?
            .to_owned();
        let models = client_data
            .get("models")
            .map(|s| {
                let models: Vec<String> = s.split(",").map(ToString::to_string).collect();
                debug!("Importing models: {:?}", models);
                models
            })
            .unwrap_or_else(|| {
                warn!("No models found, using default");
                Vec::new()
            });
        info!("Importing APIClient");
        Ok(APIClient::new(endpoint, api_key, models))
    }
}

// Threads
pub struct Thread {
    pub openai_api_client: OpenAIClient,
    pub model: String,
}

impl Thread {
    pub fn new(client: &APIClient, model: &str) -> Self {
        info!("Creating new Thread with model: {}", model);
        debug!("Creating new OpenAI API Client");
        let openai_api_client =
            OpenAIClient::new_with_endpoint(client.endpoint.clone(), client.api_key.clone());
        debug!("Successfully created new OpenAI API Client");
        Thread {
            openai_api_client,
            model: model.to_owned(),
        }
    }

    pub fn completion(&self, message: &str, chat: Chat) -> String {
        let mut chat_messages = Vec::new();
        for message in &chat.messages {
            chat_messages.push(ChatCompletionMessage {
                role: match message.sender {
                    Sender::User => MessageRole::user,
                    Sender::Model => MessageRole::assistant,
                },
                content: Content::Text(message.content.to_string()),
                name: None,
            });
        }

        chat_messages.push(ChatCompletionMessage {
            role: MessageRole::user,
            content: Content::Text(message.to_string()),
            name: None,
        });

        let req = ChatCompletionRequest::new(self.model.clone(), chat_messages);
        match self.openai_api_client.chat_completion(req) {
            Ok(result) => result
                .choices
                .get(0)
                .and_then(|choice| choice.message.content.as_ref())
                .map(|content| content.to_string())
                .unwrap_or_else(|| "No completion response found".to_string()),
            Err(err) => {
                error!("Error during chat completion: {}", err);
                ANSWER_ERROR.to_string()
            }
        }
    }
}
