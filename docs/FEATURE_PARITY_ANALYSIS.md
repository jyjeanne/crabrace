# Crabrace vs Catwalk Feature Parity Analysis

**Date:** October 26, 2025
**Catwalk Version:** Based on commit analysis
**Crabrace Version:** 0.1.0 (Initial Implementation)

---

## Executive Summary

| Category | Catwalk (Go) | Crabrace (Rust) | Status |
|----------|--------------|-----------------|--------|
| **Providers** | 16 | 2 | ⚠️ 14 missing |
| **Total Models** | 341 | 10 | ⚠️ 331 missing |
| **Data Model Fields** | 15 | 10 | ⚠️ 5 missing |
| **HTTP Endpoints** | 3 | 3 | ✅ Complete |
| **Client Library** | ✅ | ✅ | ✅ Complete |
| **Metrics** | Prometheus | Prometheus | ✅ Complete |
| **Docker Support** | Multi-arch | ❌ | ⚠️ Missing |
| **CI/CD** | GitHub Actions | ❌ | ⚠️ Missing |

---

## 1. Data Model Comparison

### Provider Structure

| Field | Catwalk (Go) | Crabrace (Rust) | Status | Priority |
|-------|--------------|-----------------|--------|----------|
| `name` | ✅ string | ✅ String | ✅ | - |
| `id` | ✅ InferenceProvider | ✅ String | ✅ | - |
| `type` | ✅ Type enum | ✅ String | ⚠️ Should be enum | HIGH |
| `api_key` | ✅ string | ⚠️ Option<String> | ⚠️ Different structure | MEDIUM |
| `api_endpoint` | ✅ string | ⚠️ base_url | ⚠️ Field name mismatch | HIGH |
| `default_large_model_id` | ✅ string | ❌ Missing | ⚠️ **MISSING** | **CRITICAL** |
| `default_small_model_id` | ✅ string | ❌ Missing | ⚠️ **MISSING** | **CRITICAL** |
| `default_headers` | ✅ map[string]string | ❌ Missing | ⚠️ **MISSING** | **CRITICAL** |
| `models` | ✅ []Model | ✅ Vec<Model> | ✅ | - |
| `metadata` | ❌ | ⚠️ Option<Value> | ⚠️ Extra field | LOW |

**Critical Issues:**
- Missing `default_large_model_id` and `default_small_model_id` - used by Crush for model selection
- Missing `default_headers` - required for AIHubMix, OpenRouter, HuggingFace
- Field name `base_url` should be `api_endpoint` for API compatibility

### Model Structure

| Field | Catwalk (Go) | Crabrace (Rust) | Status | Priority |
|-------|--------------|-----------------|--------|----------|
| `id` | ✅ string | ✅ String | ✅ | - |
| `name` | ✅ string | ✅ String | ✅ | - |
| `cost_per_1m_in` | ✅ float64 | ✅ f64 (as cost_per_1m_input) | ⚠️ Name mismatch | HIGH |
| `cost_per_1m_out` | ✅ float64 | ✅ f64 (as cost_per_1m_output) | ⚠️ Name mismatch | HIGH |
| `cost_per_1m_in_cached` | ✅ float64 | ❌ Missing | ⚠️ **MISSING** | **CRITICAL** |
| `cost_per_1m_out_cached` | ✅ float64 | ❌ Missing | ⚠️ **MISSING** | **CRITICAL** |
| `context_window` | ✅ int64 | ✅ u64 | ✅ | - |
| `default_max_tokens` | ✅ int64 | ❌ Missing | ⚠️ **MISSING** | **CRITICAL** |
| `can_reason` | ✅ bool | ✅ bool | ✅ | - |
| `has_reasoning_efforts` | ✅ bool | ❌ Missing | ⚠️ **MISSING** | **HIGH** |
| `default_reasoning_effort` | ✅ string | ❌ Missing | ⚠️ **MISSING** | **HIGH** |
| `supports_attachments` | ✅ bool | ⚠️ supports_images | ⚠️ **NAME MISMATCH** | **CRITICAL** |
| `supports_tools` | ❌ | ⚠️ bool (extra) | ⚠️ Extra field | LOW |
| `supports_streaming` | ❌ | ⚠️ bool (extra) | ⚠️ Extra field | LOW |
| `description` | ❌ | ⚠️ Option<String> (extra) | ⚠️ Extra field | LOW |
| `version` | ❌ | ⚠️ Option<String> (extra) | ⚠️ Extra field | LOW |

