use std::env;

use crate::chatgpt_client::chatgpt_client::{ChatGPTClient, ChatGPTClientConfig};
use crate::llm_client::{LLMClientTrait, MyLLMClient};
use crate::mock_client::mock_client::MockLLMClient;
use crate::LLMProvider;

impl MyLLMClient {
    pub fn new(llm_provider: LLMProvider) -> Self {
        let mock_client = if llm_provider == LLMProvider::Mock {
            Some(MockLLMClient { id: 0 })
        } else {
            None
        };
        let chatgpt_client = if llm_provider == LLMProvider::ChatGPT {
            // TODO: get from args
            let api_key = env::var("CHATGPT_API_KEY").unwrap();
            let config = ChatGPTClientConfig { api_key };
            Some(ChatGPTClient::with_config(config))
        } else {
            None
        };
        let gemini_client = if llm_provider == LLMProvider::Gemini {
            None
        } else {
            None
        };
        Self {
            mock_client,
            chatgpt_client,
            gemini_client,
        }
    }

    pub async fn chat(&self, message: &str) -> String {
        match &self.mock_client {
            Some(mock_client) => mock_client.chat(message).await,
            None => {
                let chatgpt_client = self.chatgpt_client.as_ref().unwrap();
                chatgpt_client.chat(message).await
            }
        }
    }
}
