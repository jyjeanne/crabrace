use crate::Provider;
use anyhow::Result;
use parking_lot::RwLock;
use std::sync::Arc;

/// Embedded provider configuration files
/// These JSON files contain provider and model metadata
const ANTHROPIC_CONFIG: &str = include_str!("configs/anthropic.json");
const OPENAI_CONFIG: &str = include_str!("configs/openai.json");

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

        // Load Anthropic
        if let Ok(provider) = serde_json::from_str::<Provider>(ANTHROPIC_CONFIG) {
            providers.push(provider);
        }

        // Load OpenAI
        if let Ok(provider) = serde_json::from_str::<Provider>(OPENAI_CONFIG) {
            providers.push(provider);
        }

        // Additional providers can be loaded here as configs are added

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
        self.providers
            .read()
            .iter()
            .map(|p| p.models.len())
            .sum()
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
        assert!(registry.count() > 0);
    }
}
