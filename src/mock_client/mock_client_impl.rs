use super::mock_client::MockLLMClient;

impl MockLLMClient {
    pub fn new() -> MockLLMClient {
        MockLLMClient { id: 1 }
    }

    pub fn chat(&self, message: &str) -> String {
        format!("Mock response to '{}'", message)
    }
}
