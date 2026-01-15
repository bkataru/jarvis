use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub name: String,
    pub url: Option<String>,
    pub server_type: Option<String>,
    pub active: bool,
    pub active_tools: Vec<String>,
    pub active_prompts: Vec<String>,
}

/// MCP tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// MCP resource definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResource {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

/// MCP prompt definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPrompt {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Vec<McpPromptArgument>,
}

/// MCP prompt argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPromptArgument {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
}

/// Tool call result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResult {
    pub success: bool,
    pub result: String,
    pub error: Option<String>,
}

/// MCP server state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerState {
    pub connected: bool,
    pub tools: Vec<McpTool>,
    pub resources: Vec<McpResource>,
    pub prompts: Vec<McpPrompt>,
}

impl McpServerState {
    /// Create a new disconnected state
    pub fn disconnected() -> Self {
        Self {
            connected: false,
            tools: Vec::new(),
            resources: Vec::new(),
            prompts: Vec::new(),
        }
    }
}

/// MCP request/response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<McpError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// Tool call parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallParams {
    pub name: String,
    pub arguments: HashMap<String, serde_json::Value>,
}

/// MCP client error type
#[derive(Debug, Clone, thiserror::Error)]
pub enum McpClientError {
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Transport error: {0}")]
    Transport(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Tool error: {0}")]
    Tool(String),
    
    #[error("Resource error: {0}")]
    Resource(String),
    
    #[error("Prompt error: {0}")]
    Prompt(String),
    
    #[error("JSON serialization error: {0}")]
    JsonSerialization(String),
    
    #[error("JSON deserialization error: {0}")]
    JsonDeserialization(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Not connected to server")]
    NotConnected,
    
    #[error("Server returned error: {0}")]
    Server(String),
    
    #[error("Timeout")]
    Timeout,
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<serde_json::Error> for McpClientError {
    fn from(err: serde_json::Error) -> Self {
        McpClientError::JsonSerialization(err.to_string())
    }
}

impl From<String> for McpClientError {
    fn from(err: String) -> Self {
        McpClientError::Unknown(err)
    }
}

impl From<&str> for McpClientError {
    fn from(err: &str) -> Self {
        McpClientError::Unknown(err.to_string())
    }
}