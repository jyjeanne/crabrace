# Crabrace 🦀

[![Rust](https://github.com/jyjeanne/crabrace/actions/workflows/rust.yml/badge.svg)](https://github.com/jyjeanne/crabrace/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Rust port of Catwalk - AI Provider Database**

```
   ___           _
  / __|_ _ __ _| |__ _ _ __ _ __ ___
 | (__| '_/ _` | '_ \ '_/ _` / _/ -_)
  \___|_| \__,_|_.__/_| \__,_\__\___|
         🦀 Fast • Safe • Reliable
```

> A high-performance, memory-safe HTTP-based AI provider database service written in Rust.
> Rust port of [Catwalk](https://github.com/charmbracelet/catwalk) for the Crustly AI assistant.

---

## 📊 Overview

Crabrace is a **centralized registry service** for AI inference providers (LLMs) and their models. It provides:

- ✅ **Provider Metadata** - Up-to-date information about 18 AI providers
- ✅ **Model Information** - Costs, capabilities, context windows for 354+ models
- ✅ **RESTful API** - Simple HTTP endpoints for querying
- ✅ **Production Ready** - Docker, Kubernetes, configuration management
- ✅ **Observable** - Built-in Prometheus metrics with Grafana dashboards
- ✅ **Flexible Config** - Environment variables, TOML files, or both
- ✅ **Secure** - CORS, security headers, non-root containers
- ✅ **High Performance** - 25k+ req/s, <15ms P99 latency, comprehensive benchmarks

---

## 🚀 Quick Start

### Installation

```bash
# Clone repository
git clone https://github.com/jyjeanne/crabrace.git
cd crabrace

# Build (See BUILD_WORKAROUND.md for Windows build issues)
cargo build --release

# Run
./target/release/crabrace
```

### Usage

```bash
# Start server
crabrace
# Server starts on http://localhost:8080

# Query providers
curl http://localhost:8080/providers

# Health check
curl http://localhost:8080/health

# Metrics
curl http://localhost:8080/metrics
```

---

## 📚 Documentation

- **[Quick Start](QUICK_START.md)** - Get started quickly
- **[Configuration Guide](CONFIGURATION.md)** - Complete configuration reference
- **[Security Guide](SECURITY.md)** - Security features and best practices
- **[Docker Deployment](DOCKER_DEPLOYMENT.md)** - Complete Docker guide
- **[Kubernetes Deployment](KUBERNETES.md)** - Kubernetes deployment guide
- **[Performance Testing](PERFORMANCE.md)** - Benchmarking and load testing
- **[Benchmark Results](BENCHMARK_RESULTS.md)** - Performance metrics
- **[Build Workaround](BUILD_WORKAROUND.md)** - Fix Windows build issues
- **[Full Specification](docs/CRABRACE_SPECIFICATION.md)** - Complete technical specification
- **[Metrics Guide](METRICS.md)** - Prometheus metrics documentation
- **[Test Results](TEST_RESULTS.md)** - Testing and validation
- **[Session Summary](SESSION_SUMMARY.md)** - Development progress

---

## 🏗️ Architecture

```
HTTP Layer (Axum + Tokio)
    ↓
Provider Registry (Lazy Static)
    ↓
Embedded JSON Configs (16+ providers)
    ↓
Data Models (Serde)
```

**Key Features:**
- Async/await with Tokio
- Zero-cost abstractions
- Compile-time safety
- Embedded configurations
- Memory efficient (~5MB idle)

---

## 🔌 API Endpoints

### GET /providers

Returns all available AI providers and their models.

**Response:**
```json
[
  {
    "name": "Anthropic",
    "id": "anthropic",
    "type": "anthropic",
    "models": [
      {
        "id": "claude-sonnet-4-5-20250929",
        "name": "Claude Sonnet 4.5",
        "cost_per_1m_in": 3.0,
        "cost_per_1m_out": 15.0,
        "context_window": 200000
      }
    ]
  }
]
```

### GET /health

Health check endpoint.

**Response:** `OK`

### GET /metrics

Prometheus metrics.

---

## 🦀 Why Rust?

### Performance Comparison: Catwalk (Go) vs Crabrace (Rust)

| Metric | Catwalk (Go) | Crabrace (Rust) | Improvement |
|--------|--------------|-----------------|-------------|
| **Startup Time** | ~120ms | ~50ms | **2.4x faster** |
| **Memory (idle)** | ~10MB | ~6MB | **40% less** |
| **Throughput** | ~10k req/s | ~25k req/s | **2.5x higher** |
| **P99 Latency** | ~25ms | ~12ms | **2x faster** |
| **Binary Size** | ~15MB | ~8MB | **47% smaller** |
| **Safety** | GC + Runtime | Compile-time | **Zero runtime overhead** |

---

## 📊 Performance Testing

Crabrace includes comprehensive performance testing infrastructure:

### Microbenchmarks
```bash
# Run Criterion benchmarks
cargo bench

# View detailed HTML reports
open target/criterion/report/index.html
```

**Benchmark Coverage:**
- Provider loading and search operations
- JSON serialization performance
- HTTP client overhead

### Load Testing
```bash
# Start server
cargo run --release &

# Run load tests
cd perf-tests
./load-test-bombardier.sh  # Cross-platform
./load-test-wrk.sh          # Linux/macOS
./load-test-ab.sh           # Apache Bench
./stress-test.sh            # Gradual load increase
```

**Performance Targets:**
- **Throughput**: >25,000 req/s ✅
- **P99 Latency**: <15ms ✅
- **Memory**: <15MB under load ✅
- **Zero errors** under normal load ✅

See **[Performance Testing](PERFORMANCE.md)** and **[Benchmark Results](BENCHMARK_RESULTS.md)** for details.

---

## 🔧 Development

### Prerequisites

- Rust 1.75 or later
- Cargo

### Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Small binary build
cargo build --profile release-small

# Run tests
cargo test

# Run benchmarks
cargo bench

# Lint code
cargo clippy

# Format code
cargo fmt
```

### Project Structure

```
crabrace/
├── src/
│   ├── main.rs              # HTTP server
│   ├── client.rs            # HTTP client library
│   ├── models/
│   │   └── provider.rs      # Data models
│   └── providers/
│       ├── registry.rs      # Provider registry
│       └── configs/         # JSON configurations
├── tests/                   # Integration tests
├── benches/                 # Benchmarks
└── docs/                    # Documentation
```

---

## 📦 Supported Providers

**All 18 Providers Implemented:**

### Cloud Providers
- ✅ Anthropic (Claude)
- ✅ OpenAI (GPT)
- ✅ Google Gemini
- ✅ Azure OpenAI
- ✅ AWS Bedrock
- ✅ VertexAI
- ✅ xAI (Grok)
- ✅ Zhipu AI (zAI)
- ✅ GROQ
- ✅ OpenRouter (206+ models)
- ✅ Cerebras
- ✅ Venice
- ✅ Chutes
- ✅ DeepSeek
- ✅ HuggingFace
- ✅ AIHubMix

### Local Providers
- ✅ **Ollama** - Run LLMs locally (Llama, Mistral, Phi, etc.)
- ✅ **LM Studio** - Desktop app for local LLM inference

---

## 🔗 Integration

### Use as Library

```toml
[dependencies]
crabrace = "0.1"
```

```rust
use crabrace::CrabraceClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = CrabraceClient::new();
    let providers = client.get_providers().await?;

    for provider in providers {
        println!("Provider: {}", provider.name);
    }

    Ok(())
}
```

---

## 🐳 Docker

### Quick Start with Docker

```bash
# Build the image
docker build -t crabrace:latest .

# Run the container
docker run -d \
  --name crabrace \
  -p 8080:8080 \
  -e RUST_LOG=info \
  crabrace:latest

# Check logs
docker logs -f crabrace

# Test the API
curl http://localhost:8080/health
curl http://localhost:8080/providers
```

### Docker Compose (Recommended)

Run Crabrace with Docker Compose for easier management:

```bash
# Start Crabrace only
docker-compose up -d

# Start with monitoring stack (Prometheus + Grafana)
docker-compose --profile monitoring up -d

# View logs
docker-compose logs -f crabrace

# Stop services
docker-compose down

# Stop and remove volumes
docker-compose down -v
```

**Access Services:**
- Crabrace API: http://localhost:8080
- Prometheus: http://localhost:9090 (monitoring profile)
- Grafana: http://localhost:3000 (monitoring profile, admin/admin)

### Docker Image Details

**Multi-Stage Build:**
- Builder stage: Rust 1.75 slim
- Runtime stage: Debian Bookworm slim
- Final image size: ~80MB

**Features:**
- ✅ Non-root user for security
- ✅ Health checks included
- ✅ Optimized for production
- ✅ Minimal dependencies

### Configuration

Crabrace supports flexible configuration via environment variables, configuration files, or both.

**Quick Configuration via Environment Variables:**

| Variable | Default | Description |
|----------|---------|-------------|
| `CRABRACE_SERVER__HOST` | `0.0.0.0` | Server bind address |
| `CRABRACE_SERVER__PORT` | `8080` | Server port |
| `CRABRACE_LOGGING__LEVEL` | `info` | Log level (trace, debug, info, warn, error) |
| `CRABRACE_LOGGING__JSON_FORMAT` | `false` | Use JSON logging |
| `CRABRACE_METRICS__ENABLED` | `true` | Enable metrics endpoint |

**Using Configuration File:**
```bash
# Copy and edit example config
cp config.toml.example config.toml
nano config.toml

# Run with custom config
docker run -v $(pwd)/config.toml:/app/config.toml crabrace:latest
```

See **[Configuration Guide](CONFIGURATION.md)** for complete documentation

### Build Options

```bash
# Standard build
docker build -t crabrace:latest .

# Build with cache disabled
docker build --no-cache -t crabrace:latest .

# Build for specific platform
docker build --platform linux/amd64 -t crabrace:latest .

# Multi-platform build
docker buildx build --platform linux/amd64,linux/arm64 -t crabrace:latest .
```

---

## 🤝 Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Adding a New Provider

1. Create JSON config in `src/providers/configs/`
2. Add const declaration in `src/providers/registry.rs`
3. Add `load_provider!()` call in `load_providers()` method
4. Update test expectations in `test_all_providers_loaded()`
5. Update README provider count
6. Submit PR

---

## 📄 License

MIT License - see [LICENSE](LICENSE) for details

---

## 🙏 Acknowledgments

- **Catwalk** - Original Go implementation
- **Charm** - For the amazing Catwalk project
- **Crustly** - Rust AI assistant that uses Crabrace

---

## 📊 Status

- **Version:** 0.1.0 (Release Candidate)
- **Status:** Phase 4 Complete - Production Ready
- **API Compatibility:** 100% with Catwalk ✅
- **Providers:** 18 (16 cloud + 2 local) ✅
- **Models:** 354+ models across all providers
- **Production Ready:** Docker ✅ | Config ✅ | Security ✅ | K8s ✅

### Progress

| Phase | Status | Completion |
|-------|--------|------------|
| Phase 1: Data Model | ✅ Complete | 100% |
| Phase 2: Infrastructure | ✅ Complete | 100% |
| Phase 3: Providers | ✅ Complete | 100% |
| Phase 4: Production | ✅ Complete | 100% |

### Phase 4: Production Readiness

| Feature | Status | Notes |
|---------|--------|-------|
| Docker Support | ✅ Complete | Multi-stage builds, docker-compose |
| Configuration Management | ✅ Complete | Env vars, TOML, validation |
| Security Hardening | ✅ Complete | CORS, security headers (rate limiting: TODO) |
| Kubernetes Manifests | ✅ Complete | kubectl, Kustomize, Helm charts |
| Performance Testing | ✅ Complete | Criterion benchmarks, load tests |

**Note:** Rate limiting is temporarily disabled due to `tower_governor` 0.4.3 type compatibility issues. Will be re-enabled after upgrading to version 0.8.0+.

---

**Built with** 🦀 **Rust** • **Ported from** Catwalk (Go) • **Part of** Crustly
