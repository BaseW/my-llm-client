use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ChatGPTClientConfig {
    pub api_key: String,
}

pub struct ChatGPTClient {
    pub api_key: String,
    pub messages: Vec<ChatGPTMessage>,
    pub internal_client: Client,
}

#[derive(Clone, Debug)]
pub enum ChatGPTMessageRole {
    SYSTEM,
    USER,
    ASSISTANT,
}

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct ChatGPTMessage {
    pub role: ChatGPTMessageRole,
    pub content: String,
}

#[derive(Serialize)]
pub struct ChatGPTChatRequest {
    pub model: String,
    pub messages: Vec<ChatGPTMessage>,
}

#[derive(Deserialize, Debug)]
pub struct ChatGPTChatChoice {
    pub index: u32,
    pub message: ChatGPTMessage,
    pub logprobs: Option<()>,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatGPTUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct ChatGPTChatResponse {
    // pub id: String,
    // pub object: String,
    pub created: u32,
    pub model: String,
    pub system_fingerprint: String,
    pub choices: Vec<ChatGPTChatChoice>,
    pub usage: ChatGPTUsage,
}
