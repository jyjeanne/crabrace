# Contributing to Crabrace

Thank you for your interest in contributing to Crabrace! This document provides guidelines and instructions for contributing.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/crabrace.git
   cd crabrace
   ```
3. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Cargo (comes with Rust)
- Git

### Building the Project

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run the server
cargo run

# Run with logging
RUST_LOG=debug cargo run
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality

Before submitting a PR, ensure your code passes all checks:

```bash
# Format code
cargo fmt

# Check formatting (CI will verify this)
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Run all checks
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

## Adding a New Provider

To add a new AI provider to Crabrace:

### 1. Create Provider Configuration

Create a new JSON file in `src/providers/configs/`:

**Example: `src/providers/configs/new_provider.json`**

```json
{
  "name": "New Provider",
  "id": "new-provider",
  "type": "new-provider",
  "base_url": "https://api.newprovider.com/v1",
  "models": [
    {
      "id": "model-id",
      "name": "Model Name",
      "cost_per_1m_in": 1.0,
      "cost_per_1m_out": 3.0,
      "context_window": 128000,
      "can_reason": false,
      "supports_images": true,
      "supports_tools": true,
      "supports_streaming": true,
      "description": "Model description"
    }
  ]
}
```

### 2. Update Registry

Edit `src/providers/registry.rs` to include the new provider:

```rust
// Add at the top with other configs
const NEW_PROVIDER_CONFIG: &str = include_str!("configs/new_provider.json");

// Add in load_providers() method
if let Ok(provider) = serde_json::from_str::<Provider>(NEW_PROVIDER_CONFIG) {
    providers.push(provider);
}
```

### 3. Test Your Addition

```bash
# Build and run
cargo run

# In another terminal, test the endpoint
curl http://localhost:8080/providers | jq '.[] | select(.id == "new-provider")'
```

### 4. Add Tests

Add a test case in `src/providers/registry.rs`:

```rust
#[test]
fn test_new_provider_loaded() {
    let registry = ProviderRegistry::new().unwrap();
    let provider = registry.get_by_id("new-provider").unwrap();
    assert!(provider.is_some());
}
```

## Commit Guidelines

We follow conventional commits:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions or changes
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

**Examples:**

```
feat: add support for Google Gemini provider
fix: correct token cost calculation for GPT-4
docs: update README with new installation instructions
test: add integration tests for provider registry
```

## Pull Request Process

1. **Update documentation** if you're adding features or changing behavior
2. **Add tests** for new functionality
3. **Run all checks** (`cargo fmt --check && cargo clippy && cargo test`)
4. **Update CHANGELOG.md** if applicable
5. **Submit PR** with a clear description of changes

### PR Description Template

```markdown
## Description
Brief description of what this PR does

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested your changes

## Checklist
- [ ] Code follows style guidelines (`cargo fmt`)
- [ ] Code passes linting (`cargo clippy`)
- [ ] All tests pass (`cargo test`)
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (if applicable)
```

## Code Style

- Follow Rust conventions and idioms
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Keep functions focused and concise
- Use `Result` and `Option` appropriately
- Avoid unwrap() in production code

## Questions?

Feel free to:
- Open an issue for bugs or feature requests
- Start a discussion for questions or ideas
- Reach out to maintainers

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
