use super::mock_client::MockLLMClient;
use crate::llm_client::LLMClientTrait;

impl LLMClientTrait for MockLLMClient {
    fn new() -> MockLLMClient {
        MockLLMClient { id: 1 }
    }

    fn chat(&self, message: &str) -> String {
        format!("Mock response to '{}'", message)
    }
}
