//! Transport layer for MCP communication

use crate::types::{McpClientError, McpRequest, McpResponse};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Request, RequestInit, Response};

/// HTTP transport for MCP
pub struct HttpTransport {
    base_url: String,
}

impl HttpTransport {
    /// Create a new HTTP transport
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    /// Send a request to the server
    pub async fn send(&self, request: McpRequest) -> Result<McpResponse, McpClientError> {
        let window = web_sys::window().ok_or(McpClientError::Transport("No window found".to_string()))?;

        let json = serde_json::to_string(&request).map_err(McpClientError::from)?;

        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_body(&JsValue::from_str(&json));

        let request = Request::new_with_str_and_init(&self.base_url, &opts)
            .map_err(|_| McpClientError::Transport("Failed to create request".to_string()))?;

        request
            .headers()
            .set("Content-Type", "application/json")
            .map_err(|_| McpClientError::Transport("Failed to set header".to_string()))?;

        let promise = window.fetch_with_request(&request);
        let response = wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .map_err(|_| McpClientError::Transport("Fetch failed".to_string()))?
            .dyn_into::<Response>()
            .map_err(|_| McpClientError::Transport("Not a response".to_string()))?;

        let json_promise = response.json().map_err(|_| McpClientError::Transport("Failed to get JSON".to_string()))?;
        let json_value = wasm_bindgen_futures::JsFuture::from(json_promise)
            .await
            .map_err(|_| McpClientError::Transport("Failed to parse JSON".to_string()))?;

        let response: McpResponse =
            serde_wasm_bindgen::from_value(json_value).map_err(|e| McpClientError::JsonDeserialization(e.to_string()))?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_creation() {
        let transport = HttpTransport::new("http://localhost:3000".to_string());
        assert_eq!(transport.base_url, "http://localhost:3000");
    }
}
