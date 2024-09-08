use super::mock_client::{MockLLMClient, MockLLMClientConfig};
use crate::llm_client::{LLMClientConfig, LLMClientTrait};

impl LLMClientConfig for MockLLMClientConfig {
    fn api_key(&self) -> secrecy::Secret<String> {
        secrecy::Secret::new("Mock API key".to_string())
    }
}

impl LLMClientTrait<MockLLMClientConfig> for MockLLMClient {
    fn new() -> MockLLMClient {
        MockLLMClient { id: 1 }
    }

    fn with_config(_: MockLLMClientConfig) -> Self {
        MockLLMClient { id: 1 }
    }

    async fn chat(&self, message: &str) -> String {
        format!("Mock response to '{}'", message)
    }
}
