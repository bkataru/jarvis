# Contributing to JARVIS

Thank you for your interest in contributing to JARVIS! This document provides guidelines and information for contributors.

## Code of Conduct

Please be respectful and constructive in all interactions. We're building this together!

## Getting Started

1. **Fork the repository** and clone your fork
2. **Install prerequisites**:
   - Rust 1.70+ via [rustup](https://rustup.rs/)
   - wasm32 target: `rustup target add wasm32-unknown-unknown`
   - Trunk: `cargo install trunk`

3. **Build and test**:
   ```bash
   cargo check --all
   cargo build --target wasm32-unknown-unknown
   ```

4. **Run the development server**:
   ```bash
   trunk serve
   ```

## Project Structure

```
jarvis/
├── jarvis-app/      # Main web application (Leptos UI)
├── jarvis-ai/       # AI inference engine
├── jarvis-mcp/      # Model Context Protocol client
├── Cargo.toml       # Workspace configuration
├── Trunk.toml       # Build configuration
└── index.html       # HTML template
```

## Development Workflow

### Making Changes

1. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following the code style guidelines

3. Test your changes:
   ```bash
   cargo check --all
   cargo clippy --all --all-targets
   cargo fmt --all -- --check
   ```

4. Commit with clear messages:
   ```bash
   git commit -m "feat: add new feature"
   ```

5. Push and create a pull request

### Commit Message Convention

We follow conventional commits:

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `test:` Adding or updating tests
- `chore:` Maintenance tasks

## Code Style

### Rust Code

- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Keep functions focused and small

### Example:

```rust
/// Calculate the mel spectrogram from audio samples
///
/// # Arguments
///
/// * `audio` - Audio samples as f32
/// * `sample_rate` - Sample rate in Hz
///
/// # Returns
///
/// Vector of mel spectrogram values
///
/// # Errors
///
/// Returns error if sample rate is invalid
pub fn audio_to_mel(audio: &[f32], sample_rate: u32) -> Result<Vec<f32>, String> {
    // Implementation
}
```

## Areas for Contribution

### High Priority

1. **Candle Integration**: Resolve dependency conflicts and integrate Candle for inference
2. **Whisper Implementation**: Speech-to-text using Candle's Whisper
3. **LLM Integration**: Text generation with TinyLlama or Phi-2
4. **Web Workers**: Off-thread inference implementation
5. **Audio Processing**: Proper resampling and VAD

### Medium Priority

1. **MCP Servers**: Implement additional built-in MCP servers
2. **UI Improvements**: Better loading states, error handling
3. **Model Caching**: Cache API integration for model storage
4. **Testing**: WASM test harness and integration tests
5. **Documentation**: More examples and tutorials

### Good First Issues

1. **UI Polish**: Improve styling and animations
2. **Error Messages**: Better user-facing error messages
3. **Documentation**: Fix typos, add examples
4. **Code Cleanup**: Remove dead code, improve organization

## Testing

### Running Tests

Currently, tests require running in a WASM environment:

```bash
wasm-pack test --headless --firefox
```

For non-WASM code:

```bash
cargo test -p jarvis-ai -p jarvis-mcp --target wasm32-unknown-unknown
```

### Writing Tests

Add tests in the same file as your code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        assert_eq!(my_function(5), 10);
    }
}
```

## Pull Request Process

1. Update documentation if needed
2. Add tests for new functionality
3. Ensure all tests pass
4. Update README if adding features
5. Request review from maintainers

### PR Checklist

- [ ] Code compiles without warnings
- [ ] Tests pass
- [ ] Documentation updated
- [ ] Commit messages follow convention
- [ ] Code formatted with `cargo fmt`
- [ ] No `cargo clippy` warnings

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Feel free to open an issue for questions or discussion!

## Acknowledgments

Special thanks to:
- Nicolas Martin for the original JARVIS TypeScript implementation
- The Leptos, Candle, and Rust communities
- All contributors!
