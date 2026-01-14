//! Audio processing utilities for JARVIS
//!
//! This module provides audio capture, processing, and playback capabilities
//! including resampling and mel spectrogram conversion for Whisper.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AudioContext, MediaStream};

/// Whisper expects 16kHz sample rate
pub const WHISPER_SAMPLE_RATE: u32 = 16000;
/// Number of mel filterbanks for Whisper
pub const N_MEL_BINS: usize = 80;
/// FFT window size
pub const N_FFT: usize = 400;
/// Hop length between frames
pub const HOP_LENGTH: usize = 160;
/// Chunk length in samples (30 seconds at 16kHz)
pub const CHUNK_LENGTH: usize = 480000;

/// Audio capture handler
pub struct AudioCapture {
    context: Option<AudioContext>,
    stream: Option<MediaStream>,
    sample_rate: u32,
}

impl AudioCapture {
    /// Create a new audio capture instance
    pub fn new() -> Self {
        Self {
            context: None,
            stream: None,
            sample_rate: 0,
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
        self.sample_rate = context.sample_rate() as u32;

        self.context = Some(context);
        self.stream = Some(stream);

        Ok(())
    }

    /// Get the current sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
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
        self.sample_rate = 0;
    }

    /// Check if audio capture is active
    pub fn is_active(&self) -> bool {
        self.stream.is_some() && self.context.is_some()
    }
}

impl Default for AudioCapture {
    fn default() -> Self {
        Self::new()
    }
}

/// Resample audio from source sample rate to target sample rate using linear interpolation
///
/// # Arguments
/// * `input` - Input audio samples
/// * `source_rate` - Source sample rate in Hz
/// * `target_rate` - Target sample rate in Hz
///
/// # Returns
/// Resampled audio samples
pub fn resample_audio(
    input: &[f32],
    source_rate: u32,
    target_rate: u32,
) -> Result<Vec<f32>, String> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    if source_rate == target_rate {
        return Ok(input.to_vec());
    }

    if source_rate == 0 || target_rate == 0 {
        return Err("Sample rate cannot be zero".to_string());
    }

    let ratio = source_rate as f64 / target_rate as f64;
    let output_len = ((input.len() as f64) / ratio).ceil() as usize;
    let mut output = Vec::with_capacity(output_len);

    for i in 0..output_len {
        let src_pos = i as f64 * ratio;
        let src_idx = src_pos.floor() as usize;
        let frac = src_pos - src_idx as f64;

        let sample = if src_idx + 1 < input.len() {
            // Linear interpolation between samples
            input[src_idx] as f64 * (1.0 - frac) + input[src_idx + 1] as f64 * frac
        } else if src_idx < input.len() {
            input[src_idx] as f64
        } else {
            0.0
        };

        output.push(sample as f32);
    }

    Ok(output)
}

/// Create mel filterbank matrix
///
/// # Arguments
/// * `n_mels` - Number of mel bins
/// * `n_fft` - FFT size
/// * `sample_rate` - Sample rate in Hz
fn create_mel_filterbank(n_mels: usize, n_fft: usize, sample_rate: u32) -> Vec<Vec<f32>> {
    let n_freqs = n_fft / 2 + 1;
    let sample_rate = sample_rate as f64;

    // Convert Hz to mel scale
    let hz_to_mel = |hz: f64| -> f64 { 2595.0 * (1.0 + hz / 700.0).ln() / 10.0_f64.ln() };

    // Convert mel to Hz
    let mel_to_hz = |mel: f64| -> f64 { 700.0 * (10.0_f64.powf(mel / 2595.0) - 1.0) };

    let mel_min = hz_to_mel(0.0);
    let mel_max = hz_to_mel(sample_rate / 2.0);

    // Create mel points
    let mel_points: Vec<f64> = (0..=n_mels + 1)
        .map(|i| mel_min + (mel_max - mel_min) * i as f64 / (n_mels + 1) as f64)
        .collect();

    // Convert back to Hz
    let hz_points: Vec<f64> = mel_points.iter().map(|&m| mel_to_hz(m)).collect();

    // Convert to FFT bin indices
    let bin_points: Vec<usize> = hz_points
        .iter()
        .map(|&hz| ((n_fft as f64 + 1.0) * hz / sample_rate).floor() as usize)
        .collect();

    // Create filterbank
    let mut filterbank = vec![vec![0.0f32; n_freqs]; n_mels];

    for i in 0..n_mels {
        let left = bin_points[i];
        let center = bin_points[i + 1];
        let right = bin_points[i + 2];

        // Rising edge
        if center > left {
            let end = center.min(n_freqs);
            for (j, v) in filterbank[i].iter_mut().enumerate().take(end).skip(left) {
                *v = (j - left) as f32 / (center - left) as f32;
            }
        }

        // Falling edge
        if right > center {
            let end = right.min(n_freqs);
            for (j, v) in filterbank[i].iter_mut().enumerate().take(end).skip(center) {
                *v = (right - j) as f32 / (right - center) as f32;
            }
        }
    }

    filterbank
}

