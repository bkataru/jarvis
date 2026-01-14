//! JARVIS MCP (Model Context Protocol) Module
//!
//! This module provides MCP server integration capabilities including:
//! - HTTP transport for remote MCP servers
//! - Built-in MCP servers (memories, camera)
//! - Tool calling and execution
//! - Resource management

pub mod client;
pub mod server;
pub mod transport;
pub mod types;

pub use client::McpClient;
pub use types::*;
