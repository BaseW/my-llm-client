use super::gemini_client::{GeminiContent, GeminiContentRole};
use reqwest::Body;
use serde::Serialize;

impl Serialize for GeminiContentRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            GeminiContentRole::User => serializer.serialize_str("user"),
            GeminiContentRole::Model => serializer.serialize_str("model"),
        }
    }
}

impl From<GeminiContent> for Body {
    fn from(content: GeminiContent) -> Self {
        serde_json::to_string(&content)
            .map(|body| Body::from(body))
            .unwrap()
    }
}
