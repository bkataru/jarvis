# TODO and Roadmap

## Immediate Priorities

### P0 - Critical (Blocking Core Functionality)

- [ ] **Resolve Candle WASM Dependencies**
  - Current blocker: getrandom version conflict (0.2 vs 0.3)
  - Options:
    1. Wait for Candle to update dependencies
    2. Use alternative inference engine (e.g., Burn with ONNX)
    3. Fork Candle and patch dependencies
  - Related: Need to uncomment Candle dependencies in Cargo.toml

- [ ] **Implement Whisper Speech-to-Text**
  - Load Whisper tiny/base model
  - Audio preprocessing (resampling to 16kHz)
  - Mel spectrogram generation
  - Run inference in Web Worker
  - Stream results back to UI

- [ ] **Implement LLM Text Generation**
  - Load TinyLlama or Phi-2 model
  - Quantization support (Q4_K_M)
  - Token-by-token streaming
  - Tool calling integration
  - Conversation history management

### P1 - High Priority (Core Features)

- [ ] **Web Workers Integration**
  - Set up wasm-bindgen-rayon for parallelism
  - Move inference to background thread
  - Progress reporting for model loading
  - Handle worker errors gracefully

- [ ] **Voice Activity Detection**
  - Implement VAD algorithm
  - Real-time audio monitoring
  - Automatic start/stop recording
  - Visual feedback for voice detection

- [ ] **Model Caching**
  - Cache API integration
  - Progressive model loading
  - Version management
  - Clear old cached models

- [ ] **Audio Resampling**
  - Integrate rubato properly
  - Support various input sample rates
  - Mono channel conversion
  - Handle browser audio constraints

### P2 - Medium Priority (Enhanced Functionality)

- [ ] **Text-to-Speech**
  - Evaluate TTS options for WASM
  - Kokoro or similar lightweight TTS
  - Voice selection
  - Speech rate control

- [ ] **MCP Server Implementation**
  - Complete HTTP transport
  - Implement tool execution
  - Resource management
  - Prompt templates

- [ ] **Built-in MCP Servers**
  - Memories server (conversation history)
  - Camera server (image capture)
  - File system server (browser storage)
  - Calculator/utility servers

- [ ] **Image-to-Text**
  - CLIP or similar vision model
  - Image preprocessing
  - Batch processing
  - Integration with chat

### P3 - Lower Priority (Nice to Have)

- [ ] **UI Enhancements**
  - Better loading states
  - Model download progress
  - Error boundary components
  - Keyboard shortcuts
  - Accessibility improvements

- [ ] **Settings Page**
  - Model selection
  - Voice settings
  - Theme selection
  - Privacy controls

- [ ] **Conversation Management**
  - Save/load conversations
  - Export chat history
  - Search conversations
  - Delete conversations

## Technical Debt

### Code Quality

- [ ] Add comprehensive error handling
- [ ] Improve type safety
- [ ] Reduce code duplication
- [ ] Better logging and debugging
- [ ] Performance profiling

### Testing

- [ ] Set up WASM test harness (wasm-pack test)
- [ ] Unit tests for all modules
- [ ] Integration tests for workflows
- [ ] E2E tests with real models
- [ ] Performance benchmarks

### Documentation

- [ ] API documentation (rustdoc)
- [ ] User guide
- [ ] Developer guide
- [ ] Video tutorials
- [ ] FAQ section

## Future Enhancements

### Advanced Features

- [ ] **Multi-modal Conversations**
  - Mix of text, audio, and images
  - Context-aware responses
  - Memory across sessions

- [ ] **Custom Model Support**
  - User-provided models
  - Model format converter
  - Fine-tuning interface

- [ ] **Collaborative Features**
  - Share conversations
  - Collaborative sessions
  - Team workspaces

### Performance

- [ ] **WebGPU Acceleration**
  - When Candle adds support
  - Benchmark vs CPU
  - Fallback for unsupported browsers

- [ ] **Model Optimization**
  - More aggressive quantization
  - Model distillation
  - Pruning techniques

- [ ] **Streaming Optimizations**
  - Faster token generation
  - Predictive loading
  - Speculative decoding

### Platform Support

- [ ] **Mobile PWA**
  - Install as app
  - Offline support
  - Push notifications

- [ ] **Desktop App**
  - Tauri wrapper
  - Better performance
  - System integration

- [ ] **Browser Extension**
  - Quick access
  - Context menu integration
  - Page summarization

## Research & Exploration

### AI/ML

- [ ] Evaluate alternative inference engines
  - Burn with WebGPU
  - ONNX Runtime Web
  - TensorFlow.js
  - Custom WASM kernels

- [ ] Model compression techniques
  - Quantization-aware training
  - Knowledge distillation
  - Pruning

- [ ] On-device fine-tuning
  - LoRA/QLoRA in browser
  - User personalization
  - Privacy-preserving training

### Architecture

- [ ] Service Worker architecture
  - Background inference
  - Offline capability
  - Push updates

- [ ] WebRTC integration
  - Real-time collaboration
  - Peer-to-peer model sharing
  - Distributed inference

- [ ] IndexedDB optimization
  - Better model storage
  - Query performance
  - Migration strategies

## Community

### Documentation

- [ ] Create video tutorials
- [ ] Write blog posts
- [ ] Prepare conference talks
- [ ] Create demo videos

### Outreach

- [ ] Submit to Awesome Rust
- [ ] Post on Reddit r/rust
- [ ] Share on HackerNews
- [ ] Tweet about progress

### Maintenance

- [ ] Set up issue templates
- [ ] Create PR templates
- [ ] Add GitHub Actions
- [ ] Configure dependabot

## Notes

### Decision Log

**2024-12** - Chose Leptos over Yew
- Reasoning: Fine-grained reactivity, smaller bundles, better DX
- Trade-offs: Less mature ecosystem

**2024-12** - Candle over Burn
- Reasoning: Better model support, proven WASM examples
- Trade-offs: No WebGPU yet (CPU only)

**2024-12** - Postponed Candle integration
- Reasoning: Dependency conflicts blocking compilation
- Plan: Implement UI first, add inference when ready

### Open Questions

1. Should we support multiple inference backends?
2. What's the minimum browser requirements?
3. How to handle model updates?
4. Privacy policy for model downloads?
5. Licensing for bundled models?

### References

- [Candle WASM Examples](https://huggingface.co/spaces/lmz/candle-whisper)
- [WebLLM Benchmarks](https://github.com/mlc-ai/web-llm)
- [Transformers.js](https://huggingface.co/docs/transformers.js)
- [Leptos Book](https://book.leptos.dev/)
