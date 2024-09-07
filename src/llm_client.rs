use crate::mock_client::mock_client::MockLLMClient;

pub trait LLMClientTrait {
    fn new() -> Self;
    fn chat(&self, message: &str) -> String;
}

pub struct MyLLMClient {
    pub mock_client: Option<MockLLMClient>,
}
