# Architecture Documentation

## Overview

JARVIS is built as a modular Rust application compiled to WebAssembly, designed to run entirely in the browser with client-side AI inference.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Browser (WASM)                          │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │             Leptos UI (jarvis-app)                  │    │
│  │  - Reactive components                              │    │
│  │  - Routing (Home, Chat, MCP)                       │    │
│  │  - State management                                 │    │
│  └────────────┬──────────────────────┬─────────────────┘    │
│               │                      │                       │
│  ┌────────────▼────────────┐   ┌────▼────────────────┐     │
│  │  AI Engine (jarvis-ai)   │   │  MCP (jarvis-mcp)   │     │
│  │  - Candle inference      │   │  - HTTP transport   │     │
│  │  - Audio processing      │   │  - Server clients   │     │
│  │  - Model management      │   │  - Tool calling     │     │
│  └─────────────────────────┘   └─────────────────────┘     │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Module Breakdown

### jarvis-app

The main application crate containing the UI and user-facing components.

**Key Components:**
- `App` - Root component with router
- `HomePage` - Voice interface with JARVIS ring
- `ChatPage` - Text-based chat interface
- `McpPage` - MCP server configuration

**Responsibilities:**
- User interface rendering
- User input handling
- Navigation/routing
- Local storage management
- Coordinating AI and MCP modules

### jarvis-ai

The AI inference engine responsible for all machine learning operations.

**Key Components:**
- `Agent` - Manages JARVIS personality and prompts
- `InferenceEngine` - Loads and runs ML models
- `AudioCapture` - Handles microphone input
- Audio processing utilities

**Responsibilities:**
- Loading quantized models (Whisper, LLM)
- Speech-to-text transcription
- Text generation
- Audio preprocessing (resampling, mel spectrograms)
- Voice activity detection

**Technologies:**
- Candle for model inference
- rubato for audio resampling
- web-sys for Web Audio API access

### jarvis-mcp

Model Context Protocol client for external tool integration.

**Key Components:**
- `McpClient` - Main client for MCP servers
- `HttpTransport` - HTTP-based communication
- Built-in servers (memories, camera)

**Responsibilities:**
- Connecting to MCP servers
- Listing available tools/resources
- Executing tool calls
- Managing server state

## Data Flow

### Chat Message Flow

```
User Input → ChatPage → InferenceEngine → LLM → Response → ChatPage → Display
```

### Voice Interaction Flow

```
Mic → AudioCapture → Whisper → Text → LLM → Response → TTS → Speaker
```

### MCP Tool Call Flow

```
LLM detects tool need → McpClient → HTTP Transport → MCP Server → Result → LLM
```

## State Management

State is managed using Leptos signals (reactive primitives):

- **UI State**: Component-local signals for form inputs, loading states
- **Conversation State**: Messages array in ChatPage
- **MCP State**: Server configurations in McpPage
- **Audio State**: Recording status, listening state in HomePage

## Browser APIs Used

- **Web Audio API**: Microphone capture, audio playback
- **Web Workers**: Off-thread model inference (planned)
- **Cache API**: Model caching (planned)
- **Local Storage**: User preferences, server configurations
- **WebGPU**: GPU-accelerated inference (future, when Candle supports it)

## Performance Considerations

### Model Loading

Models are large (75 MB - 1.5 GB). Strategies:
- Progressive loading with progress feedback
- Cache API for persistent storage
- Lazy loading (load on first use)

### Inference Performance

- CPU-only inference in current Candle WASM
- 2-3x real-time for Whisper tiny/base
- Smaller quantized models (Q4_K_M) for LLMs
- Web Workers to prevent UI blocking (planned)

### Memory Management

- WASM limited to 4GB by default
- Explicit model unloading when switching
- Streaming inference for long outputs
- Buffer pooling for audio processing

## Security

- All inference runs locally (no data sent to servers)
- MCP servers can be HTTP-only (user must trust endpoints)
- CSP headers for production deployment
- CORS considerations for MCP endpoints

## Future Enhancements

1. **WebGPU Acceleration**: When Candle adds WASM WebGPU support
2. **Streaming Inference**: Token-by-token generation
3. **Model Sharding**: Split large models across multiple requests
4. **Service Worker**: Offline support and background processing
5. **IndexedDB**: Better model storage than Cache API
6. **SharedArrayBuffer**: For multi-threaded inference
