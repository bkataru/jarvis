# JARVIS - Rust Edition

A modern AI assistant web application built entirely in Rust and compiled to WebAssembly. This is a recreation of the original [nico-martin/jarvis](https://github.com/nico-martin/jarvis) TypeScript/Preact application, featuring voice interaction, real-time conversation, and MCP (Model Context Protocol) server integration.

## ✨ Features

- **🎤 Voice Interaction**: Voice activity detection, speech-to-text (Whisper), and text-to-speech capabilities
- **💬 Real-time Chat**: Interactive conversation interface with AI assistant
- **🔌 MCP Server Integration**: Connect and interact with Model Context Protocol servers
- **🖼️ Image-to-Text**: Convert images to text descriptions (planned)
- **🎨 Multi-modal Interface**: Switch between voice-activated JARVIS mode and traditional chat
- **🦀 Pure Rust/WASM**: Runs locally in your browser with no external dependencies
- **⚡ WebGPU Ready**: Prepared for GPU-accelerated inference (when Candle adds support)

## 🏗️ Architecture

This application is built with a modular architecture:

```
jarvis/
├── jarvis-app/     # Main Leptos-based web application
├── jarvis-ai/      # AI inference engine (Candle-based)
└── jarvis-mcp/     # Model Context Protocol client
```

### Tech Stack

- **Frontend Framework**: [Leptos](https://leptos.dev/) - Reactive Rust web framework
- **AI/ML Engine**: [Candle](https://github.com/huggingface/candle) - ML framework in Rust
- **Build Tool**: [Trunk](https://trunkrs.dev/) - WASM web application bundler
- **Styling**: Tailwind CSS (via CDN)
- **MCP**: Custom implementation based on Model Context Protocol
- **Audio Processing**: rubato for resampling, web-sys for capture

### Models

The application uses quantized models optimized for browser deployment:

- **Speech-to-Text**: Whisper Tiny/Base (75-142 MB)
- **Text Generation**: TinyLlama 1.1B or Phi-2 (600-1500 MB quantized)
- **Voice Activity Detection**: Custom implementation

## 🚀 Getting Started

### Prerequisites

- **Rust** (1.70 or later) - [Install Rust](https://rustup.rs/)
- **Trunk** - WASM bundler
  ```bash
  cargo install trunk
  ```
- **wasm32 target**:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- **Modern Browser** with WebAssembly support (Chrome 113+, Firefox 141+, Safari 26+)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/bkataru/jarvis.git
   cd jarvis
   ```

2. Build and run the development server:
   ```bash
   trunk serve
   ```

3. Open your browser and navigate to `http://localhost:8080`

### Building for Production

```bash
trunk build --release
```

The optimized build will be in the `dist/` directory.

## 📖 Usage

### JARVIS Voice Mode

1. Click the animated ring on the home page to activate voice interaction
2. Speak naturally to interact with the AI assistant
3. Visual feedback indicates when JARVIS is listening and responding

### Chat Mode

1. Click "Chat Mode" to switch to text-based interaction
2. Type messages in the input field
3. Press Enter or click Send to submit

### MCP Settings

1. Navigate to "MCP Settings" to configure Model Context Protocol servers
2. Add HTTP-based MCP servers by providing name and URL
3. Enable/disable servers and specific tools as needed

## 🧪 Testing

Run the test suite:

```bash
cargo test --all
```

Run tests for a specific crate:

```bash
cargo test -p jarvis-ai
cargo test -p jarvis-mcp
cargo test -p jarvis-app
```

## 📝 Development

### Project Structure

```
jarvis-app/src/
├── components/     # Reusable UI components
│   ├── button.rs
│   ├── jarvis_ring.rs
│   └── message.rs
├── pages/          # Page components
│   ├── home.rs     # Voice interface
│   ├── chat.rs     # Chat interface
│   └── mcp.rs      # MCP settings
├── state/          # Application state
├── utils/          # Utility functions
└── lib.rs          # App entry point

jarvis-ai/src/
├── agent.rs        # JARVIS agent & prompts
├── audio.rs        # Audio capture & processing
├── inference.rs    # Model inference engine
├── models.rs       # Model definitions
├── types.rs        # Type definitions
└── lib.rs          # Module exports

jarvis-mcp/src/
├── client.rs       # MCP client
├── server.rs       # Built-in MCP servers
├── transport.rs    # HTTP transport
├── types.rs        # MCP type definitions
└── lib.rs          # Module exports
```

### Adding New Features

1. **New UI Component**: Add to `jarvis-app/src/components/`
2. **New Page**: Add to `jarvis-app/src/pages/` and update routing in `lib.rs`
3. **AI Feature**: Implement in `jarvis-ai/src/`
4. **MCP Server**: Add to `jarvis-mcp/src/server.rs`

### Code Style

This project uses standard Rust formatting:

```bash
cargo fmt --all
```

And linting:

```bash
cargo clippy --all --all-targets
```

## 🛣️ Roadmap

- [x] Basic application structure
- [x] Leptos UI framework integration
- [x] MCP client foundation
- [ ] Candle model loading
- [ ] Whisper speech-to-text
- [ ] LLM text generation
- [ ] Voice activity detection
- [ ] Text-to-speech synthesis
- [ ] WebGPU acceleration (when available in Candle)
- [ ] Progressive model loading
- [ ] IndexedDB/Cache API for models
- [ ] Web Workers for inference
- [ ] Image-to-text capabilities
- [ ] Full MCP server support

## 🙏 Acknowledgements

This project is a Rust recreation of the original JARVIS application:

- **Original Author**: [Nicolas Martin](https://github.com/nico-martin) ([@nico-martin](https://github.com/nico-martin))
- **Original Repository**: [nico-martin/jarvis](https://github.com/nico-martin/jarvis)
- **License**: MIT

### Dependencies & Frameworks

- **[Leptos](https://leptos.dev/)** - Reactive web framework for Rust
- **[Candle](https://github.com/huggingface/candle)** - ML framework by Hugging Face
- **[Trunk](https://trunkrs.dev/)** - WASM web application bundler
- **[wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)** - JavaScript/Rust interop
- **[web-sys](https://rustwasm.github.io/wasm-bindgen/web-sys/)** - Web API bindings
- **[rubato](https://github.com/HEnquist/rubato)** - Audio resampling
- **[serde](https://serde.rs/)** - Serialization framework

### Inspiration & References

- **Research Report**: Based on "Building browser-based AI inference in pure Rust/WASM"
- **Candle WASM Examples**: [Hugging Face Spaces](https://huggingface.co/collections/radames/candle-wasm-examples-650898dee13ff96230ce3e1f)
- **Model Context Protocol**: [MCP Specification](https://modelcontextprotocol.io/)
- **Transformers.js**: Reference for browser-based AI

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

Copyright (c) 2024 Bhargav Kataru

Based on JARVIS by Nicolas Martin (mail@nico.dev)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📧 Contact

Bhargav Kataru - [@bkataru](https://github.com/bkataru)

Project Link: [https://github.com/bkataru/jarvis](https://github.com/bkataru/jarvis)

---

**Note**: This is a work in progress. Many features are still being implemented. The current version provides the foundational structure and UI, with AI capabilities being actively developed.
