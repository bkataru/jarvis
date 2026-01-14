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

/// Download a model from HuggingFace
#[cfg(not(target_arch = "wasm32"))]
pub async fn download_model(
    model_type: ModelType,
    on_progress: impl Fn(LoadProgress),
) -> Result<Vec<u8>, String> {
    use std::io::Read;
    use reqwest::Client;

    let client = Client::new();
    let url = format!("https://huggingface.co/{}/resolve/main/model.safetensors", model_type.model_name());
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let total_bytes = response.content_length().unwrap_or(0);
    let mut loaded_bytes = 0;
    let mut data = Vec::with_capacity(total_bytes as usize);
    let mut stream = response.bytes_stream();
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        loaded_bytes += chunk.len() as u64;
        data.extend_from_slice(&chunk);
        on_progress(LoadProgress::new(loaded_bytes, total_bytes));
    }
    
    Ok(data)
}

/// Download a model from HuggingFace (WASM version)
#[cfg(target_arch = "wasm32")]
pub async fn download_model(
    model_type: ModelType,
    on_progress: impl Fn(LoadProgress),
) -> Result<Vec<u8>, String> {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, Response};
    
    let window = web_sys::window().ok_or("No window found")?;
    let url = format!("https://huggingface.co/{}/resolve/main/model.safetensors", model_type.model_name());
    
    let opts = RequestInit::new();
    opts.set_method("GET");
    
    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|_| "Failed to create request")?;
    
    let promise = window.fetch_with_request(&request);
    let response = JsFuture::from(promise)
        .await
        .map_err(|_| "Fetch failed")?
        .dyn_into::<Response>()
        .map_err(|_| "Not a response")?;
    
    let content_length = response
        .headers()
        .get("content-length")
        .ok()
        .flatten()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    
    let array_buffer_promise = response.array_buffer().map_err(|_| "Failed to get array buffer")?;
    let array_buffer = JsFuture::from(array_buffer_promise)
        .await
        .map_err(|_| "Failed to get array buffer data")?;
    
    let bytes = js_sys::Uint8Array::new(&array_buffer);
    let mut data = vec![0; bytes.length() as usize];
    bytes.copy_to(&mut data);
    
    on_progress(LoadProgress::new(data.len() as u64, content_length));
    
    Ok(data)
}
