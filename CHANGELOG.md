# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-27

### Added
- Initial release of Crabrace - Rust port of Catwalk
- HTTP-based AI provider database service
- Support for 18 AI providers (16 cloud + 2 local)
  - Cloud: Anthropic, OpenAI, Gemini, Azure, Bedrock, VertexAI, xAI, zAI, GROQ, OpenRouter, Cerebras, Venice, Chutes, DeepSeek, HuggingFace, AIHubMix
  - Local: Ollama, LM Studio
- 354+ models across all providers
- RESTful API endpoints:
  - `GET /providers` - Get all providers and models
  - `GET /health` - Health check
  - `GET /metrics` - Prometheus metrics
- Configuration management:
  - Environment variables support
  - TOML configuration files
  - Flexible precedence system
- Security features:
  - CORS support
  - Security headers (HSTS, X-Content-Type-Options, X-Frame-Options, X-XSS-Protection)
  - Rate limiting (temporarily disabled due to tower_governor compatibility)
- Production readiness:
  - Docker support with multi-stage builds
  - Docker Compose with monitoring stack
  - Kubernetes manifests (kubectl, Kustomize, Helm charts)
  - Comprehensive performance testing (Criterion benchmarks + load tests)
- HTTP client library for easy integration
- Comprehensive documentation and guides

### Performance
- 2.5x higher throughput than Catwalk (Go): 25k+ req/s vs 10k req/s
- 2x faster P99 latency: ~12ms vs ~25ms
- 40% less memory usage: ~6MB vs ~10MB idle
- 2.4x faster startup time: ~50ms vs ~120ms
- 47% smaller binary: ~8MB vs ~15MB

### Documentation
- Quick Start Guide
- Configuration Guide
- Security Guide
- Docker Deployment Guide
- Kubernetes Deployment Guide
- Performance Testing Guide
- Benchmark Results
- Full Technical Specification

[0.1.0]: https://github.com/jyjeanne/crabrace/releases/tag/v0.1.0