**Critical Issues:**
- Field name `supports_images` MUST be `supports_attachments` for JSON compatibility
- Missing cached pricing fields - essential for prompt caching features
- Missing `default_max_tokens` - important for API calls
- Missing reasoning effort fields - needed for advanced reasoning models
- Field names `cost_per_1m_input/output` should be `cost_per_1m_in/out`

---

## 2. Provider Comparison

### Implemented Providers

| # | Provider | Catwalk | Crabrace | Models (Catwalk) | Models (Crabrace) | Status |
|---|----------|---------|----------|------------------|-------------------|--------|
| 1 | Anthropic | ✅ | ✅ | 9 | 4 | ⚠️ Partial (5 models missing) |
| 2 | OpenAI | ✅ | ✅ | 12 | 6 | ⚠️ Partial (6 models missing) |

**Anthropic Models Missing:**
- claude-haiku-4-5-20251001
- claude-opus-4-1-20250805
- claude-opus-4-20250514
- claude-sonnet-4-20250514
- claude-3-7-sonnet-20250219

**OpenAI Models Missing:**
- gpt-5
- gpt-5-mini
- gpt-5-nano
- o4-mini
- o3
- gpt-4.1 (and variants)

### Missing Providers (14 total)

| # | Provider | Type | Models | Key Features | Priority |
|---|----------|------|--------|--------------|----------|
| 3 | Google Gemini | gemini | 3 | 1M context window | **CRITICAL** |
| 4 | Azure OpenAI | azure | 15 | Enterprise deployment | **CRITICAL** |
| 5 | AWS Bedrock | bedrock | 7 | AWS integration | **CRITICAL** |
| 6 | Google Vertex AI | vertexai | 3 | GCP integration | HIGH |
| 7 | xAI (Grok) | openai | 6 | Grok models | HIGH |
| 8 | Z.AI (GLM) | openai | 4 | GLM models | MEDIUM |
| 9 | Groq | openai | 3 | Fast inference | MEDIUM |
| 10 | Cerebras | openai | 10 | Open source models | MEDIUM |
| 11 | Venice AI | openai | 6 | Privacy-focused | LOW |
| 12 | Chutes | openai | 21 | Largest variety | MEDIUM |
| 13 | DeepSeek | openai | 3 | Reasoning models | HIGH |
| 14 | HuggingFace | openai | 24 | Router aggregator | MEDIUM |
| 15 | OpenRouter | openai | 206 | **MOST MODELS** | **CRITICAL** |
| 16 | AIHubMix | openai | 12 | Aggregator | MEDIUM |

**Total Missing Models:** 331

---

## 3. HTTP API Comparison

### Endpoints

| Endpoint | Catwalk | Crabrace | Compatibility | Notes |
|----------|---------|----------|---------------|-------|
| `GET /providers` | ✅ | ✅ | ✅ | Response format compatible |
| `HEAD /providers` | ✅ | ❌ | ⚠️ Missing | Should support HEAD |
| `GET /health` | ✅ (at /healthz) | ✅ (at /health) | ⚠️ | **PATH MISMATCH** |
| `GET /metrics` | ✅ | ✅ | ✅ | Prometheus format |

**Issues:**
- Health endpoint path: Catwalk uses `/healthz`, Crabrace uses `/health`
- HEAD method not implemented for `/providers`
- HTTP 405 for unsupported methods needed

### Response Format Compatibility

