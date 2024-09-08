use super::chatgpt_client::{
    ChatGPTChatRequest, ChatGPTChatResponse, ChatGPTClient, ChatGPTClientConfig, ChatGPTMessage,
    ChatGPTMessageRole,
};
use crate::llm_client::{LLMClientConfig, LLMClientTrait};
use reqwest::{Body, Client};
use serde::{Deserialize, Serialize};

impl LLMClientConfig for ChatGPTClientConfig {
    fn api_key(&self) -> secrecy::Secret<String> {
        secrecy::Secret::new("ChatGPT API key".to_string())
    }
}

impl Serialize for ChatGPTMessageRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ChatGPTMessageRole::SYSTEM => serializer.serialize_str("system"),
            ChatGPTMessageRole::USER => serializer.serialize_str("user"),
            ChatGPTMessageRole::ASSISTANT => serializer.serialize_str("assistant"),
        }
    }
}

impl<'de> Deserialize<'de> for ChatGPTMessageRole {
    fn deserialize<D>(deserializer: D) -> Result<ChatGPTMessageRole, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "system" => Ok(ChatGPTMessageRole::SYSTEM),
            "user" => Ok(ChatGPTMessageRole::USER),
            "assistant" => Ok(ChatGPTMessageRole::ASSISTANT),
            _ => Err(serde::de::Error::custom("unexpected value")),
        }
    }
}

impl From<ChatGPTChatRequest> for Body {
    fn from(request: ChatGPTChatRequest) -> Self {
        serde_json::to_string(&request)
            .map(|body| Body::from(body))
            .unwrap()
    }
}

impl LLMClientTrait<ChatGPTClientConfig> for ChatGPTClient {
    fn new() -> ChatGPTClient {
        let internal_client = Client::new();
        let api_key = "".to_string();
        let messages = vec![];
        ChatGPTClient {
            internal_client,
            api_key,
            messages,
        }
    }

    fn with_config(config: ChatGPTClientConfig) -> Self {
        let internal_client = Client::new();
        let api_key = config.api_key;
        let messages = vec![];
        ChatGPTClient {
            internal_client,
            api_key,
            messages,
        }
    }

    async fn chat(&self, message: &str) -> String {
        // TODO: validate for api_key
        let authorization_header = format!("Bearer {}", self.api_key);
        let mut new_messages = self.messages.clone();
        let new_message = ChatGPTMessage {
            role: ChatGPTMessageRole::USER,
            content: message.to_string(),
        };
        new_messages.push(new_message);
        let request = ChatGPTChatRequest {
            model: "gpt-4o-mini".to_string(),
            messages: new_messages,
        };
        let response = self
            .internal_client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", authorization_header)
            .header("Content-Type", "application/json")
            .body(request)
            .send()
            .await;
        match response {
            Ok(response) => {
                let response_body = response.json::<ChatGPTChatResponse>().await;
                match response_body {
                    Ok(response_body) => match response_body.choices.first() {
                        Some(choice) => choice.message.content.clone(),
                        None => panic!("Failed to get response"),
                    },
                    Err(_) => panic!("Failed to parse response"),
                }
            }
            Err(_) => panic!("Failed to send request"),
        }
    }
}
