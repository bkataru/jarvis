use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of the AI model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelStatus {
    Idle,
    ModelLoading,
    ConversationLoading,
    Ready,
}

/// Role of a message in a conversation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    Assistant,
    User,
    System,
}

/// Type of message part
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagePartType {
    Text,
    ToolCall,
}

/// Base message part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePartBase {
    pub id: String,
    #[serde(rename = "type")]
    pub part_type: MessagePartType,
}

/// Text message part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePartText {
    #[serde(flatten)]
    pub base: MessagePartBase,
    pub text: String,
}

/// Response media for tool calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMedia {
    #[serde(rename = "type")]
    pub media_type: String,
    pub data: String,
    pub mime_type: String,
}

/// Tool call message part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePartTool {
    #[serde(flatten)]
    pub base: MessagePartBase,
    pub function_name: String,
    pub parameters: serde_json::Value,
    pub response: String,
    pub response_media: Option<ResponseMedia>,
}

/// Message part enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessagePart {
    #[serde(rename = "TEXT")]
    Text(MessagePartText),
    #[serde(rename = "TOOL_CALL")]
    ToolCall(MessagePartTool),
}

/// Message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: MessageRole,
    pub message_parts: Vec<MessagePart>,
}

impl Message {
    /// Create a new user message
    pub fn user(text: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role: MessageRole::User,
            message_parts: vec![MessagePart::Text(MessagePartText {
                base: MessagePartBase {
                    id: Uuid::new_v4().to_string(),
                    part_type: MessagePartType::Text,
                },
                text,
            })],
        }
    }

    /// Create a new assistant message
    pub fn assistant(text: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role: MessageRole::Assistant,
            message_parts: vec![MessagePart::Text(MessagePartText {
                base: MessagePartBase {
                    id: Uuid::new_v4().to_string(),
                    part_type: MessagePartType::Text,
                },
                text,
            })],
        }
    }

    /// Create a new system message
    pub fn system(text: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role: MessageRole::System,
            message_parts: vec![MessagePart::Text(MessagePartText {
                base: MessagePartBase {
                    id: Uuid::new_v4().to_string(),
                    part_type: MessagePartType::Text,
                },
                text,
            })],
        }
    }
}

/// Configuration options for a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationOptions {
    pub log_enabled: bool,
    pub conversation_end_keyword: Option<String>,
}

impl Default for ConversationOptions {
    fn default() -> Self {
        Self {
            log_enabled: true,
            conversation_end_keyword: Some("CONVERSATION_ENDED".to_string()),
        }
    }
}