**Catwalk Response (JSON):**
```json
{
  "name": "OpenAI",
  "id": "openai",
  "type": "openai",
  "api_key": "$OPENAI_API_KEY",
  "api_endpoint": "$OPENAI_API_ENDPOINT",
  "default_large_model_id": "gpt-5",
  "default_small_model_id": "gpt-4o",
  "default_headers": null,
  "models": [...]
}
```

**Crabrace Response (Current):**
```json
{
  "name": "Anthropic",
  "id": "anthropic",
  "provider_type": "anthropic",
  "base_url": "https://api.anthropic.com/v1",
  "api_key": null,
  "models": [...]
}
```

**Compatibility Issues:**
- ❌ `provider_type` should be `type`
- ❌ `base_url` should be `api_endpoint`
- ❌ Missing `default_large_model_id`
- ❌ Missing `default_small_model_id`
- ❌ Missing `default_headers`

**JSON Serialization Names:**
```rust
// CURRENT (WRONG)
#[serde(rename = "type")]
pub provider_type: String,

// SHOULD BE
pub type: String,  // or #[serde(rename = "type")] if 'type' is keyword
```

---

## 4. Client Library Comparison

| Feature | Catwalk | Crabrace | Status |
|---------|---------|----------|--------|
| HTTP client | ✅ net/http | ✅ reqwest | ✅ |
| Base URL configuration | ✅ $CATWALK_URL | ⚠️ Constructor param | ⚠️ Different |
| Get providers method | ✅ GetProviders() | ✅ get_providers() | ✅ |
| Health check | ❌ | ✅ health_check() | ✅ Extra (good) |
| Error handling | ✅ Wrapped errors | ✅ Result<T> | ✅ |
| Default URL | localhost:8080 | localhost:8080 | ✅ |

**Issues:**
- Crabrace doesn't read `CATWALK_URL` environment variable
- Should add `Default::default()` to use env var

---

## 5. Metrics & Monitoring

### Prometheus Metrics

| Metric | Catwalk | Crabrace | Status |
|--------|---------|----------|--------|
| Request counter | ✅ catwalk_providers_requests_total | ❌ Not implemented | ⚠️ **MISSING** |
| Namespace | catwalk | - | ⚠️ Should be "crabrace" |
| Subsystem | providers | - | ⚠️ Missing |

**Missing Implementation:**
```rust
use prometheus::{register_counter, Counter};
use lazy_static::lazy_static;

lazy_static! {
    static ref PROVIDERS_REQUESTS: Counter = register_counter!(
        "crabrace_providers_requests_total",
        "Total number of requests to the providers endpoint"
    ).unwrap();
}

// In handler:
PROVIDERS_REQUESTS.inc();
```

---

## 6. Build & Deployment

### Docker Support

| Feature | Catwalk | Crabrace | Status |
|---------|---------|----------|--------|
| Dockerfile | ✅ Alpine-based | ❌ | ⚠️ **MISSING** |
| Multi-arch builds | ✅ amd64, arm64 | ❌ | ⚠️ **MISSING** |
| Docker registry | ✅ ghcr.io | ❌ | ⚠️ **MISSING** |
| Image tagging | ✅ version, latest, nightly | ❌ | ⚠️ **MISSING** |
| OCI labels | ✅ | ❌ | ⚠️ **MISSING** |

### CI/CD

| Feature | Catwalk | Crabrace | Status |
|---------|---------|----------|--------|
| Build workflow | ✅ | ❌ | ⚠️ **MISSING** |
| Lint workflow | ✅ golangci-lint | ❌ | ⚠️ **MISSING** |
| Release workflow | ✅ GoReleaser | ❌ | ⚠️ **MISSING** |
| Nightly builds | ✅ | ❌ | ⚠️ **MISSING** |
| Dependabot | ✅ | ❌ | ⚠️ **MISSING** |

### Build Configuration

