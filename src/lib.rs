//! # Crabrace - AI Provider Database
//!
//! A high-performance, memory-safe HTTP-based AI provider database service written in Rust.
//! This is a Rust port of [Catwalk](https://github.com/charmbracelet/catwalk).
//!
//! ## Features
//!
//! - **Provider Metadata** - Up-to-date information about 16+ AI providers
//! - **Model Information** - Costs, capabilities, context windows
//! - **RESTful API** - Simple HTTP endpoints for querying
//! - **Observable** - Built-in Prometheus metrics
//! - **Client Library** - Async HTTP client for easy integration
//!
//! ## Example Usage
//!
//! ```no_run
//! use crabrace::CrabraceClient;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = CrabraceClient::new("http://localhost:8080");
//!     let providers = client.get_providers().await?;
//!
//!     for provider in providers {
//!         println!("Provider: {} ({} models)", provider.name, provider.models.len());
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod metrics;
pub mod models;
pub mod providers;
pub mod security;

pub use config::Config;
pub use models::provider::{Model, Provider};

use anyhow::Result;
use reqwest::Client as HttpClient;

/// Crabrace HTTP client for querying provider information
#[derive(Debug, Clone)]
pub struct CrabraceClient {
    base_url: String,
    http_client: HttpClient,
}

impl CrabraceClient {
    /// Create a new Crabrace client
    ///
    /// # Arguments
    ///
    /// * `base_url` - Base URL of the Crabrace server (e.g., "http://localhost:8080")
    ///
    /// # Example
    ///
    /// ```
    /// use crabrace::CrabraceClient;
    ///
    /// let client = CrabraceClient::new("http://localhost:8080");
    /// ```
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            http_client: HttpClient::new(),
        }
    }

    /// Create a new client with a custom HTTP client
    ///
    /// # Arguments
    ///
    /// * `base_url` - Base URL of the Crabrace server
    /// * `http_client` - Custom reqwest HTTP client
    pub fn with_client(base_url: impl Into<String>, http_client: HttpClient) -> Self {
        Self {
            base_url: base_url.into(),
            http_client,
        }
    }

    /// Get all available AI providers and their models
    ///
    /// # Returns
    ///
    /// A vector of `Provider` objects containing provider metadata and model information
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The response cannot be parsed as JSON
    /// - The server returns a non-200 status code
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use crabrace::CrabraceClient;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = CrabraceClient::new("http://localhost:8080");
    /// let providers = client.get_providers().await?;
    ///
    /// for provider in providers {
    ///     println!("Provider: {}", provider.name);
    ///     for model in &provider.models {
    ///         println!("  - {} ({})", model.name, model.id);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_providers(&self) -> Result<Vec<Provider>> {
        let url = format!("{}/providers", self.base_url);
        let response = self.http_client.get(&url).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get providers: HTTP {}", response.status());
        }

        let providers: Vec<Provider> = response.json().await?;
        Ok(providers)
    }

    /// Check if the Crabrace server is healthy
    ///
    /// # Returns
    ///
    /// `true` if the server is healthy, `false` otherwise
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use crabrace::CrabraceClient;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = CrabraceClient::new("http://localhost:8080");
    /// if client.health_check().await? {
    ///     println!("Server is healthy");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.base_url);
        let response = self.http_client.get(&url).send().await?;
        Ok(response.status().is_success())
    }
}

impl Default for CrabraceClient {
    fn default() -> Self {
        Self::new("http://localhost:8080")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = CrabraceClient::new("http://localhost:8080");
        assert_eq!(client.base_url, "http://localhost:8080");
    }

    #[test]
    fn test_client_default() {
        let client = CrabraceClient::default();
        assert_eq!(client.base_url, "http://localhost:8080");
    }
}
