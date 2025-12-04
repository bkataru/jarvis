//! Built-in MCP servers

/// Memories server for storing conversation context
pub struct MemoriesServer {
    memories: Vec<String>,
}

impl MemoriesServer {
    /// Create a new memories server
    pub fn new() -> Self {
        Self {
            memories: Vec::new(),
        }
    }

    /// Store a memory
    pub fn store(&mut self, memory: String) {
        self.memories.push(memory);
    }

    /// Retrieve all memories
    pub fn retrieve(&self) -> &[String] {
        &self.memories
    }

    /// Clear all memories
    pub fn clear(&mut self) {
        self.memories.clear();
    }
}

impl Default for MemoriesServer {
    fn default() -> Self {
        Self::new()
    }
}

/// Camera server for taking pictures
pub struct CameraServer;

impl CameraServer {
    /// Create a new camera server
    pub fn new() -> Self {
        Self
    }

    /// Take a picture
    pub async fn take_picture(&self) -> Result<Vec<u8>, String> {
        // TODO: Implement camera capture
        Err("Camera capture not yet implemented".to_string())
    }
}

impl Default for CameraServer {
    fn default() -> Self {
        Self::new()
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
        server.clear();
        assert_eq!(server.retrieve().len(), 0);
    }
}