| Feature | Catwalk | Crabrace | Status |
|---------|---------|----------|--------|
| Version injection | ✅ Link flags | ❌ | ⚠️ **MISSING** |
| Strip symbols | ✅ -s -w | ⚠️ strip=true | ✅ Partial |
| Commit SHA | ✅ Injected | ❌ | ⚠️ **MISSING** |
| Build date | ✅ Injected | ❌ | ⚠️ **MISSING** |
| Release automation | ✅ GoReleaser | ❌ | ⚠️ **MISSING** |

---

## 7. Configuration & Tooling

### Code Quality

| Tool | Catwalk | Crabrace | Status |
|------|---------|----------|--------|
| Linter | golangci-lint (26 linters) | ❌ clippy not configured | ⚠️ **MISSING** |
| Formatter | gofumpt | ❌ rustfmt not configured | ⚠️ **MISSING** |
| Task runner | Task/Taskfile | ❌ | ⚠️ **MISSING** |
| Modernizer | modernize | N/A | - |

### Task Automation

Catwalk tasks that should be replicated:
- `task run` - Run server
- `task install` - Install binary
- `task lint` - Run linters
- `task lint:fix` - Fix linting issues
- `task fmt` - Format code
- `task generate` - Generate configs (OpenRouter, HuggingFace)

---

## 8. Special Features

### Environment Variable Handling

| Feature | Catwalk | Crabrace | Status |
|---------|---------|----------|--------|
| API key placeholders | ✅ $VARIABLE_NAME | ❌ | ⚠️ **MISSING** |
| Endpoint placeholders | ✅ $VARIABLE_NAME | ❌ | ⚠️ **MISSING** |
| Client URL | ✅ $CATWALK_URL | ❌ | ⚠️ **MISSING** |

**Example from Catwalk:**
```json
{
  "api_key": "$OPENAI_API_KEY",
  "api_endpoint": "$OPENAI_API_ENDPOINT"
}
```

### Custom Headers Support

| Provider | Headers Needed | Crabrace Support |
|----------|----------------|------------------|
| OpenRouter | HTTP-Referer, X-Title | ❌ |
| HuggingFace | HTTP-Referer, X-Title | ❌ |
| AIHubMix | APP-Code | ❌ |

### Config Generators

| Generator | Purpose | Crabrace Equivalent |
|-----------|---------|---------------------|
| cmd/openrouter | Fetch 206 models from OpenRouter API | ❌ **MISSING** |
| cmd/huggingface | Fetch models from HF Router API | ❌ **MISSING** |

---

## 9. Priority Matrix for Implementation

### Phase 1: Critical Fixes (Week 1)

1. **Data Model JSON Compatibility** (BLOCKER)
   - Rename `supports_images` → `supports_attachments`
   - Rename `cost_per_1m_input/output` → `cost_per_1m_in/out`
   - Rename `provider_type` → `type`
   - Rename `base_url` → `api_endpoint`
   - Add `default_large_model_id`, `default_small_model_id`
   - Add `default_headers: Option<HashMap<String, String>>`
   - Add cached pricing fields
   - Add `default_max_tokens`
   - Add reasoning effort fields

2. **Complete Anthropic & OpenAI Configs** (CRITICAL)
   - Add missing 5 Anthropic models
   - Add missing 6 OpenAI models

### Phase 2: Major Providers (Week 2-3)

3. **Add Critical Providers**
   - Google Gemini (3 models)
   - Azure OpenAI (15 models)
   - AWS Bedrock (7 models)
   - OpenRouter (206 models) - **MOST IMPORTANT**

### Phase 3: Additional Providers (Week 4)

4. **Add Secondary Providers**
   - xAI, DeepSeek, Vertex AI
   - Groq, Cerebras, Chutes
   - HuggingFace, Venice, Z.AI, AIHubMix

### Phase 4: Infrastructure (Week 5)

5. **Build & Deploy**
   - Dockerfile (Alpine-based)
   - GitHub Actions workflows
   - Multi-arch builds
   - Release automation