/// Compute Short-Time Fourier Transform magnitude spectrum
fn compute_stft_magnitude(audio: &[f32], n_fft: usize, hop_length: usize) -> Vec<Vec<f32>> {
    let n_freqs = n_fft / 2 + 1;
    let n_frames = (audio.len().saturating_sub(n_fft)) / hop_length + 1;

    if n_frames == 0 {
        return vec![];
    }

    // Create Hann window
    let window: Vec<f32> = (0..n_fft)
        .map(|i| 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / n_fft as f32).cos()))
        .collect();

    let mut magnitudes = Vec::with_capacity(n_frames);

    for frame in 0..n_frames {
        let start = frame * hop_length;
        let end = (start + n_fft).min(audio.len());

        // Apply window and zero-pad if necessary
        let windowed: Vec<f32> = (0..n_fft)
            .map(|i| {
                if start + i < end {
                    audio[start + i] * window[i]
                } else {
                    0.0
                }
            })
            .collect();

        // Compute DFT (simplified - real FFT would be more efficient)
        let mut frame_magnitudes = Vec::with_capacity(n_freqs);
        for k in 0..n_freqs {
            let mut real = 0.0f64;
            let mut imag = 0.0f64;

            for (n, &sample) in windowed.iter().enumerate() {
                let angle = -2.0 * std::f64::consts::PI * k as f64 * n as f64 / n_fft as f64;
                real += sample as f64 * angle.cos();
                imag += sample as f64 * angle.sin();
            }

            let magnitude = (real * real + imag * imag).sqrt() as f32;
            frame_magnitudes.push(magnitude);
        }

        magnitudes.push(frame_magnitudes);
    }

    magnitudes
}

/// Convert audio waveform to log mel spectrogram for Whisper
///
/// # Arguments
/// * `audio` - Audio samples (should be 16kHz mono)
/// * `sample_rate` - Sample rate of the audio
///
/// # Returns
/// Log mel spectrogram as a flat vector (n_mels x n_frames)
pub fn audio_to_mel(audio: &[f32], sample_rate: u32) -> Result<Vec<f32>, String> {
    if audio.is_empty() {
        return Err("Empty audio input".to_string());
    }

    // Resample to Whisper's expected rate if necessary
    let audio = if sample_rate != WHISPER_SAMPLE_RATE {
        resample_audio(audio, sample_rate, WHISPER_SAMPLE_RATE)?
    } else {
        audio.to_vec()
    };

    // Pad or truncate to chunk length
    let mut padded_audio = audio;
    if padded_audio.len() < CHUNK_LENGTH {
        padded_audio.resize(CHUNK_LENGTH, 0.0);
    } else if padded_audio.len() > CHUNK_LENGTH {
        padded_audio.truncate(CHUNK_LENGTH);
    }

    // Compute STFT magnitude spectrum
    let stft = compute_stft_magnitude(&padded_audio, N_FFT, HOP_LENGTH);

    if stft.is_empty() {
        return Err("Failed to compute STFT".to_string());
    }

    // Create mel filterbank
    let filterbank = create_mel_filterbank(N_MEL_BINS, N_FFT, WHISPER_SAMPLE_RATE);

    // Apply mel filterbank and convert to log scale
    let mut mel_spec = Vec::with_capacity(N_MEL_BINS * stft.len());

    for frame in &stft {
        for filter in &filterbank {
            let mut sum = 0.0f32;
            for (i, &f) in filter.iter().enumerate() {
                if i < frame.len() {
                    // Apply mel filter (using power spectrum)
                    sum += f * frame[i] * frame[i];
                }
            }
            // Convert to log scale with small epsilon to avoid log(0)
            let log_mel = (sum.max(1e-10)).ln();
            mel_spec.push(log_mel);
        }
    }

    // Normalize to match Whisper's expected range
    let max_val = mel_spec.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let min_val = max_val - 8.0; // Dynamic range of 8 (about 80dB)

    for val in &mut mel_spec {
        *val = ((*val - min_val) / (max_val - min_val)).clamp(0.0, 1.0) * 2.0 - 1.0;
    }

    Ok(mel_spec)
}

/// Normalize audio samples to [-1, 1] range
pub fn normalize_audio(audio: &mut [f32]) {
    let max_abs = audio.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
    if max_abs > 0.0 {
        for sample in audio.iter_mut() {
            *sample /= max_abs;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_capture_creation() {
        let capture = AudioCapture::new();
        assert!(capture.context.is_none());
        assert!(capture.stream.is_none());
        assert!(!capture.is_active());
    }

    #[test]
    fn test_resample_same_rate() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = resample_audio(&input, 16000, 16000).unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn test_resample_downsample() {
        let input: Vec<f32> = (0..100).map(|i| i as f32).collect();
        let result = resample_audio(&input, 48000, 16000).unwrap();
        // Downsampling 3:1, should get roughly 1/3 of samples
        assert!(result.len() < input.len());
    }

    #[test]
    fn test_resample_empty() {
        let input: Vec<f32> = vec![];
        let result = resample_audio(&input, 48000, 16000).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_audio_to_mel_empty() {
        let result = audio_to_mel(&[], 16000);
        assert!(result.is_err());
    }

    #[test]
    fn test_normalize_audio() {
        let mut audio = vec![0.5, -1.0, 0.25, 0.0];
        normalize_audio(&mut audio);
        assert_eq!(audio[1], -1.0); // Max should be normalized to +-1
        assert_eq!(audio[0], 0.5); // Others should scale proportionally
    }
}
