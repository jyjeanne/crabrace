use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an AI inference provider (e.g., Anthropic, OpenAI, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Provider {
    /// Display name of the provider (e.g., "Anthropic", "OpenAI")
    pub name: String,

    /// Unique identifier for the provider (e.g., "anthropic", "openai")
    pub id: String,

    /// Provider type/category (serialized as "type" in JSON)
    #[serde(rename = "type")]
    pub provider_type: String,

    /// API key placeholder (e.g., "$ANTHROPIC_API_KEY")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// API endpoint URL (serialized as "api_endpoint" in JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,

    /// Default model ID for large/complex tasks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_large_model_id: Option<String>,

    /// Default model ID for small/fast tasks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_small_model_id: Option<String>,

    /// Custom HTTP headers required by provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_headers: Option<HashMap<String, String>>,

    /// List of models available from this provider
    #[serde(default)]
    pub models: Vec<Model>,
}

/// Represents an AI model with its capabilities and pricing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Model {
    /// Unique model identifier (e.g., "claude-sonnet-4-5-20250929")
    pub id: String,

    /// Human-readable model name (e.g., "Claude Sonnet 4.5")
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

    /// Whether the model supports extended thinking/reasoning
    #[serde(default)]
    pub can_reason: bool,

    /// Whether the model supports reasoning_effort parameter
    #[serde(default)]
    pub has_reasoning_efforts: bool,

    /// Default reasoning effort level (minimal, low, medium, high)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_reasoning_effort: Option<String>,

    /// Whether the model supports image/attachment inputs
    #[serde(default)]
    pub supports_attachments: bool,
}

impl Provider {
    /// Create a new provider
    pub fn new(name: String, id: String, provider_type: String) -> Self {
        Self {
            name,
            id,
            provider_type,
            api_key: None,
            api_endpoint: None,
            default_large_model_id: None,
            default_small_model_id: None,
            default_headers: None,
            models: Vec::new(),
        }
    }

    /// Add a model to this provider
    pub fn with_model(mut self, model: Model) -> Self {
        self.models.push(model);
        self
    }

    /// Add multiple models to this provider
    pub fn with_models(mut self, models: Vec<Model>) -> Self {
        self.models.extend(models);
        self
    }

    /// Set the API endpoint for this provider
    pub fn with_api_endpoint(mut self, api_endpoint: String) -> Self {
        self.api_endpoint = Some(api_endpoint);
        self
    }

    /// Get a model by ID
    pub fn get_model(&self, model_id: &str) -> Option<&Model> {
        self.models.iter().find(|m| m.id == model_id)
    }

    /// Get the default large model
    pub fn default_large_model(&self) -> Option<&Model> {
        self.default_large_model_id
            .as_ref()
            .and_then(|id| self.get_model(id))
    }

    /// Get the default small model
    pub fn default_small_model(&self) -> Option<&Model> {
        self.default_small_model_id
            .as_ref()
            .and_then(|id| self.get_model(id))
    }
}

impl Model {
    /// Create a new model with basic information
    pub fn new(
        id: String,
        name: String,
        cost_per_1m_in: f64,
        cost_per_1m_out: f64,
        context_window: u64,
        default_max_tokens: u64,
    ) -> Self {
        Self {
            id,
            name,
            cost_per_1m_in,
            cost_per_1m_out,
            cost_per_1m_in_cached: None,
            cost_per_1m_out_cached: None,
            context_window,
            default_max_tokens,
            can_reason: false,
            has_reasoning_efforts: false,
            default_reasoning_effort: None,
            supports_attachments: false,
        }
    }

    /// Calculate cost for a given number of input and output tokens
    ///
    /// Returns the total cost in USD
    /// If use_cache is true and cached pricing is available, uses cached pricing
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

    /// Check if the given token count fits within the context window
    pub fn fits_in_context(&self, tokens: u64) -> bool {
        tokens <= self.context_window
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = Provider::new(
            "Anthropic".to_string(),
            "anthropic".to_string(),
            "anthropic".to_string(),
        );

        assert_eq!(provider.name, "Anthropic");
        assert_eq!(provider.id, "anthropic");
        assert_eq!(provider.models.len(), 0);
    }

    #[test]
    fn test_model_cost_calculation() {
        let model = Model::new(
            "test-model".to_string(),
            "Test Model".to_string(),
            3.0,  // $3 per 1M input tokens
            15.0, // $15 per 1M output tokens
            200_000,
            5000,
        );

        // Test with 100k input and 50k output tokens (no caching)
        let cost = model.calculate_cost(100_000, 50_000, false);
        // (100k / 1M * $3) + (50k / 1M * $15) = $0.30 + $0.75 = $1.05
        assert_eq!(cost, 1.05);
    }

    #[test]
    fn test_model_cost_calculation_with_cache() {
        let mut model = Model::new(
            "test-model".to_string(),
            "Test Model".to_string(),
            3.0,
            15.0,
            200_000,
            5000,
        );
        model.cost_per_1m_in_cached = Some(0.3);
        model.cost_per_1m_out_cached = Some(0.3);

        // Test with caching
        let cost = model.calculate_cost(100_000, 50_000, true);
        // (100k / 1M * $0.3) + (50k / 1M * $0.3) = $0.03 + $0.015 = $0.045
        assert_eq!(cost, 0.045);
    }

    #[test]
    fn test_context_window() {
        let model = Model::new(
            "test-model".to_string(),
            "Test Model".to_string(),
            3.0,
            15.0,
            200_000,
            5000,
        );

        assert!(model.fits_in_context(100_000));
        assert!(model.fits_in_context(200_000));
        assert!(!model.fits_in_context(200_001));
    }

    #[test]
    fn test_model_capabilities() {
        let mut model = Model::new(
            "test-model".to_string(),
            "Test Model".to_string(),
            3.0,
            15.0,
            200_000,
            5000,
        );

        model.supports_attachments = true;
        model.can_reason = true;

        assert!(model.supports_attachments);
        assert!(model.can_reason);
    }

    #[test]
    fn test_provider_with_models() {
        let model = Model::new(
            "test-model".to_string(),
            "Test Model".to_string(),
            3.0,
            15.0,
            200_000,
            5000,
        );

        let provider = Provider::new(
            "Test Provider".to_string(),
            "test".to_string(),
            "test".to_string(),
        )
        .with_model(model);

        assert_eq!(provider.models.len(), 1);
        assert_eq!(provider.models[0].id, "test-model");
    }

    #[test]
    fn test_default_models() {
        let large_model = Model::new(
            "large-model".to_string(),
            "Large Model".to_string(),
            3.0,
            15.0,
            200_000,
            50000,
        );

        let small_model = Model::new(
            "small-model".to_string(),
            "Small Model".to_string(),
            1.0,
            5.0,
            100_000,
            8000,
        );

        let mut provider = Provider::new(
            "Test Provider".to_string(),
            "test".to_string(),
            "test".to_string(),
        );

        provider.default_large_model_id = Some("large-model".to_string());
        provider.default_small_model_id = Some("small-model".to_string());
        provider = provider.with_model(large_model).with_model(small_model);

        assert!(provider.default_large_model().is_some());
        assert_eq!(provider.default_large_model().unwrap().id, "large-model");
        assert!(provider.default_small_model().is_some());
        assert_eq!(provider.default_small_model().unwrap().id, "small-model");
    }
}
