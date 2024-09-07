use crate::llm_client::MyLLMClient;
use crate::mock_client::mock_client::MockLLMClient;

impl MyLLMClient {
    pub fn new(is_mock_client: bool) -> Self {
        let mock_client = if is_mock_client {
            Some(MockLLMClient { id: 0 })
        } else {
            None
        };
        Self { mock_client }
    }

    pub fn chat(&self, message: &str) -> String {
        match &self.mock_client {
            Some(mock_client) => mock_client.chat(message),
            None => panic!("Not implemented"),
        }
    }
}
