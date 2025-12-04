//! JARVIS AI Module
//!
//! This module provides AI capabilities for the JARVIS application including:
//! - Speech-to-text (Whisper)
//! - Text generation (LLM)
//! - Text-to-speech
//! - Voice activity detection
//! - Image-to-text

pub mod agent;
pub mod audio;
pub mod inference;
pub mod models;
pub mod types;

pub use agent::Agent;
pub use types::*;
