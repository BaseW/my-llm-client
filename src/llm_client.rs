use std::future::Future;

use crate::{
    chatgpt_client::chatgpt_client::ChatGPTClient, mock_client::mock_client::MockLLMClient,
};
use secrecy::Secret;

pub trait LLMClientConfig {
    fn api_key(&self) -> Secret<String>;
}

pub trait LLMClientTrait<C: LLMClientConfig> {
    fn new() -> Self;
    fn with_config(config: C) -> Self;
    fn chat(&self, message: &str) -> impl Future<Output = String>;
}

pub struct MyLLMClient {
    pub mock_client: Option<MockLLMClient>,
    pub chatgpt_client: Option<ChatGPTClient>,
    // TODO: add Gemini client
    pub gemini_client: Option<MockLLMClient>,
}
