//! Audio processing utilities for JARVIS
//!
//! This module provides audio capture, processing, and playback capabilities.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AudioContext, MediaStream};

/// Audio capture handler
pub struct AudioCapture {
    context: Option<AudioContext>,
    stream: Option<MediaStream>,
}

impl AudioCapture {
    /// Create a new audio capture instance
    pub fn new() -> Self {
        Self {
            context: None,
            stream: None,
        }
    }

    /// Initialize audio capture
    pub async fn init(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window found")?;
        let navigator = window.navigator();
        let media_devices = navigator
            .media_devices()
            .map_err(|_| "No media devices available")?;

        // Request microphone access
        let constraints = web_sys::MediaStreamConstraints::new();
        constraints.set_audio(&JsValue::from(true));
        constraints.set_video(&JsValue::from(false));

        let promise = media_devices.get_user_media_with_constraints(&constraints)?;
        let stream = wasm_bindgen_futures::JsFuture::from(promise)
            .await?
            .dyn_into::<MediaStream>()?;

        // Create audio context
        let context = AudioContext::new()?;

        self.context = Some(context);
        self.stream = Some(stream);

        Ok(())
    }

    /// Stop audio capture and release resources
    pub fn stop(&mut self) {
        if let Some(stream) = &self.stream {
            let tracks = stream.get_audio_tracks();
            for i in 0..tracks.length() {
                let track = tracks.get(i);
                if let Ok(track) = track.dyn_into::<web_sys::MediaStreamTrack>() {
                    track.stop();
                }
            }
        }
        self.stream = None;
        self.context = None;
    }
}

impl Default for AudioCapture {
    fn default() -> Self {
        Self::new()
    }
}

/// Resample audio from source sample rate to target sample rate
pub fn resample_audio(
    input: &[f32],
    _source_rate: u32,
    _target_rate: u32,
) -> Result<Vec<f32>, String> {
    // TODO: Implement proper resampling with rubato
    // For now, just return the input
    Ok(input.to_vec())
}

/// Convert audio to mel spectrogram for Whisper
pub fn audio_to_mel(_audio: &[f32], _sample_rate: u32) -> Result<Vec<f32>, String> {
    // TODO: Implement mel spectrogram conversion
    Err("Mel spectrogram conversion not yet implemented".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_capture_creation() {
        let capture = AudioCapture::new();
        assert!(capture.context.is_none());
        assert!(capture.stream.is_none());
    }
}
