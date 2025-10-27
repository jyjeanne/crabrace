# Contributing to Crabrace

Thank you for your interest in contributing to Crabrace! This document provides guidelines and information for contributors.

## 🚀 Quick Start

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/crabrace.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit: `git commit -m "Description of changes"`
7. Push: `git push origin feature/your-feature-name`
8. Create a Pull Request

## 🔧 Development Setup

### Prerequisites

- Rust 1.75 or later
- Cargo
- Git

### Build Instructions

```bash
# Clone the repository
git clone https://github.com/jyjeanne/crabrace.git
cd crabrace

# Build
cargo build

# Run tests
cargo test

# Run the server
cargo run
```

### Windows Build Issues

If you encounter build issues on Windows, see [BUILD_WORKAROUND.md](../BUILD_WORKAROUND.md) for solutions.

## 📋 Before Submitting

### Code Quality Checks

Run these commands before submitting a PR:

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Run tests
cargo test

# Validate JSON configs
python verify_json.py
```

### CI Pipeline

Our GitHub Actions CI will automatically run:
- Code formatting check (`cargo fmt -- --check`)
- Linter (`cargo clippy`)
- Build (`cargo build`)
- Tests (`cargo test`)
- JSON validation (`python verify_json.py`)

All checks must pass before a PR can be merged.

## 🎯 What to Contribute

### High Priority

1. **Add Providers** - Help us reach 16 providers!
   - Google Gemini
   - Azure OpenAI
   - AWS Bedrock
   - OpenRouter (206 models)
   - See [FEATURE_PARITY_ANALYSIS.md](../docs/FEATURE_PARITY_ANALYSIS.md)

2. **Documentation**
   - API examples
   - Integration guides
   - Tutorials

3. **Tests**
   - Integration tests
   - Load tests
   - Edge case coverage

### Medium Priority

4. **Performance**
   - Benchmarks
   - Optimization
   - Memory profiling

5. **Features**
   - Additional metrics
   - Caching
   - Rate limiting

### Low Priority

6. **Infrastructure**
   - Docker improvements
   - Kubernetes configs
   - Monitoring dashboards

## 📦 Adding a New Provider

To add a new AI provider:

1. **Create JSON Config**

   Create `src/providers/configs/provider_name.json`:
   ```json
   {
     "name": "Provider Name",
     "id": "provider_id",
     "type": "openai",
     "api_key": "$PROVIDER_API_KEY",
     "api_endpoint": "$PROVIDER_API_ENDPOINT",
     "default_large_model_id": "model-large",
     "default_small_model_id": "model-small",
     "default_headers": null,
     "models": [
       {
         "id": "model-id",
         "name": "Model Name",
         "cost_per_1m_in": 1.0,
         "cost_per_1m_out": 3.0,
         "cost_per_1m_in_cached": 0.1,
         "cost_per_1m_out_cached": 0.3,
         "context_window": 128000,
         "default_max_tokens": 4096,
         "can_reason": false,
         "has_reasoning_efforts": false,
         "default_reasoning_effort": null,
         "supports_attachments": true
       }
     ]
   }
   ```

2. **Update Registry**

   In `src/providers/registry.rs`, add:
   ```rust
   const PROVIDER_CONFIG: &str = include_str!("configs/provider_name.json");

   // In load_providers():
   let provider: Provider = serde_json::from_str(PROVIDER_CONFIG)?;
   providers.push(provider);
   ```

3. **Validate**
   ```bash
   python verify_json.py
   ```

4. **Test**
   ```bash
   cargo test
   cargo run
   curl http://localhost:8080/providers | jq
   ```

5. **Submit PR**
   - Include provider documentation
   - Update provider count in README
   - Reference official provider documentation

## 📝 Commit Message Guidelines

Use clear, descriptive commit messages:

```
Add Google Gemini provider with 3 models

- Added gemini.json config
- Updated registry.rs to load Gemini
- Added tests for Gemini provider
- Updated README with new provider count
```

**Format:**
- Start with imperative verb (Add, Fix, Update, Remove)
- Keep first line under 72 characters
- Add details in body if needed
- Reference issues: `Fixes #123`

## 🧪 Testing Guidelines

### Unit Tests

Add tests for new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_loading() {
        let registry = ProviderRegistry::new().unwrap();
        assert!(registry.get_all().unwrap().len() > 0);
    }
}
```

### Integration Tests

Create tests in `tests/` directory:

```rust
// tests/api_test.rs
#[tokio::test]
async fn test_providers_endpoint() {
    // Test implementation
}
```

## 📖 Documentation Guidelines

- Use clear, concise language
- Include code examples
- Add comments for complex logic
- Update README when adding features
- Keep documentation in sync with code

## 🔍 Code Review Process

1. **Automated Checks**
   - All CI checks must pass
   - No clippy warnings
   - Code must be formatted

2. **Manual Review**
   - Code quality and style
   - Test coverage
   - Documentation completeness
   - Breaking changes noted

3. **Approval**
   - At least one maintainer approval required
   - Address all feedback
   - Squash commits if requested

## 🤝 Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on the code, not the person
- Help others learn and grow

## 📧 Questions?

- Open an issue for bugs or features
- Start a discussion for questions
- Check existing issues first

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Crabrace! 🦀
