//! JARVIS AI Module
//!
//! This module provides AI capabilities for the JARVIS application including:
//! - Speech-to-text (Whisper)
//! - Text generation (LLM)
//! - Text-to-speech
//! - Voice activity detection
//! - Image-to-text
//!
//! # Backend Support
//!
//! This crate uses the [Burn](https://burn.dev/) ML framework which supports multiple backends:
//! - `ndarray` (default): CPU backend, works everywhere in WASM
//! - `wgpu`: GPU backend using WebGPU (requires browser support)
//!
//! Enable the `wgpu` feature for GPU acceleration:
//! ```toml
//! jarvis-ai = { path = "../jarvis-ai", features = ["wgpu"] }
//! ```

// Required for wgpu backend due to deeply nested associated types
#![recursion_limit = "256"]

pub mod agent;
pub mod audio;
pub mod inference;
pub mod models;
pub mod types;

pub use agent::Agent;
pub use inference::{InferenceConfig, InferenceEngine, ModelState};
pub use models::{LoadProgress, ModelType};
pub use types::*;
