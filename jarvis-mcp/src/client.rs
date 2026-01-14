//! MCP client for connecting to and interacting with MCP servers

use crate::transport::HttpTransport;
use crate::types::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// Request ID generator
static REQUEST_ID: AtomicU64 = AtomicU64::new(1);

fn next_request_id() -> String {
    REQUEST_ID.fetch_add(1, Ordering::SeqCst).to_string()
}

/// MCP client for managing server connections
pub struct McpClient {
    config: McpServerConfig,
    state: McpServerState,
    transport: Option<HttpTransport>,
}

impl McpClient {
    /// Create a new MCP client
    pub fn new(config: McpServerConfig) -> Self {
        Self {
            config,
            state: McpServerState::disconnected(),
            transport: None,
        }
    }

    /// Connect to the MCP server
    pub async fn connect(&mut self) -> Result<(), String> {
        let url = self
            .config
            .url
            .as_ref()
            .ok_or("Server URL not configured")?;

        log::info!("Connecting to MCP server: {} at {}", self.config.name, url);

        // Create transport
        let transport = HttpTransport::new(url.clone());

        // Send initialize request
        let init_request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: next_request_id(),
            method: "initialize".to_string(),
            params: Some(serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "roots": { "listChanged": true },
                    "sampling": {}
                },
                "clientInfo": {
                    "name": "jarvis",
                    "version": "0.1.0"
                }
            })),
        };

        match transport.send(init_request).await {
            Ok(response) => {
                if let Some(error) = response.error {
                    log::error!("Server returned error: {}", error.message);
                    return Err(format!("Server error: {}", error.message));
                }

                log::info!("Successfully connected to MCP server: {}", self.config.name);
                self.transport = Some(transport);
                self.state.connected = true;

                // Fetch available tools, resources, and prompts
                if let Err(e) = self.refresh_capabilities().await {
                    log::warn!("Failed to fetch capabilities: {}", e);
                }

                Ok(())
            }
            Err(e) => {
                log::error!("Failed to connect to MCP server: {}", e);
                Err(format!("Connection failed: {}", e))
            }
        }
    }

    /// Refresh tools, resources, and prompts from the server
    async fn refresh_capabilities(&mut self) -> Result<(), String> {
        let transport = self.transport.as_ref().ok_or("Not connected")?;

        // List tools
        let tools_request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: next_request_id(),
            method: "tools/list".to_string(),
            params: None,
        };

        if let Ok(response) = transport.send(tools_request).await {
            if let Some(result) = response.result {
                if let Some(tools) = result.get("tools") {
                    if let Ok(tools) = serde_json::from_value::<Vec<McpTool>>(tools.clone()) {
                        self.state.tools = tools;
                        log::info!("Loaded {} tools from server", self.state.tools.len());
                    }
                }
            }
        }

        // List resources
        let resources_request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: next_request_id(),
            method: "resources/list".to_string(),
            params: None,
        };

        if let Ok(response) = transport.send(resources_request).await {
            if let Some(result) = response.result {
                if let Some(resources) = result.get("resources") {
                    if let Ok(resources) =
                        serde_json::from_value::<Vec<McpResource>>(resources.clone())
                    {
                        self.state.resources = resources;
                        log::info!(
                            "Loaded {} resources from server",
                            self.state.resources.len()
                        );
                    }
                }
            }
        }

        // List prompts
        let prompts_request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: next_request_id(),
            method: "prompts/list".to_string(),
            params: None,
        };

        if let Ok(response) = transport.send(prompts_request).await {
            if let Some(result) = response.result {
                if let Some(prompts) = result.get("prompts") {
                    if let Ok(prompts) = serde_json::from_value::<Vec<McpPrompt>>(prompts.clone()) {
                        self.state.prompts = prompts;
                        log::info!("Loaded {} prompts from server", self.state.prompts.len());
                    }
                }
            }
        }

        Ok(())
    }

    /// Disconnect from the MCP server
    pub fn disconnect(&mut self) {
        log::info!("Disconnecting from MCP server: {}", self.config.name);
        self.transport = None;
        self.state = McpServerState::disconnected();
    }

    /// List available tools
    pub async fn list_tools(&self) -> Result<Vec<McpTool>, String> {
        if !self.state.connected {
            return Err("Not connected to server".to_string());
        }
        Ok(self.state.tools.clone())
    }

    /// Call a tool
    pub async fn call_tool(&self, params: ToolCallParams) -> Result<ToolCallResult, String> {
        if !self.state.connected {
            return Err("Not connected to server".to_string());
        }

        let transport = self.transport.as_ref().ok_or("Transport not initialized")?;

        log::info!("Calling tool: {}", params.name);

        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: next_request_id(),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": params.name,
                "arguments": params.arguments
            })),
        };

        match transport.send(request).await {
            Ok(response) => {
                if let Some(error) = response.error {
                    return Ok(ToolCallResult {
                        success: false,
                        result: String::new(),
                        error: Some(error.message),
                    });
                }

                let result = response
                    .result
                    .map(|r| serde_json::to_string(&r).unwrap_or_default())
                    .unwrap_or_default();

                Ok(ToolCallResult {
                    success: true,
                    result,
                    error: None,
                })
            }
            Err(e) => Ok(ToolCallResult {
                success: false,
                result: String::new(),
                error: Some(e),
            }),
        }
    }

    /// List available resources
    pub async fn list_resources(&self) -> Result<Vec<McpResource>, String> {
        if !self.state.connected {
            return Err("Not connected to server".to_string());
        }
        Ok(self.state.resources.clone())
    }

    /// Read a resource
    pub async fn read_resource(&self, uri: &str) -> Result<String, String> {
        if !self.state.connected {
            return Err("Not connected to server".to_string());
        }

        let transport = self.transport.as_ref().ok_or("Transport not initialized")?;

        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: next_request_id(),
            method: "resources/read".to_string(),
            params: Some(serde_json::json!({
                "uri": uri
            })),
        };

        let response = transport.send(request).await?;

        if let Some(error) = response.error {
            return Err(error.message);
        }

        response
            .result
            .and_then(|r| r.get("contents").cloned())
            .map(|c| serde_json::to_string(&c).unwrap_or_default())
            .ok_or_else(|| "No content in response".to_string())
    }

    /// List available prompts
    pub async fn list_prompts(&self) -> Result<Vec<McpPrompt>, String> {
        if !self.state.connected {
            return Err("Not connected to server".to_string());
        }
        Ok(self.state.prompts.clone())
    }

    /// Get a prompt with arguments
    pub async fn get_prompt(
        &self,
        name: &str,
        arguments: Option<serde_json::Value>,
    ) -> Result<String, String> {
        if !self.state.connected {
            return Err("Not connected to server".to_string());
        }

        let transport = self.transport.as_ref().ok_or("Transport not initialized")?;

        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: next_request_id(),
            method: "prompts/get".to_string(),
            params: Some(serde_json::json!({
                "name": name,
                "arguments": arguments
            })),
        };

        let response = transport.send(request).await?;

        if let Some(error) = response.error {
            return Err(error.message);
        }

        response
            .result
            .map(|r| serde_json::to_string(&r).unwrap_or_default())
            .ok_or_else(|| "No result in response".to_string())
    }

    /// Get server configuration
    pub fn config(&self) -> &McpServerConfig {
        &self.config
    }

    /// Get server state
    pub fn state(&self) -> &McpServerState {
        &self.state
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.state.connected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = McpServerConfig {
            name: "test".to_string(),
            url: Some("http://localhost:3000".to_string()),
            server_type: None,
            active: true,
            active_tools: vec![],
            active_prompts: vec![],
        };
        let client = McpClient::new(config);
        assert!(!client.is_connected());
    }

    #[test]
    fn test_request_id_generator() {
        let id1 = next_request_id();
        let id2 = next_request_id();
        assert_ne!(id1, id2);
    }
}
