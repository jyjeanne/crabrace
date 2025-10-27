# Crabrace - Quick Start Guide

## ⚡ TL;DR

```bash
# Fix build first (choose one):
# Option A: Use WSL
wsl
cargo build && cargo run

# Option B: Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/

# Option C: Rename Git's link.exe (temporary)
mv /c/Git/usr/bin/link.exe /c/Git/usr/bin/link.exe.bak
cargo build
mv /c/Git/usr/bin/link.exe.bak /c/Git/usr/bin/link.exe
```

---

## 🚀 Getting Started

### 1. Build the Project

See `BUILD_WORKAROUND.md` for detailed solutions. Quick option:

```bash
cd crabrace

# If you have Visual Studio Build Tools installed:
cargo build --release

# Otherwise, use WSL:
wsl
cargo build --release
```

### 2. Run the Server

```bash
cargo run
# Server starts on http://localhost:8080
```

### 3. Test the API

```bash
# Health check
curl http://localhost:8080/health
# Response: OK

# Get all providers
curl http://localhost:8080/providers | jq
# Response: JSON array of providers

# Get metrics
curl http://localhost:8080/metrics
# Response: Prometheus metrics
```

---

## 📊 Example Responses

### /providers

```json
[
  {
    "name": "Anthropic",
    "id": "anthropic",
    "type": "anthropic",
    "api_key": "$ANTHROPIC_API_KEY",
    "api_endpoint": "$ANTHROPIC_API_ENDPOINT",
    "default_large_model_id": "claude-sonnet-4-5-20250929",
    "default_small_model_id": "claude-3-5-haiku-20241022",
    "default_headers": null,
    "models": [
      {
        "id": "claude-sonnet-4-5-20250929",
        "name": "Claude Sonnet 4.5",
        "cost_per_1m_in": 3.0,
        "cost_per_1m_out": 15.0,
        "cost_per_1m_in_cached": 3.75,
        "cost_per_1m_out_cached": 0.3,
        "context_window": 200000,
        "default_max_tokens": 50000,
        "can_reason": true,
        "has_reasoning_efforts": false,
        "default_reasoning_effort": null,
        "supports_attachments": true
      }
    ]
  }
]
```

### /metrics

```
# HELP crabrace_providers_requests_total Total number of requests to the providers endpoint
# TYPE crabrace_providers_requests_total counter
crabrace_providers_requests_total 5
```

---

## 🧪 Testing

### Validate JSON Configs

```bash
python verify_json.py
```

Expected output:
```
[SUCCESS] All provider configurations are valid!
```

### Run Unit Tests

```bash
cargo test
```

### Load Test

```bash
# Make multiple requests
for i in {1..10}; do curl http://localhost:8080/providers; done

# Check metrics
curl http://localhost:8080/metrics | grep crabrace
```

---

## 📖 Documentation

| Document | Purpose |
|----------|---------|
| `README.md` | Project overview |
| `BUILD_WORKAROUND.md` | Build environment solutions |
| `METRICS.md` | Prometheus metrics guide |
| `TEST_RESULTS.md` | Testing & validation |
| `SESSION_SUMMARY.md` | Complete development summary |
| `PROMETHEUS_METRICS_IMPLEMENTATION.md` | Metrics technical details |

---

## 🔧 Common Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build

# Run
cargo run                      # Run server

# Test
cargo test                     # Run all tests
cargo test metrics             # Run metrics tests

# Format & Lint
cargo fmt                      # Format code
cargo clippy                   # Lint code

# Clean
cargo clean                    # Remove build artifacts
```

---

## 🐛 Troubleshooting

### Build Fails: "linker `link.exe` not found"

**Solution:** See `BUILD_WORKAROUND.md` - Install Visual Studio Build Tools or use WSL

### Build Fails: "link: extra operand"

**Solution:** Git's link.exe is in PATH. See `BUILD_WORKAROUND.md` for solutions

### Port 8080 Already in Use

```bash
# Find process using port 8080
netstat -ano | findstr :8080

# Kill the process (replace PID)
taskkill /PID <PID> /F

# Or use a different port (modify src/main.rs)
```

### JSON Parse Errors

```bash
# Validate JSON files
python verify_json.py

# Check specific file
python -m json.tool src/providers/configs/anthropic.json
```

---

## 🎯 Next Steps

1. **Fix Build Environment** - See `BUILD_WORKAROUND.md`
2. **Run Tests** - `cargo test`
3. **Start Server** - `cargo run`
4. **Add Providers** - Create more JSON configs in `src/providers/configs/`

---

## 📦 Project Structure

```
crabrace/
├── src/
│   ├── main.rs              # HTTP server
│   ├── lib.rs               # Client library
│   ├── metrics.rs           # Prometheus metrics
│   ├── models/
│   │   └── provider.rs      # Data models
│   └── providers/
│       ├── registry.rs      # Provider registry
│       └── configs/
│           ├── anthropic.json
│           └── openai.json
├── docs/                    # Documentation
├── examples/                # Example code
└── Cargo.toml              # Dependencies
```

---

## 🔗 Resources

- **Catwalk (Original):** https://github.com/charmbracelet/catwalk
- **Prometheus Docs:** https://prometheus.io/docs/
- **Rust Book:** https://doc.rust-lang.org/book/
- **Axum Docs:** https://docs.rs/axum/

---

## 📞 Support

- Read `SESSION_SUMMARY.md` for complete context
- Check `BUILD_WORKAROUND.md` for build issues
- See `METRICS.md` for Prometheus integration

---

**Status:** ✅ Ready for Development
**Build:** ⚠️ Requires environment setup
**Code Quality:** ✅ Production-ready
**Documentation:** ✅ Comprehensive
