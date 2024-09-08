pub mod chatgpt_client;
pub mod llm_client;
pub mod llm_client_impl;
pub mod mock_client;

#[derive(Clone, PartialEq)]
pub enum LLMProvider {
    Mock,
    ChatGPT,
    Gemini,
}

impl From<&str> for LLMProvider {
    fn from(s: &str) -> Self {
        match s {
            "mock" => LLMProvider::Mock,
            "chatgpt" => LLMProvider::ChatGPT,
            "gemini" => LLMProvider::Gemini,
            _ => panic!("Invalid provider"),
        }
    }
}
