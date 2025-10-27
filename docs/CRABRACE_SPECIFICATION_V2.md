# Crabrace - Complete Technical Specification v2.0

**Project:** Crabrace (Rust port of Catwalk)
**Language:** Rust
**Rust Edition:** 2021
**MSRV:** 1.75.0
**Original Project:** Catwalk (Go) - https://github.com/charmbracelet/catwalk
**Purpose:** AI Provider Database for Crustly and Rust AI Applications
**Created:** October 26, 2025
**Updated:** October 26, 2025 (Post-Catwalk Analysis)
**Version:** 2.0 (Complete Feature Parity)
**Status:** ✅ Ready for Implementation

---

## Document Revision History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | Oct 26, 2025 | Initial specification |
| **2.0** | **Oct 26, 2025** | **Complete feature parity with Catwalk after codebase analysis** |

### Changes in v2.0

- ✅ Updated data models to match Catwalk JSON format exactly
- ✅ Added all 16 providers (was 2)
- ✅ Added all 341 models (was 10)
- ✅ Fixed field naming for API compatibility
- ✅ Added cached pricing support
- ✅ Added reasoning effort fields
- ✅ Added default model selection
- ✅ Added custom headers support
- ✅ Added Prometheus metrics specification
- ✅ Added Docker multi-arch deployment
- ✅ Added CI/CD workflows
- ✅ Added config generator tools
- ✅ Complete Catwalk compatibility guaranteed

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Architecture & Design](#2-architecture--design)
3. [Complete Data Model](#3-complete-data-model)
4. [HTTP API Specification](#4-http-api-specification)
5. [Provider Registry (16 Providers)](#5-provider-registry-16-providers)
6. [All Models (341 Total)](#6-all-models-341-total)
7. [Prometheus Metrics](#7-prometheus-metrics)
8. [Client Library](#8-client-library)
9. [Docker & Deployment](#9-docker--deployment)
10. [CI/CD Workflows](#10-cicd-workflows)
11. [Config Generators](#11-config-generators)
12. [Development Guide](#12-development-guide)
13. [Testing Strategy](#13-testing-strategy)
14. [Implementation Roadmap](#14-implementation-roadmap)

---

## 1. Executive Summary

### Overview

**Crabrace** is a 100% compatible Rust port of Catwalk, providing a high-performance HTTP service that serves as a centralized database of AI inference providers and their models.

### Stats

| Metric | Value |
|--------|-------|
| **Total Providers** | 16 |
| **Total Models** | 341 |
| **HTTP Endpoints** | 3 (/providers, /health, /metrics) |
| **Source Lines** | ~1000 (estimated) |
| **Dependencies** | Tokio, Axum, Serde, Prometheus, Reqwest |
| **Binary Size** | ~8-10MB (stripped) |
| **Memory Usage** | ~5MB (idle) |
| **Startup Time** | <50ms |

### Supported Providers (16)

1. **Anthropic** - Claude models (9 models)
2. **OpenAI** - GPT models (12 models)
3. **Google Gemini** - Gemini models (3 models)
4. **Azure OpenAI** - Enterprise GPT (15 models)
5. **AWS Bedrock** - Claude on AWS (7 models)
6. **Google Vertex AI** - Gemini on GCP (3 models)
7. **xAI** - Grok models (6 models)
8. **Z.AI** - GLM models (4 models)
9. **Groq** - Fast inference (3 models)
10. **Cerebras** - Open source (10 models)
11. **Venice AI** - Privacy-focused (6 models)
12. **Chutes** - Mixed models (21 models)
13. **DeepSeek** - Reasoning models (3 models)
14. **HuggingFace** - Router aggregator (24 models)
15. **OpenRouter** - Largest selection (206 models)
16. **AIHubMix** - Multi-provider aggregator (12 models)

---

## 2. Architecture & Design

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   HTTP Clients                           │
│     (Crustly, other Rust apps, curl, browsers)          │
└────────────────────┬────────────────────────────────────┘
                     │ HTTP GET
                     ↓
┌─────────────────────────────────────────────────────────┐
│              Axum HTTP Server (Port 8080)                │
│  ┌──────────────┬──────────────┬────────────────────┐  │
│  │ /providers   │  /health     │  /metrics          │  │
│  │ (GET/HEAD)   │  (GET)       │  (GET)             │  │
│  └──────┬───────┴──────────────┴─────────┬──────────┘  │
└─────────┼────────────────────────────────┼─────────────┘
          │                                │
          ↓                                ↓
┌─────────────────────────────┐  ┌──────────────────────┐
│   Provider Registry          │  │  Prometheus Counter  │
│   (Arc<RwLock<Vec<Provider>>>)│  │  (requests_total)   │
└──────────┬──────────────────┘  └──────────────────────┘
           │
           ↓
┌─────────────────────────────────────────────────────────┐
│        Embedded JSON Configs (16 files)                 │
│  ┌───────────┬──────────┬──────────┬──────────────┐   │
│  │ anthropic │  openai  │  gemini  │  ... (13 more)│   │
│  │   .json   │  .json   │  .json   │     .json    │   │
│  └───────────┴──────────┴──────────┴──────────────┘   │
│              (compile-time embedding)                   │
└─────────────────────────────────────────────────────────┘
```

### Request Flow

```
1. Client → HTTP GET /providers
2. Axum Router → providers_handler()
3. Handler → Registry.get_all()
4. Registry → Read from Arc<RwLock<Vec<Provider>>>
5. Serialize → JSON (via Serde)
6. Prometheus → Increment counter
7. Response → 200 OK + JSON body
```

### Thread Safety

```rust
use std::sync::Arc;
use parking_lot::RwLock;

// Provider registry is shared across all threads
pub static REGISTRY: Lazy<Arc<ProviderRegistry>> = Lazy::new(|| {
    Arc::new(ProviderRegistry::new().expect("Failed to load providers"))
});

pub struct ProviderRegistry {
    providers: Arc<RwLock<Vec<Provider>>>,  // Thread-safe
}
```

---

## 3. Complete Data Model

### 3.1 Provider Structure

**Rust Definition (100% Catwalk Compatible):**

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Provider {
    /// Display name (e.g., "Anthropic", "OpenAI")
    pub name: String,

    /// Unique identifier (e.g., "anthropic", "openai")
    pub id: String,

    /// Provider type (openai, anthropic, gemini, azure, bedrock, vertexai)
    /// Note: Using serde rename since 'type' is a Rust keyword
    #[serde(rename = "type")]
    pub provider_type: String,

    /// API key placeholder (e.g., "$ANTHROPIC_API_KEY")
    /// Will be replaced by actual key at runtime by client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// API endpoint URL (e.g., "$ANTHROPIC_API_ENDPOINT", "https://api.x.ai/v1")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,

    /// Default model ID for large/complex tasks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_large_model_id: Option<String>,

    /// Default model ID for small/fast tasks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_small_model_id: Option<String>,

    /// Custom HTTP headers required by provider (e.g., APP-Code, X-Title)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_headers: Option<HashMap<String, String>>,

    /// List of available models
    #[serde(default)]
    pub models: Vec<Model>,
}
```

**JSON Example (Anthropic):**

```json
{
  "name": "Anthropic",
  "id": "anthropic",
  "type": "anthropic",
  "api_key": "$ANTHROPIC_API_KEY",
  "api_endpoint": "$ANTHROPIC_API_ENDPOINT",
  "default_large_model_id": "claude-sonnet-4-5-20250929",
  "default_small_model_id": "claude-3-5-haiku-20241022",
  "default_headers": null,
  "models": [...]
}
```

**JSON Example (AIHubMix with Custom Headers):**

```json
{
  "name": "AIHubMix",
  "id": "aihubmix",
  "type": "openai",
  "api_key": "$AIHUBMIX_API_KEY",
  "api_endpoint": "https://aihubmix.com/v1",
  "default_large_model_id": "claude-sonnet-4-5",
  "default_small_model_id": "claude-3-5-haiku",
  "default_headers": {
    "APP-Code": "IUFF7106"
  },
  "models": [...]
}
```

### 3.2 Model Structure

**Rust Definition (100% Catwalk Compatible):**

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Model {
    /// Model identifier (e.g., "claude-sonnet-4-5-20250929", "gpt-5")
    pub id: String,

    /// Human-readable name (e.g., "Claude Sonnet 4.5", "GPT-5")
    pub name: String,

    /// Cost per 1 million input tokens (USD)
    pub cost_per_1m_in: f64,

    /// Cost per 1 million output tokens (USD)
    pub cost_per_1m_out: f64,

    /// Cost per 1 million cached input tokens (USD) - for prompt caching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_1m_in_cached: Option<f64>,

    /// Cost per 1 million cached output tokens (USD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_1m_out_cached: Option<f64>,

    /// Maximum context window size in tokens
    pub context_window: u64,

    /// Default maximum output tokens
    pub default_max_tokens: u64,

    /// Whether model supports extended thinking/reasoning
    #[serde(default)]
    pub can_reason: bool,

    /// Whether model supports reasoning_effort parameter
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_reasoning_efforts: bool,  // Note: "efforts" is plural in Catwalk

    /// Default reasoning effort level (minimal, low, medium, high)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_reasoning_effort: Option<String>,

    /// Whether model supports image/attachment inputs
    #[serde(default)]
    pub supports_attachments: bool,
}

// Helper function for serde skip
fn is_false(b: &bool) -> bool {
    !b
}
```

**JSON Example (Claude Sonnet 4.5):**

```json
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
  "supports_attachments": true
}
```

**JSON Example (GPT-5 with Reasoning Effort):**

```json
{
  "id": "gpt-5",
  "name": "GPT-5",
  "cost_per_1m_in": 1.25,
  "cost_per_1m_out": 10.0,
  "cost_per_1m_in_cached": 0.25,
  "cost_per_1m_out_cached": 0.25,
  "context_window": 400000,
  "default_max_tokens": 128000,
  "can_reason": true,
  "has_reasoning_efforts": true,
  "default_reasoning_effort": "minimal",
  "supports_attachments": true
}
```

### 3.3 Helper Methods

```rust
impl Provider {
    /// Get default large model
    pub fn default_large_model(&self) -> Option<&Model> {
        self.default_large_model_id.as_ref()
            .and_then(|id| self.get_model(id))
    }

    /// Get default small model
    pub fn default_small_model(&self) -> Option<&Model> {
        self.default_small_model_id.as_ref()
            .and_then(|id| self.get_model(id))
    }

    /// Get model by ID
    pub fn get_model(&self, model_id: &str) -> Option<&Model> {
        self.models.iter().find(|m| m.id == model_id)
    }
}

impl Model {
    /// Calculate total cost for given token counts
    pub fn calculate_cost(&self, input_tokens: u64, output_tokens: u64, use_cache: bool) -> f64 {
        let input_cost = if use_cache && self.cost_per_1m_in_cached.is_some() {
            (input_tokens as f64 / 1_000_000.0) * self.cost_per_1m_in_cached.unwrap()
        } else {
            (input_tokens as f64 / 1_000_000.0) * self.cost_per_1m_in
        };

        let output_cost = if use_cache && self.cost_per_1m_out_cached.is_some() {
            (output_tokens as f64 / 1_000_000.0) * self.cost_per_1m_out_cached.unwrap()
        } else {
            (output_tokens as f64 / 1_000_000.0) * self.cost_per_1m_out
        };

        input_cost + output_cost
    }
}
```

---

## 4. HTTP API Specification

### 4.1 GET /providers

**Description:** Returns all available AI providers and their models

**Method:** `GET`, `HEAD`

**Headers:**
- Request: None required
- Response: `Content-Type: application/json`

**Status Codes:**
- `200 OK` - Success
- `405 Method Not Allowed` - Non-GET/HEAD method
- `500 Internal Server Error` - JSON encoding failure

**Response Format:**

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
        "supports_attachments": true
      }
    ]
  }
]
```

**Implementation:**

```rust
async fn providers_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Provider>>, (StatusCode, String)> {
    // Increment Prometheus counter
    PROVIDERS_REQUESTS.inc();

    match state.registry.get_all() {
        Ok(providers) => Ok(Json(providers)),
        Err(e) => {
            tracing::error!("Failed to get providers: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve providers".to_string(),
            ))
        }
    }
}
```

### 4.2 GET /health

**Description:** Health check endpoint for load balancers/orchestration

**Method:** `GET`

**Status Codes:**
- `200 OK` - Service healthy

**Response:** Plain text `"OK"`

**Implementation:**

```rust
async fn health_handler() -> &'static str {
    "OK"
}
```

**Note:** Catwalk uses `/healthz`, but `/health` is more conventional. Consider supporting both:

```rust
.route("/health", get(health_handler))
.route("/healthz", get(health_handler))  // Alias for Catwalk compatibility
```

### 4.3 GET /metrics

**Description:** Prometheus metrics endpoint

**Method:** `GET`

**Headers:**
- Response: `Content-Type: text/plain; version=0.0.4`

**Status Codes:**
- `200 OK` - Metrics available
- `500 Internal Server Error` - Encoding failure

**Response Format:** Prometheus text format

```
# HELP crabrace_providers_requests_total Total number of requests to the providers endpoint
# TYPE crabrace_providers_requests_total counter
crabrace_providers_requests_total 1234
```

**Implementation:**

```rust
async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();

    match encoder.encode(&metric_families, &mut buffer) {
        Ok(_) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/plain; version=0.0.4")],
            buffer,
        ),
        Err(e) => {
            tracing::error!("Failed to encode metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "text/plain")],
                b"Failed to encode metrics".to_vec(),
            )
        }
    }
}
```

---

## 5. Provider Registry (16 Providers)

### 5.1 Embedded Configuration System

**File Structure:**

```
src/providers/configs/
├── anthropic.json       (9 models)
├── openai.json          (12 models)
├── gemini.json          (3 models)
├── azure.json           (15 models)
├── bedrock.json         (7 models)
├── vertexai.json        (3 models)
├── xai.json             (6 models)
├── zai.json             (4 models)
├── groq.json            (3 models)
├── cerebras.json        (10 models)
├── venice.json          (6 models)
├── chutes.json          (21 models)
├── deepseek.json        (3 models)
├── huggingface.json     (24 models)
├── openrouter.json      (206 models)
└── aihubmix.json        (12 models)
```

**Registry Implementation:**

```rust
// src/providers/registry.rs
use crate::Provider;
use anyhow::Result;
use parking_lot::RwLock;
use std::sync::Arc;

// Embed all provider configs at compile time
const ANTHROPIC_CONFIG: &str = include_str!("configs/anthropic.json");
const OPENAI_CONFIG: &str = include_str!("configs/openai.json");
const GEMINI_CONFIG: &str = include_str!("configs/gemini.json");
const AZURE_CONFIG: &str = include_str!("configs/azure.json");
const BEDROCK_CONFIG: &str = include_str!("configs/bedrock.json");
const VERTEXAI_CONFIG: &str = include_str!("configs/vertexai.json");
const XAI_CONFIG: &str = include_str!("configs/xai.json");
const ZAI_CONFIG: &str = include_str!("configs/zai.json");
const GROQ_CONFIG: &str = include_str!("configs/groq.json");
const CEREBRAS_CONFIG: &str = include_str!("configs/cerebras.json");
const VENICE_CONFIG: &str = include_str!("configs/venice.json");
const CHUTES_CONFIG: &str = include_str!("configs/chutes.json");
const DEEPSEEK_CONFIG: &str = include_str!("configs/deepseek.json");
const HUGGINGFACE_CONFIG: &str = include_str!("configs/huggingface.json");
const OPENROUTER_CONFIG: &str = include_str!("configs/openrouter.json");
const AIHUBMIX_CONFIG: &str = include_str!("configs/aihubmix.json");

pub struct ProviderRegistry {
    providers: Arc<RwLock<Vec<Provider>>>,
}

impl ProviderRegistry {
    pub fn new() -> Result<Self> {
        let mut providers = Vec::with_capacity(16);

        // Load all providers
        providers.push(Self::load_provider(ANTHROPIC_CONFIG)?);
        providers.push(Self::load_provider(OPENAI_CONFIG)?);
        providers.push(Self::load_provider(GEMINI_CONFIG)?);
        providers.push(Self::load_provider(AZURE_CONFIG)?);
        providers.push(Self::load_provider(BEDROCK_CONFIG)?);
        providers.push(Self::load_provider(VERTEXAI_CONFIG)?);
        providers.push(Self::load_provider(XAI_CONFIG)?);
        providers.push(Self::load_provider(ZAI_CONFIG)?);
        providers.push(Self::load_provider(GROQ_CONFIG)?);
        providers.push(Self::load_provider(CEREBRAS_CONFIG)?);
        providers.push(Self::load_provider(VENICE_CONFIG)?);
        providers.push(Self::load_provider(CHUTES_CONFIG)?);
        providers.push(Self::load_provider(DEEPSEEK_CONFIG)?);
        providers.push(Self::load_provider(HUGGINGFACE_CONFIG)?);
        providers.push(Self::load_provider(OPENROUTER_CONFIG)?);
        providers.push(Self::load_provider(AIHUBMIX_CONFIG)?);

        Ok(Self {
            providers: Arc::new(RwLock::new(providers)),
        })
    }

    fn load_provider(json: &str) -> Result<Provider> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn get_all(&self) -> Result<Vec<Provider>> {
        Ok(self.providers.read().clone())
    }

    pub fn get_by_id(&self, id: &str) -> Result<Option<Provider>> {
        Ok(self.providers.read().iter().find(|p| p.id == id).cloned())
    }
}
```

### 5.2 Provider Summary Table

| # | Provider | ID | Type | Models | Context Range | Default Large | Default Small |
|---|----------|----|----|--------|---------------|---------------|---------------|
| 1 | Anthropic | anthropic | anthropic | 9 | 200k | claude-sonnet-4-5-20250929 | claude-3-5-haiku-20241022 |
| 2 | OpenAI | openai | openai | 12 | 128k-400k | gpt-5 | gpt-4o |
| 3 | Google Gemini | gemini | gemini | 3 | 1M | gemini-2.5-pro | gemini-2.5-flash |
| 4 | Azure OpenAI | azure | azure | 15 | 128k-1M | gpt-5 | gpt-5-mini |
| 5 | AWS Bedrock | bedrock | bedrock | 7 | 200k | claude-sonnet-4-5 | claude-3-5-haiku |
| 6 | Vertex AI | vertexai | vertexai | 3 | 1M | gemini-2.5-pro | gemini-2.5-flash |
| 7 | xAI | xai | openai | 6 | 131k-2M | grok-code-fast | grok-3-mini |
| 8 | Z.AI | zai | openai | 4 | 131k-204k | glm-4.6 | glm-4.5-air |
| 9 | Groq | groq | openai | 3 | 131k | kimi-k2-0905 | qwen3-32b |
| 10 | Cerebras | cerebras | openai | 10 | 32k-131k | qwen-3-coder-480b | qwen-3-32b |
| 11 | Venice | venice | openai | 6 | 32k-131k | qwen3-235b | mistral-31-24b |
| 12 | Chutes | chutes | openai | 21 | 32k-262k | qwen3-coder-480b | (same) |
| 13 | DeepSeek | deepseek | openai | 3 | 128k | deepseek-reasoner | deepseek-chat |
| 14 | HuggingFace | huggingface | openai | 24 | varies | kimi-k2-0905 | gpt-oss-20b |
| 15 | **OpenRouter** | **openrouter** | **openai** | **206** | **varies** | **claude-sonnet-4** | **claude-3.5-haiku** |
| 16 | AIHubMix | aihubmix | openai | 12 | 131k-1M | claude-sonnet-4-5 | claude-3-5-haiku |

**Total Models: 341**

---

## 6. All Models (341 Total)

Due to the large number of models, this section provides a categorized summary. Full JSON configs will be in `src/providers/configs/*.json`.

### 6.1 Model Categories

**By Capability:**

| Capability | Count | Examples |
|------------|-------|----------|
| **Reasoning** | ~80 | Claude Sonnet 4.5, GPT-5, o3, DeepSeek R1, GLM-4.6 |
| **Vision** | ~150 | Claude 3+, GPT-4o+, Gemini 2.5 |
| **Large Context (>200k)** | ~50 | Gemini (1M), Grok 4 Fast (2M), GPT-4.1 (1M) |
| **Cached Pricing** | ~100 | Claude models, GPT-5 series, Gemini |
| **Reasoning Effort** | ~30 | GPT-5, GLM-4.6, Kimi K2 |

**By Provider Type:**

| Type | Providers | Total Models |
|------|-----------|--------------|
| anthropic | 1 | 9 |
| openai | 1 | 12 |
| gemini | 1 | 3 |
| azure | 1 | 15 |
| bedrock | 1 | 7 |
| vertexai | 1 | 3 |
| openai-compatible | 10 | 292 |

### 6.2 Featured Models

**Top Reasoning Models:**

1. **Claude Sonnet 4.5** - $3/$15 per 1M tokens, 200k context
2. **GPT-5** - $1.25/$10 per 1M tokens, 400k context, reasoning effort
3. **o3** - $2/$8 per 1M tokens, 200k context, medium reasoning effort
4. **DeepSeek V3.1 Thinking** - $0.56/$1.68 per 1M tokens, 128k context
5. **GLM-4.6** - $0.6/$2.2 per 1M tokens, 204k context

**Top Vision Models:**

1. **GPT-4o** - $2.5/$10 per 1M tokens, 128k context
2. **Claude 3.5 Sonnet** - $3/$15 per 1M tokens, 200k context
3. **Gemini 2.5 Pro** - $1.25/$10 per 1M tokens, 1M context

**Most Affordable:**

1. **GPT-5 Nano** - $0.05/$0.4 per 1M tokens
2. **Cerebras Qwen 3 32B** - $0.1/$0.4 per 1M tokens
3. **Venice Llama 3.2 3B** - $0.15/$0.6 per 1M tokens

**Largest Context:**

1. **Grok 4 Fast** - 2M tokens
2. **Gemini 2.5 Pro/Flash** - 1M tokens
3. **Azure GPT-4.1** - 1M tokens

---

## 7. Prometheus Metrics

### 7.1 Metric Definitions

```rust
use prometheus::{register_counter, Counter};
use once_cell::sync::Lazy;

pub static PROVIDERS_REQUESTS: Lazy<Counter> = Lazy::new(|| {
    register_counter!(
        "crabrace_providers_requests_total",
        "Total number of requests to the providers endpoint"
    )
    .expect("Failed to register counter")
});
```

### 7.2 Usage

```rust
// In providers_handler
PROVIDERS_REQUESTS.inc();
```

### 7.3 Metrics Endpoint Output

```
# HELP crabrace_providers_requests_total Total number of requests to the providers endpoint
# TYPE crabrace_providers_requests_total counter
crabrace_providers_requests_total 1234

# HELP process_cpu_seconds_total Total user and system CPU time spent in seconds.
# TYPE process_cpu_seconds_total counter
process_cpu_seconds_total 0.12
...
```

---

## 8. Client Library

### 8.1 Client Structure

```rust
// src/client.rs
use crate::Provider;
use anyhow::Result;
use reqwest::Client as HttpClient;

#[derive(Debug, Clone)]
pub struct CrabraceClient {
    base_url: String,
    http_client: HttpClient,
}

impl CrabraceClient {
    /// Create client with default URL (localhost:8080) or from $CRABRACE_URL env var
    pub fn new() -> Self {
        let base_url = std::env::var("CRABRACE_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string());
        Self {
            base_url,
            http_client: HttpClient::new(),
        }
    }

    /// Create client with explicit URL
    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            base_url: url.into(),
            http_client: HttpClient::new(),
        }
    }

    /// Get all providers
    pub async fn get_providers(&self) -> Result<Vec<Provider>> {
        let url = format!("{}/providers", self.base_url);
        let response = self.http_client.get(&url).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP {}", response.status());
        }

        Ok(response.json().await?)
    }

    /// Health check
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.base_url);
        let response = self.http_client.get(&url).send().await?;
        Ok(response.status().is_success())
    }
}

impl Default for CrabraceClient {
    fn default() -> Self {
        Self::new()
    }
}
```

### 8.2 Usage Example

```rust
use crabrace::CrabraceClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Option 1: Default (uses $CRABRACE_URL or localhost:8080)
    let client = CrabraceClient::new();

    // Option 2: Explicit URL
    let client = CrabraceClient::with_url("http://production.example.com:8080");

    // Get all providers
    let providers = client.get_providers().await?;

    for provider in providers {
        println!("{}: {} models", provider.name, provider.models.len());
    }

    Ok(())
}
```

---

## 9. Docker & Deployment

### 9.1 Dockerfile

```dockerfile
# Multi-stage build
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src

# Build for release
RUN cargo build --release --locked

# Runtime stage
FROM alpine:latest

# Copy binary
COPY --from=builder /app/target/release/crabrace /usr/bin/crabrace

# Expose port
EXPOSE 8080

# Run
CMD ["/usr/bin/crabrace"]
```

### 9.2 Multi-Arch Build

```bash
# Build for multiple architectures
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t ghcr.io/your-org/crabrace:latest \
  --push .
```

### 9.3 Docker Compose

```yaml
version: '3.8'

services:
  crabrace:
    image: ghcr.io/your-org/crabrace:latest
    ports:
      - "8080:8080"
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "wget", "--quiet", "--tries=1", "--spider", "http://localhost:8080/health"]
      interval: 30s
      timeout: 5s
      retries: 3
```

### 9.4 Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: crabrace
spec:
  replicas: 3
  selector:
    matchLabels:
      app: crabrace
  template:
    metadata:
      labels:
        app: crabrace
    spec:
      containers:
      - name: crabrace
        image: ghcr.io/your-org/crabrace:latest
        ports:
        - containerPort: 8080
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: crabrace
spec:
  selector:
    app: crabrace
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8080
  type: LoadBalancer
```

---

## 10. CI/CD Workflows

### 10.1 Build Workflow

**.github/workflows/build.yml:**

```yaml
name: Build

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: 1.75.0
        override: true
        components: rustfmt, clippy

    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

    - name: Cache cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}

    - name: Cache target
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}

    - name: Check formatting
      run: cargo fmt --check

    - name: Run clippy
      run: cargo clippy -- -D warnings

    - name: Run tests
      run: cargo test --verbose

    - name: Build release
      run: cargo build --release --verbose
```

### 10.2 Release Workflow

**.github/workflows/release.yml:**

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: 1.75.0
        override: true

    - name: Build release binaries
      run: |
        cargo build --release
        strip target/release/crabrace

    - name: Create release
      uses: softprops/action-gh-release@v1
      with:
        files: |
          target/release/crabrace
        body: |
          Release ${{ github.ref_name }}

          ## What's Changed
          See CHANGELOG.md for details
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### 10.3 Docker Build Workflow

**.github/workflows/docker.yml:**

```yaml
name: Docker

on:
  push:
    branches: [ main ]
    tags:
      - 'v*'

jobs:
  docker:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Set up Docker Buildx
      uses: docker/setup-buildx-action@v3

    - name: Log in to GitHub Container Registry
      uses: docker/login-action@v3
      with:
        registry: ghcr.io
        username: ${{ github.actor }}
        password: ${{ secrets.GITHUB_TOKEN }}

    - name: Extract metadata
      id: meta
      uses: docker/metadata-action@v5
      with:
        images: ghcr.io/${{ github.repository }}
        tags: |
          type=ref,event=branch
          type=ref,event=pr
          type=semver,pattern={{version}}
          type=semver,pattern={{major}}.{{minor}}
          type=sha

    - name: Build and push
      uses: docker/build-push-action@v5
      with:
        context: .
        platforms: linux/amd64,linux/arm64
        push: true
        tags: ${{ steps.meta.outputs.tags }}
        labels: ${{ steps.meta.outputs.labels }}
        cache-from: type=gha
        cache-to: type=gha,mode=max
```

---

## 11. Config Generators

### 11.1 OpenRouter Generator

**Purpose:** Fetch 206 models from OpenRouter API and generate config

**File:** `tools/generate_openrouter.rs`

```rust
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    data: Vec<OpenRouterModel>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterModel {
    id: String,
    name: String,
    pricing: OpenRouterPricing,
    context_length: u64,
    // ... other fields
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();

    // Fetch models
    let response = client
        .get("https://openrouter.ai/api/v1/models")
        .send()
        .await?
        .json::<OpenRouterResponse>()
        .await?;

    // Filter models with tool support
    let models: Vec<_> = response.data
        .into_iter()
        .filter(|m| m.supports_tools)
        .collect();

    // Convert to Crabrace format
    // Generate JSON
    // Write to configs/openrouter.json

    Ok(())
}
```

### 11.2 HuggingFace Generator

**Purpose:** Fetch models from HuggingFace Router API

**File:** `tools/generate_huggingface.rs`

```rust
// Similar structure to OpenRouter generator
// Fetches from https://router.huggingface.co/v1/models
// Filters by supported providers (fireworks-ai, groq, cerebras, hf-inference)
```

---

## 12. Development Guide

### 12.1 Setup

```bash
# Clone repository
git clone https://github.com/your-org/crabrace.git
cd crabrace

# Install Rust 1.75+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build

# Run
cargo run

# Test
curl http://localhost:8080/providers | jq
```

### 12.2 Adding a New Provider

1. **Create JSON config** in `src/providers/configs/newprovider.json`:

```json
{
  "name": "New Provider",
  "id": "newprovider",
  "type": "openai",
  "api_key": "$NEWPROVIDER_API_KEY",
  "api_endpoint": "https://api.newprovider.com/v1",
  "default_large_model_id": "model-large",
  "default_small_model_id": "model-small",
  "default_headers": null,
  "models": [...]
}
```

2. **Add to registry** in `src/providers/registry.rs`:

```rust
const NEWPROVIDER_CONFIG: &str = include_str!("configs/newprovider.json");

// In new() method:
providers.push(Self::load_provider(NEWPROVIDER_CONFIG)?);
```

3. **Test:**

```bash
cargo run
curl http://localhost:8080/providers | jq '.[] | select(.id == "newprovider")'
```

### 12.3 Code Quality

```bash
# Format
cargo fmt

# Lint
cargo clippy -- -D warnings

# Test
cargo test

# Bench
cargo bench
```

---

## 13. Testing Strategy

### 13.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_serialization() {
        let json = include_str!("configs/anthropic.json");
        let provider: Provider = serde_json::from_str(json).unwrap();

        assert_eq!(provider.id, "anthropic");
        assert_eq!(provider.models.len(), 9);
    }

    #[test]
    fn test_cost_calculation() {
        let model = Model {
            id: "test".into(),
            name: "Test".into(),
            cost_per_1m_in: 3.0,
            cost_per_1m_out: 15.0,
            cost_per_1m_in_cached: Some(0.3),
            cost_per_1m_out_cached: Some(0.3),
            context_window: 200000,
            default_max_tokens: 5000,
            can_reason: false,
            has_reasoning_efforts: false,
            default_reasoning_effort: None,
            supports_attachments: true,
        };

        let cost = model.calculate_cost(100_000, 50_000, false);
        assert_eq!(cost, 1.05);  // (100k/1M * 3) + (50k/1M * 15)
    }
}
```

### 13.2 Integration Tests

```rust
#[tokio::test]
async fn test_providers_endpoint() {
    // Start server
    let addr = "127.0.0.1:0";
    // ... spawn server

    // Make request
    let response = reqwest::get("http://localhost:8080/providers").await.unwrap();
    assert_eq!(response.status(), 200);

    let providers: Vec<Provider> = response.json().await.unwrap();
    assert_eq!(providers.len(), 16);
}
```

### 13.3 Compatibility Tests

```rust
#[tokio::test]
async fn test_catwalk_compatibility() {
    // Compare output with Catwalk server
    let crabrace_response = get_crabrace_providers().await;
    let catwalk_response = get_catwalk_providers().await;

    // Verify identical structure
    assert_json_eq!(crabrace_response, catwalk_response);
}
```

---

## 14. Implementation Roadmap

### Phase 1: Data Model & Core (Week 1)

- [x] Create project structure
- [ ] Implement correct Provider struct with all fields
- [ ] Implement correct Model struct with all fields
- [ ] Add all 16 provider JSON configs (copy from Catwalk)
- [ ] Implement ProviderRegistry with all 16 providers
- [ ] Unit tests for data models
- [ ] Unit tests for JSON serialization

### Phase 2: HTTP Server (Week 1-2)

- [ ] Implement Axum HTTP server
- [ ] GET /providers endpoint with HEAD support
- [ ] GET /health endpoint
- [ ] GET /metrics endpoint with Prometheus counter
- [ ] Error handling (405, 500)
- [ ] Request logging

### Phase 3: Client Library (Week 2)

- [ ] Implement CrabraceClient
- [ ] Support $CRABRACE_URL environment variable
- [ ] get_providers() method
- [ ] health_check() method
- [ ] Client documentation and examples

### Phase 4: Build & Deploy (Week 2-3)

- [ ] Dockerfile (Alpine, multi-stage)
- [ ] Docker multi-arch builds (amd64, arm64)
- [ ] GitHub Actions: build workflow
- [ ] GitHub Actions: release workflow
- [ ] GitHub Actions: Docker workflow
- [ ] Kubernetes manifests

### Phase 5: Tooling (Week 3)

- [ ] OpenRouter config generator
- [ ] HuggingFace config generator
- [ ] Makefile or Justfile for common tasks
- [ ] CI linting and formatting checks

### Phase 6: Documentation & Polish (Week 3-4)

- [ ] Complete README with examples
- [ ] API documentation
- [ ] Integration guide for Crustly
- [ ] Performance benchmarks vs Catwalk
- [ ] CHANGELOG

### Phase 7: Testing & Validation (Week 4)

- [ ] Integration tests for all endpoints
- [ ] Compatibility tests vs Catwalk
- [ ] Load testing
- [ ] Security audit
- [ ] Final verification of all 341 models

### Success Criteria

- ✅ 100% API compatibility with Catwalk
- ✅ All 16 providers implemented
- ✅ All 341 models included
- ✅ JSON response identical to Catwalk
- ✅ Docker multi-arch images published
- ✅ CI/CD fully automated
- ✅ Documentation complete
- ✅ Performance targets met (startup <50ms, memory <5MB)

---

## Appendix A: Complete Cargo.toml

```toml
[package]
name = "crabrace"
version = "0.1.0"
edition = "2021"
rust-version = "1.75.0"
authors = ["Crabrace Contributors"]
description = "High-performance AI provider database service in Rust"
license = "MIT"
repository = "https://github.com/your-org/crabrace"

[[bin]]
name = "crabrace"
path = "src/main.rs"

[lib]
name = "crabrace"
path = "src/lib.rs"

[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }

# HTTP server
axum = { version = "0.7", features = ["macros"] }
tower = { version = "0.4", features = ["util", "timeout"] }
tower-http = { version = "0.5", features = ["trace", "cors", "compression-gzip"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Metrics
prometheus = { version = "0.13", features = ["process"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# HTTP client (for library)
reqwest = { version = "0.11", features = ["json"] }

# Concurrency
parking_lot = "0.12"
once_cell = "1.19"

[dev-dependencies]
tokio-test = "0.4"
criterion = "0.5"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

---

## Appendix B: File Structure

```
crabrace/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── CHANGELOG.md
├── CONTRIBUTING.md
├── .gitignore
├── Dockerfile
├── docker-compose.yml
│
├── .github/
│   └── workflows/
│       ├── build.yml
│       ├── release.yml
│       ├── docker.yml
│       └── lint.yml
│
├── docs/
│   ├── CRABRACE_SPECIFICATION_V2.md (this file)
│   ├── FEATURE_PARITY_ANALYSIS.md
│   └── API.md
│
├── src/
│   ├── main.rs                    # HTTP server binary
│   ├── lib.rs                     # Client library
│   ├── client.rs                  # HTTP client implementation
│   │
│   ├── models/
│   │   ├── mod.rs
│   │   └── provider.rs            # Provider & Model structs
│   │
│   └── providers/
│       ├── mod.rs
│       ├── registry.rs            # Provider registry
│       └── configs/
│           ├── anthropic.json     (9 models)
│           ├── openai.json        (12 models)
│           ├── gemini.json        (3 models)
│           ├── azure.json         (15 models)
│           ├── bedrock.json       (7 models)
│           ├── vertexai.json      (3 models)
│           ├── xai.json           (6 models)
│           ├── zai.json           (4 models)
│           ├── groq.json          (3 models)
│           ├── cerebras.json      (10 models)
│           ├── venice.json        (6 models)
│           ├── chutes.json        (21 models)
│           ├── deepseek.json      (3 models)
│           ├── huggingface.json   (24 models)
│           ├── openrouter.json    (206 models)
│           └── aihubmix.json      (12 models)
│
├── examples/
│   └── client_example.rs
│
├── tools/
│   ├── generate_openrouter.rs
│   └── generate_huggingface.rs
│
└── tests/
    ├── integration_tests.rs
    └── compatibility_tests.rs
```

---

## Summary

This specification defines a complete, production-ready Rust implementation of Catwalk with:

✅ **100% API Compatibility** - Identical JSON response format
✅ **All 16 Providers** - Complete provider coverage
✅ **All 341 Models** - Complete model catalog
✅ **Complete Data Model** - All fields from Catwalk
✅ **Docker Multi-Arch** - amd64 + arm64 support
✅ **CI/CD Workflows** - Automated build, test, release
✅ **Prometheus Metrics** - Observable and monitorable
✅ **Client Library** - Easy integration
✅ **Config Generators** - Automated provider updates

**This specification is ready for implementation.**

---

**Document Version:** 2.0
**Last Updated:** October 26, 2025
**Status:** ✅ Complete & Ready for Development
