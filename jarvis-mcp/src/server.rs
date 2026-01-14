//! Built-in MCP servers

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, HtmlVideoElement, MediaStream, MediaStreamConstraints,
};

/// Memories server for storing conversation context
pub struct MemoriesServer {
    memories: Vec<String>,
    max_memories: usize,
}

impl MemoriesServer {
    /// Create a new memories server
    pub fn new() -> Self {
        Self {
            memories: Vec::new(),
            max_memories: 100,
        }
    }

    /// Create with custom max memories limit
    pub fn with_limit(max_memories: usize) -> Self {
        Self {
            memories: Vec::new(),
            max_memories,
        }
    }

    /// Store a memory
    pub fn store(&mut self, memory: String) {
        if self.memories.len() >= self.max_memories {
            self.memories.remove(0); // Remove oldest
        }
        self.memories.push(memory);
    }

    /// Retrieve all memories
    pub fn retrieve(&self) -> &[String] {
        &self.memories
    }

    /// Retrieve most recent N memories
    pub fn retrieve_recent(&self, count: usize) -> &[String] {
        let start = self.memories.len().saturating_sub(count);
        &self.memories[start..]
    }

    /// Search memories containing keyword
    pub fn search(&self, keyword: &str) -> Vec<&String> {
        let keyword_lower = keyword.to_lowercase();
        self.memories
            .iter()
            .filter(|m| m.to_lowercase().contains(&keyword_lower))
            .collect()
    }

    /// Clear all memories
    pub fn clear(&mut self) {
        self.memories.clear();
    }

    /// Get memory count
    pub fn count(&self) -> usize {
        self.memories.len()
    }
}

impl Default for MemoriesServer {
    fn default() -> Self {
        Self::new()
    }
}

/// Camera server for taking pictures
pub struct CameraServer {
    video: Option<HtmlVideoElement>,
    stream: Option<MediaStream>,
    canvas: Option<HtmlCanvasElement>,
}

impl CameraServer {
    /// Create a new camera server
    pub fn new() -> Self {
        Self {
            video: None,
            stream: None,
            canvas: None,
        }
    }

    /// Initialize camera access
    pub async fn init(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window found")?;
        let document = window.document().ok_or("No document found")?;
        let navigator = window.navigator();
        
        // Get media devices
        let media_devices = navigator
            .media_devices()
            .map_err(|_| "No media devices available")?;
        
        // Request camera access with video only
        let constraints = MediaStreamConstraints::new();
        constraints.set_video(&JsValue::from(true));
        constraints.set_audio(&JsValue::from(false));
        
        let promise = media_devices.get_user_media_with_constraints(&constraints)?;
        let stream = wasm_bindgen_futures::JsFuture::from(promise)
            .await?
            .dyn_into::<MediaStream>()?;
        
        // Create hidden video element
        let video = document
            .create_element("video")?
            .dyn_into::<HtmlVideoElement>()?;
        video.set_autoplay(true);
        video.set_muted(true);
        video.set_attribute("playsinline", "true")?;
        video.style().set_property("display", "none")?;
        
        // Attach stream to video
        video.set_src_object(Some(&stream));
        
        // Wait for video to be ready
        let promise = video.play()?;
        wasm_bindgen_futures::JsFuture::from(promise).await?;
        
        // Create canvas for capture
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;
        
        self.video = Some(video);
        self.stream = Some(stream);
        self.canvas = Some(canvas);
        
        log::info!("Camera initialized successfully");
        Ok(())
    }

