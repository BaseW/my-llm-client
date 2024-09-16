use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum GeminiContentRole {
    User,
    Model,
}

#[derive(Serialize, Deserialize)]
pub struct GeminiContentPart {
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct GeminiContent {
    parts: Vec<GeminiContentPart>,
    role: GeminiContentRole,
}

#[derive(Deserialize)]
pub enum FinishReason {
    FinishReasonUnspecified,
    Stop,
    MaxTokens,
    Safety,
    Recitation,
    Language,
    Other,
    BlockList,
    ProhibitedContent,
    SPII,
    MalformedFunctionCall,
}

#[derive(Deserialize)]
pub struct GeminiSafetyRating {}

#[derive(Deserialize)]
pub struct AttributionSourceId {}

#[derive(Deserialize)]
pub struct GroundingAttribution {
    souurceId: AttributionSourceId,
    content: GeminiContent,
}

#[derive(Deserialize)]
pub struct CitationMetadata {}

#[derive(Deserialize)]
pub struct GeminiContentCandidate {
    content: GeminiContent,
    finish_reason: FinishReason,
    safety_ratings: Vec<GeminiSafetyRating>,
    citation_metadata: CitationMetadata,
    token_count: i32,
    index: usize,
}

#[derive(Deserialize)]
pub enum GeminiPromptFeedbackBlockReason {
    BlockReasonUnspecified,
    Safety,
    Other,
    BlockList,
    ProhibitedContent,
}

#[derive(Deserialize)]
pub struct GeminiPromptFeedbackSafetyRating {}

#[derive(Deserialize)]
pub struct GeminiPromptFeedback {
    block_reason: GeminiPromptFeedbackBlockReason,
    safety_rating: GeminiPromptFeedbackSafetyRating,
}

#[derive(Deserialize)]
pub struct GeminiUsageMetadata {
    prompt_token_count: i32,
    cached_content_token_count: i32,
    candidates_token_count: i32,
    total_token_count: i32,
}

#[derive(Deserialize)]
pub struct GeminiContentResponse {
    prompt_feedback: GeminiPromptFeedback,
    usage_metadata: GeminiUsageMetadata,
}
