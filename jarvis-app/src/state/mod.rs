#![allow(dead_code)]
//! Application state management
//!
//! This module provides global state management for the JARVIS application
//! using Leptos signals and context.

use jarvis_ai::{InferenceConfig, InferenceEngine, Message, ModelType};
use jarvis_mcp::{McpClient, McpServerConfig};
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Global application state
#[derive(Clone)]
pub struct AppState {
    /// Current conversation messages
    pub messages: RwSignal<Vec<Message>>,
    /// Whether the AI is currently processing
    pub is_processing: RwSignal<bool>,
    /// Currently selected model
    pub current_model: RwSignal<Option<ModelType>>,
    /// Whether a model is loaded
    pub model_loaded: RwSignal<bool>,
    /// Error message if any
    pub error: RwSignal<Option<String>>,
    /// MCP servers configuration
    pub mcp_servers: RwSignal<Vec<McpServerConfig>>,
    /// Inference configuration
    pub inference_config: RwSignal<InferenceConfig>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self {
            messages: RwSignal::new(Vec::new()),
            is_processing: RwSignal::new(false),
            current_model: RwSignal::new(None),
            model_loaded: RwSignal::new(false),
            error: RwSignal::new(None),
            mcp_servers: RwSignal::new(Vec::new()),
            inference_config: RwSignal::new(InferenceConfig::default()),
        }
    }

    /// Add a user message to the conversation
    pub fn add_user_message(&self, content: String) {
        self.messages.update(|msgs| {
            msgs.push(Message::user(content));
        });
    }

    /// Add an assistant message to the conversation
    pub fn add_assistant_message(&self, content: String) {
        self.messages.update(|msgs| {
            msgs.push(Message::assistant(content));
        });
    }

    /// Add a system message to the conversation
    pub fn add_system_message(&self, content: String) {
        self.messages.update(|msgs| {
            msgs.push(Message::system(content));
        });
    }

    /// Clear all messages
    pub fn clear_messages(&self) {
        self.messages.set(Vec::new());
    }

    /// Set error message
    pub fn set_error(&self, error: Option<String>) {
        self.error.set(error);
    }

    /// Clear error
    pub fn clear_error(&self) {
        self.error.set(None);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Provide the application state to the component tree
pub fn provide_app_state() {
    let state = AppState::new();
    provide_context(state);
}

/// Get the application state from context
pub fn use_app_state() -> AppState {
    expect_context::<AppState>()
}

/// AI service for managing inference
pub struct AiService {
    engine: Rc<RefCell<InferenceEngine>>,
}

impl AiService {
    /// Create a new AI service
    pub fn new() -> Self {
        Self {
            engine: Rc::new(RefCell::new(InferenceEngine::new())),
        }
    }

    /// Load a model
    pub fn load_model(&self, model: ModelType) -> Result<(), String> {
        let mut engine = self.engine.borrow_mut();
        engine.load_model(model)
    }

    /// Generate a response from messages
    pub fn generate(&self, messages: &[Message]) -> Result<String, String> {
        self.engine.borrow().generate(messages)
    }

    /// Transcribe audio
    pub fn transcribe(&self, audio: &[f32]) -> Result<String, String> {
        self.engine.borrow().transcribe(audio)
    }

    /// Check if model is ready
    pub fn is_ready(&self) -> bool {
        self.engine.borrow().is_ready()
    }

    /// Check if model is loading
    pub fn is_loading(&self) -> bool {
        self.engine.borrow().is_loading()
    }
}

impl Default for AiService {
    fn default() -> Self {
        Self::new()
    }
}

/// MCP service for managing server connections
pub struct McpService {
    clients: Rc<RefCell<Vec<McpClient>>>,
}

impl McpService {
    /// Create a new MCP service
    pub fn new() -> Self {
        Self {
            clients: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Add a server configuration and connect
    pub async fn add_server(&self, config: McpServerConfig) -> Result<(), String> {
        let mut client = McpClient::new(config);
        client.connect().await?;
        self.clients.borrow_mut().push(client);
        Ok(())
    }

    /// Disconnect all servers
    pub fn disconnect_all(&self) {
        for client in self.clients.borrow_mut().iter_mut() {
            client.disconnect();
        }
        self.clients.borrow_mut().clear();
    }

    /// Get number of connected servers
    pub fn connected_count(&self) -> usize {
        self.clients
            .borrow()
            .iter()
            .filter(|c| c.is_connected())
            .count()
    }
}

impl Default for McpService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let state = AppState::new();
        assert!(state.messages.get_untracked().is_empty());
        assert!(!state.is_processing.get_untracked());
        assert!(state.current_model.get_untracked().is_none());
    }
}
