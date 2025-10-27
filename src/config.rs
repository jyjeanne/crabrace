use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;
use tracing::Level;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server configuration
    pub server: ServerConfig,

    /// Logging configuration
    pub logging: LoggingConfig,

    /// Metrics configuration
    pub metrics: MetricsConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to
    #[serde(default = "default_host")]
    pub host: String,

    /// Port to bind to
    #[serde(default = "default_port")]
    pub port: u16,

    /// Enable compression
    #[serde(default = "default_true")]
    pub compression: bool,

    /// Request timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,

    /// Use JSON format for logs
    #[serde(default)]
    pub json_format: bool,

    /// Show target in logs
    #[serde(default)]
    pub show_target: bool,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics endpoint
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Metrics endpoint path
    #[serde(default = "default_metrics_path")]
    pub path: String,
}

// Default value functions
fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_true() -> bool {
    true
}

fn default_timeout() -> u64 {
    30
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_metrics_path() -> String {
    "/metrics".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
            metrics: MetricsConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            compression: default_true(),
            timeout_seconds: default_timeout(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            json_format: false,
            show_target: false,
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            path: default_metrics_path(),
        }
    }
}

impl Config {
    /// Load configuration from multiple sources with precedence:
    /// 1. Environment variables (highest priority)
    /// 2. Configuration file (if provided)
    /// 3. Default values (lowest priority)
    pub fn load() -> Result<Self> {
        // Try to load .env file if it exists
        let _ = dotenvy::dotenv();

        let mut builder = config::Config::builder();

        // Start with defaults
        builder = builder.add_source(config::Config::try_from(&Config::default())?);

        // Load from config file if it exists
        let config_file = std::env::var("CRABRACE_CONFIG")
            .unwrap_or_else(|_| "config.toml".to_string());

        if Path::new(&config_file).exists() {
            builder = builder.add_source(config::File::with_name(&config_file));
        }

        // Override with environment variables
        // Environment variables should be prefixed with CRABRACE_
        // e.g., CRABRACE_SERVER__PORT=8080
        builder = builder.add_source(
            config::Environment::with_prefix("CRABRACE")
                .separator("__")
                .try_parsing(true),
        );

        let config = builder
            .build()
            .context("Failed to build configuration")?
            .try_deserialize()
            .context("Failed to deserialize configuration")?;

        Ok(config)
    }

    /// Get the socket address to bind to
    pub fn socket_addr(&self) -> Result<SocketAddr> {
        let addr = format!("{}:{}", self.server.host, self.server.port);
        addr.parse()
            .with_context(|| format!("Invalid socket address: {}", addr))
    }

    /// Get the tracing level
    pub fn tracing_level(&self) -> Level {
        match self.logging.level.to_lowercase().as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => {
                eprintln!("Invalid log level '{}', using 'info'", self.logging.level);
                Level::INFO
            }
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Validate port
        if self.server.port == 0 {
            anyhow::bail!("Server port cannot be 0");
        }

        // Validate timeout
        if self.server.timeout_seconds == 0 {
            anyhow::bail!("Server timeout cannot be 0");
        }

        // Validate log level
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.logging.level.to_lowercase().as_str()) {
            anyhow::bail!(
                "Invalid log level '{}'. Valid levels: {}",
                self.logging.level,
                valid_levels.join(", ")
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert!(config.server.compression);
        assert_eq!(config.logging.level, "info");
        assert!(config.metrics.enabled);
    }

    #[test]
    fn test_socket_addr() {
        let config = Config::default();
        let addr = config.socket_addr().unwrap();
        assert_eq!(addr.to_string(), "0.0.0.0:8080");
    }

    #[test]
    fn test_tracing_level() {
        let mut config = Config::default();

        config.logging.level = "debug".to_string();
        assert_eq!(config.tracing_level(), Level::DEBUG);

        config.logging.level = "warn".to_string();
        assert_eq!(config.tracing_level(), Level::WARN);

        config.logging.level = "info".to_string();
        assert_eq!(config.tracing_level(), Level::INFO);
    }

    #[test]
    fn test_validate_config() {
        let mut config = Config::default();

        // Valid config
        assert!(config.validate().is_ok());

        // Invalid port
        config.server.port = 0;
        assert!(config.validate().is_err());

        config.server.port = 8080;

        // Invalid timeout
        config.server.timeout_seconds = 0;
        assert!(config.validate().is_err());

        config.server.timeout_seconds = 30;

        // Invalid log level
        config.logging.level = "invalid".to_string();
        assert!(config.validate().is_err());
    }
}