    /// Take a picture and return as PNG bytes
    pub async fn take_picture(&self) -> Result<Vec<u8>, String> {
        let video = self.video.as_ref()
            .ok_or("Camera not initialized")?;
        let canvas = self.canvas.as_ref()
            .ok_or("Canvas not initialized")?;
        
        // Set canvas size to video dimensions
        let width = video.video_width();
        let height = video.video_height();
        
        if width == 0 || height == 0 {
            return Err("Video dimensions not available".to_string());
        }
        
        canvas.set_width(width);
        canvas.set_height(height);
        
        // Draw video frame to canvas
        let context = canvas
            .get_context("2d")
            .map_err(|_| "Failed to get 2D context")?
            .ok_or("No 2D context available")?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .map_err(|_| "Failed to cast to CanvasRenderingContext2d")?;
        
        context
            .draw_image_with_html_video_element(video, 0.0, 0.0)
            .map_err(|_| "Failed to draw video to canvas")?;
        
        // Get image data as PNG data URL
        let data_url = canvas
            .to_data_url_with_type("image/png")
            .map_err(|_| "Failed to get data URL")?;
        
        // Convert data URL to bytes
        // Format: data:image/png;base64,<base64data>
        let base64_data = data_url
            .strip_prefix("data:image/png;base64,")
            .ok_or("Invalid data URL format")?;
        
        // Decode base64 using web APIs
        let window = web_sys::window().ok_or("No window")?;
        let decoded = window
            .atob(base64_data)
            .map_err(|_| "Failed to decode base64")?;
        
        // Convert to bytes
        let bytes: Vec<u8> = decoded.chars().map(|c| c as u8).collect();
        
        log::info!("Captured image: {}x{}, {} bytes", width, height, bytes.len());
        Ok(bytes)
    }

    /// Take a picture and return as base64 string
    pub async fn take_picture_base64(&self) -> Result<String, String> {
        let video = self.video.as_ref()
            .ok_or("Camera not initialized")?;
        let canvas = self.canvas.as_ref()
            .ok_or("Canvas not initialized")?;
        
        // Set canvas size to video dimensions
        let width = video.video_width();
        let height = video.video_height();
        
        if width == 0 || height == 0 {
            return Err("Video dimensions not available".to_string());
        }
        
        canvas.set_width(width);
        canvas.set_height(height);
        
        // Draw video frame to canvas
        let context = canvas
            .get_context("2d")
            .map_err(|_| "Failed to get 2D context")?
            .ok_or("No 2D context available")?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .map_err(|_| "Failed to cast to CanvasRenderingContext2d")?;
        
        context
            .draw_image_with_html_video_element(video, 0.0, 0.0)
            .map_err(|_| "Failed to draw video to canvas")?;
        
        // Get image data as data URL
        canvas
            .to_data_url_with_type("image/png")
            .map_err(|_| "Failed to get data URL".to_string())
    }

    /// Stop camera and release resources
    pub fn stop(&mut self) {
        if let Some(stream) = &self.stream {
            let tracks = stream.get_video_tracks();
            for i in 0..tracks.length() {
                let track = tracks.get(i);
                if let Ok(track) = track.dyn_into::<web_sys::MediaStreamTrack>() {
                    track.stop();
                }
            }
        }
        self.video = None;
        self.stream = None;
        self.canvas = None;
        log::info!("Camera stopped");
    }

    /// Check if camera is active
    pub fn is_active(&self) -> bool {
        self.video.is_some() && self.stream.is_some()
    }
}

impl Default for CameraServer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for CameraServer {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memories_server() {
        let mut server = MemoriesServer::new();
        server.store("Test memory".to_string());
        assert_eq!(server.retrieve().len(), 1);
        assert_eq!(server.count(), 1);
        server.clear();
        assert_eq!(server.retrieve().len(), 0);
    }

    #[test]
    fn test_memories_server_limit() {
        let mut server = MemoriesServer::with_limit(3);
        server.store("First".to_string());
        server.store("Second".to_string());
        server.store("Third".to_string());
        server.store("Fourth".to_string());
        
        assert_eq!(server.count(), 3);
        assert_eq!(server.retrieve()[0], "Second");
    }

    #[test]
    fn test_memories_search() {
        let mut server = MemoriesServer::new();
        server.store("Meeting with John".to_string());
        server.store("Lunch appointment".to_string());
        server.store("Meeting notes".to_string());
        
        let results = server.search("meeting");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_memories_recent() {
        let mut server = MemoriesServer::new();
        server.store("First".to_string());
        server.store("Second".to_string());
        server.store("Third".to_string());
        
        let recent = server.retrieve_recent(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0], "Second");
        assert_eq!(recent[1], "Third");
    }

    #[test]
    fn test_camera_server_creation() {
        let camera = CameraServer::new();
        assert!(!camera.is_active());
    }
}
