use serde::Serialize;

pub enum GeminiContentRole {
    User,
    Model,
}

#[derive(Serialize)]
pub struct GeminiContentPart {
    text: String,
}

#[derive(Serialize)]
pub struct GeminiContent {
    parts: Vec<GeminiContentPart>,
    role: GeminiContentRole,
}
