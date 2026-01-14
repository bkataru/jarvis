//! Model definitions and loading utilities

use serde::{Deserialize, Serialize};

/// Available models for inference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelType {
    WhisperTiny,
    WhisperBase,
    WhisperSmall,
    Phi2,
    TinyLlama,
}

impl ModelType {
    /// Get the model name for downloading
    pub fn model_name(&self) -> &str {
        match self {
            ModelType::WhisperTiny => "openai/whisper-tiny.en",
            ModelType::WhisperBase => "openai/whisper-base.en",
            ModelType::WhisperSmall => "openai/whisper-small",
            ModelType::Phi2 => "microsoft/phi-2",
            ModelType::TinyLlama => "TinyLlama/TinyLlama-1.1B-Chat-v1.0",
        }
    }

    /// Get estimated model size in MB
    pub fn size_mb(&self) -> u32 {
        match self {
            ModelType::WhisperTiny => 75,
            ModelType::WhisperBase => 142,
            ModelType::WhisperSmall => 466,
            ModelType::Phi2 => 1500,
            ModelType::TinyLlama => 600,
        }
    }

    /// Get estimated RAM usage in MB
    pub fn ram_mb(&self) -> u32 {
        match self {
            ModelType::WhisperTiny => 390,
            ModelType::WhisperBase => 500,
            ModelType::WhisperSmall => 1000,
            ModelType::Phi2 => 2000,
            ModelType::TinyLlama => 800,
        }
    }
}

/// Model loading progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadProgress {
    pub loaded_bytes: u64,
    pub total_bytes: u64,
    pub percentage: f32,
}

impl LoadProgress {
    /// Create a new progress tracker
    pub fn new(loaded: u64, total: u64) -> Self {
        let percentage = if total > 0 {
            (loaded as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        Self {
            loaded_bytes: loaded,
            total_bytes: total,
            percentage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_names() {
        assert_eq!(ModelType::WhisperTiny.model_name(), "openai/whisper-tiny.en");
        assert_eq!(ModelType::Phi2.model_name(), "microsoft/phi-2");
    }

    #[test]
    fn test_load_progress() {
        let progress = LoadProgress::new(50, 100);
        assert_eq!(progress.percentage, 50.0);
    }
}