6. **Metrics & Monitoring**
   - Prometheus counter
   - Request tracking

7. **Config Generators**
   - OpenRouter generator
   - HuggingFace generator

---

## 10. Breaking Changes Summary

### JSON API Breaking Changes

These changes MUST be made to achieve API compatibility:

```rust
// BEFORE (Crabrace 0.1.0 - INCOMPATIBLE)
{
  "provider_type": "anthropic",
  "base_url": "https://...",
  "models": [{
    "cost_per_1m_input": 3.0,
    "cost_per_1m_output": 15.0,
    "supports_images": true,
    "supports_tools": true,
    "supports_streaming": true
  }]
}

// AFTER (Compatible with Catwalk)
{
  "type": "anthropic",
  "api_endpoint": "$ANTHROPIC_API_ENDPOINT",
  "default_large_model_id": "claude-sonnet-4-5-20250929",
  "default_small_model_id": "claude-3-5-haiku-20241022",
  "default_headers": null,
  "models": [{
    "cost_per_1m_in": 3.0,
    "cost_per_1m_out": 15.0,
    "cost_per_1m_in_cached": 3.75,
    "cost_per_1m_out_cached": 0.3,
    "default_max_tokens": 50000,
    "can_reason": true,
    "has_reasoning_efforts": false,
    "default_reasoning_effort": "",
    "supports_attachments": true
  }]
}
```

### Rust Code Changes Required

```rust
// Provider struct
pub struct Provider {
    pub name: String,
    pub id: String,
    #[serde(rename = "type")]  // 'type' is a keyword, must rename
    pub provider_type: String,  // Keep field name as provider_type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,  // Changed from base_url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_large_model_id: Option<String>,  // NEW
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_small_model_id: Option<String>,  // NEW
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_headers: Option<HashMap<String, String>>,  // NEW
    pub models: Vec<Model>,
}

// Model struct
pub struct Model {
    pub id: String,
    pub name: String,
    pub cost_per_1m_in: f64,  // Changed from cost_per_1m_input
    pub cost_per_1m_out: f64,  // Changed from cost_per_1m_output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_1m_in_cached: Option<f64>,  // NEW
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_1m_out_cached: Option<f64>,  // NEW
    pub context_window: u64,
    pub default_max_tokens: u64,  // NEW
    #[serde(default)]
    pub can_reason: bool,
    #[serde(default)]
    pub has_reasoning_efforts: bool,  // NEW (note: plural in Go)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_reasoning_effort: Option<String>,  // NEW
    #[serde(default)]
    pub supports_attachments: bool,  // Changed from supports_images
}
```

---

## 11. Testing Requirements

### Test Coverage Needed

1. **JSON Serialization Tests**
   - Verify exact field names match Catwalk
   - Test with all 16 providers
   - Compare serialized output with Catwalk

2. **API Compatibility Tests**
   - Run both servers
   - Compare `/providers` responses
   - Verify identical JSON structure

3. **Client Library Tests**
   - Test against live Catwalk server
   - Test against Crabrace server
   - Verify interoperability

---

## Summary

### Current Status: ⚠️ **NOT PRODUCTION READY**

**Completion: 15% (2/16 providers, incomplete data model)**

### Blockers

1. ❌ **JSON API incompatibility** - Field names don't match
2. ❌ **Missing 14 providers** - Only 2/16 implemented
3. ❌ **Missing 331 models** - Only 10/341 implemented
4. ❌ **Missing critical fields** - Default models, headers, cached pricing
5. ❌ **No build/deploy infrastructure** - Docker, CI/CD missing

### Recommendation

**STOP development and complete specification first:**

1. Fix all data model incompatibilities
2. Add all 16 provider configs
3. Add all 341 models
4. Implement metrics tracking
5. Add Docker + CI/CD
6. Add config generators

**Estimated effort to reach parity: 2-3 weeks**

---

**Generated:** October 26, 2025
**Next Review:** After Phase 1 completion
