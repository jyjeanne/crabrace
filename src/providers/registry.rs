use crate::Provider;
use anyhow::Result;
use parking_lot::RwLock;
use std::sync::Arc;

/// Embedded provider configuration files
/// These JSON files contain provider and model metadata
const ANTHROPIC_CONFIG: &str = include_str!("configs/anthropic.json");
const OPENAI_CONFIG: &str = include_str!("configs/openai.json");
const GEMINI_CONFIG: &str = include_str!("configs/gemini.json");
const AZURE_CONFIG: &str = include_str!("configs/azure.json");
const BEDROCK_CONFIG: &str = include_str!("configs/bedrock.json");
const VERTEXAI_CONFIG: &str = include_str!("configs/vertexai.json");
const XAI_CONFIG: &str = include_str!("configs/xai.json");
const ZAI_CONFIG: &str = include_str!("configs/zai.json");
const GROQ_CONFIG: &str = include_str!("configs/groq.json");
const OPENROUTER_CONFIG: &str = include_str!("configs/openrouter.json");
const CEREBRAS_CONFIG: &str = include_str!("configs/cerebras.json");
const VENICE_CONFIG: &str = include_str!("configs/venice.json");
const CHUTES_CONFIG: &str = include_str!("configs/chutes.json");
const DEEPSEEK_CONFIG: &str = include_str!("configs/deepseek.json");
const HUGGINGFACE_CONFIG: &str = include_str!("configs/huggingface.json");
const AIHUBMIX_CONFIG: &str = include_str!("configs/aihubmix.json");

/// Provider registry that manages all available AI providers
pub struct ProviderRegistry {
    providers: Arc<RwLock<Vec<Provider>>>,
}

impl ProviderRegistry {
    /// Create a new provider registry and load all providers
    pub fn new() -> Result<Self> {
        let registry = Self {
            providers: Arc::new(RwLock::new(Vec::new())),
        };

        registry.load_providers()?;
        Ok(registry)
    }

    /// Load all provider configurations from embedded JSON files
    fn load_providers(&self) -> Result<()> {
        let mut providers = self.providers.write();

        // Helper macro to load a provider configuration
        macro_rules! load_provider {
            ($config:expr, $name:expr) => {
                if let Ok(provider) = serde_json::from_str::<Provider>($config) {
                    providers.push(provider);
                } else {
                    tracing::warn!("Failed to load {} provider configuration", $name);
                }
            };
        }

        // Load all provider configurations
        load_provider!(ANTHROPIC_CONFIG, "Anthropic");
        load_provider!(OPENAI_CONFIG, "OpenAI");
        load_provider!(GEMINI_CONFIG, "Gemini");
        load_provider!(AZURE_CONFIG, "Azure");
        load_provider!(BEDROCK_CONFIG, "Bedrock");
        load_provider!(VERTEXAI_CONFIG, "VertexAI");
        load_provider!(XAI_CONFIG, "xAI");
        load_provider!(ZAI_CONFIG, "zAI");
        load_provider!(GROQ_CONFIG, "Groq");
        load_provider!(OPENROUTER_CONFIG, "OpenRouter");
        load_provider!(CEREBRAS_CONFIG, "Cerebras");
        load_provider!(VENICE_CONFIG, "Venice");
        load_provider!(CHUTES_CONFIG, "Chutes");
        load_provider!(DEEPSEEK_CONFIG, "DeepSeek");
        load_provider!(HUGGINGFACE_CONFIG, "HuggingFace");
        load_provider!(AIHUBMIX_CONFIG, "AIHubMix");

        Ok(())
    }

    /// Get all providers
    pub fn get_all(&self) -> Result<Vec<Provider>> {
        let providers = self.providers.read();
        Ok(providers.clone())
    }

    /// Get a specific provider by ID
    pub fn get_by_id(&self, id: &str) -> Result<Option<Provider>> {
        let providers = self.providers.read();
        Ok(providers.iter().find(|p| p.id == id).cloned())
    }

    /// Get a specific model from a provider
    pub fn get_model(&self, provider_id: &str, model_id: &str) -> Result<Option<crate::Model>> {
        let providers = self.providers.read();
        Ok(providers
            .iter()
            .find(|p| p.id == provider_id)
            .and_then(|p| p.get_model(model_id))
            .cloned())
    }

    /// Get the total number of providers
    pub fn count(&self) -> usize {
        self.providers.read().len()
    }

    /// Get the total number of models across all providers
    pub fn model_count(&self) -> usize {
        self.providers.read().iter().map(|p| p.models.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ProviderRegistry::new();
        assert!(registry.is_ok());
    }

    #[test]
    fn test_get_all_providers() {
        let registry = ProviderRegistry::new().unwrap();
        let providers = registry.get_all().unwrap();
        assert!(!providers.is_empty());
    }

    #[test]
    fn test_provider_count() {
        let registry = ProviderRegistry::new().unwrap();
        // Should have 16 providers loaded
        assert_eq!(registry.count(), 16);
    }

    #[test]
    fn test_all_providers_loaded() {
        let registry = ProviderRegistry::new().unwrap();
        let providers = registry.get_all().unwrap();

        // Check that all expected providers are present
        let expected_providers = vec![
            "anthropic",
            "openai",
            "gemini",
            "azure",
            "bedrock",
            "vertexai",
            "xai",
            "zai",
            "groq",
            "openrouter",
            "cerebras",
            "venice",
            "chutes",
            "deepseek",
            "huggingface",
            "aihubmix",
        ];

        assert_eq!(providers.len(), expected_providers.len());

        for expected_id in expected_providers {
            assert!(
                providers.iter().any(|p| p.id == expected_id),
                "Provider '{}' not found in registry",
                expected_id
            );
        }
    }

    #[test]
    fn test_get_provider_by_id() {
        let registry = ProviderRegistry::new().unwrap();

        // Test getting specific providers
        let anthropic = registry.get_by_id("anthropic").unwrap();
        assert!(anthropic.is_some());
        assert_eq!(anthropic.unwrap().id, "anthropic");

        let openai = registry.get_by_id("openai").unwrap();
        assert!(openai.is_some());
        assert_eq!(openai.unwrap().id, "openai");

        // Test non-existent provider
        let nonexistent = registry.get_by_id("nonexistent").unwrap();
        assert!(nonexistent.is_none());
    }
}
