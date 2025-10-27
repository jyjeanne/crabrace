use crate::config::{CorsConfig, RateLimitConfig, SecurityHeadersConfig};
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use std::time::Duration;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;

/// Build CORS middleware layer from configuration
pub fn build_cors_layer(config: &CorsConfig) -> Option<CorsLayer> {
    if !config.enabled {
        return None;
    }

    let mut cors = CorsLayer::new();

    // Configure allowed origins
    if config.allowed_origins.contains(&"*".to_string()) {
        cors = cors.allow_origin(AllowOrigin::any());
    } else {
        let origins: Vec<HeaderValue> = config
            .allowed_origins
            .iter()
            .filter_map(|origin| origin.parse().ok())
            .collect();
        cors = cors.allow_origin(origins);
    }

    // Configure allowed methods
    let methods: Vec<Method> = config
        .allowed_methods
        .iter()
        .filter_map(|m| m.parse().ok())
        .collect();
    cors = cors.allow_methods(methods);

    // Configure allowed headers
    let headers: Vec<header::HeaderName> = config
        .allowed_headers
        .iter()
        .filter_map(|h| h.parse().ok())
        .collect();
    cors = cors.allow_headers(headers);

    // Configure max age
    cors = cors.max_age(Duration::from_secs(config.max_age_seconds));

    Some(cors)
}

/// Build rate limiting middleware layer from configuration
///
/// Note: This function currently returns None due to type compatibility issues
/// with tower_governor 0.4.3. Rate limiting will be re-enabled after upgrading
/// to a newer version that exposes the necessary types publicly.
///
/// TODO: Upgrade to tower_governor 0.8.0+ and re-implement rate limiting
pub fn build_rate_limit_layer<T>(_config: &RateLimitConfig) -> Option<T> {
    // Temporarily disabled due to tower_governor 0.4.3 type visibility issues
    // The GovernorLayer requires 2 generic arguments but the rate limiter types
    // (DefaultDirectRateLimiter, DefaultKeyedStateStore) are private.
    // This will be fixed when upgrading to tower_governor 0.8.0+
    None
}

/// Build security headers middleware layers from configuration
pub fn build_security_headers_layers(
    config: &SecurityHeadersConfig,
) -> Vec<SetResponseHeaderLayer<HeaderValue>> {
    if !config.enabled {
        return vec![];
    }

    let mut layers = Vec::new();

    // HSTS header
    if config.hsts {
        let value = HeaderValue::from_static("max-age=31536000; includeSubDomains");
        layers.push(SetResponseHeaderLayer::overriding(
            header::STRICT_TRANSPORT_SECURITY,
            value,
        ));
    }

    // X-Content-Type-Options header
    if config.content_type_options {
        let value = HeaderValue::from_static("nosniff");
        layers.push(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            value,
        ));
    }

    // X-Frame-Options header
    if config.frame_options {
        let value = HeaderValue::from_static("DENY");
        layers.push(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            value,
        ));
    }

    // X-XSS-Protection header
    if config.xss_protection {
        let value = HeaderValue::from_static("1; mode=block");
        layers.push(SetResponseHeaderLayer::overriding(
            header::X_XSS_PROTECTION,
            value,
        ));
    }

    layers
}

/// Custom rate limit error response
pub struct RateLimitError;

impl IntoResponse for RateLimitError {
    fn into_response(self) -> Response {
        (
            StatusCode::TOO_MANY_REQUESTS,
            "Too many requests. Please try again later.",
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cors_layer_disabled() {
        let mut config = CorsConfig::default();
        config.enabled = false;
        assert!(build_cors_layer(&config).is_none());
    }

    #[test]
    fn test_cors_layer_enabled() {
        let config = CorsConfig::default();
        assert!(build_cors_layer(&config).is_some());
    }

    #[test]
    fn test_rate_limit_layer_disabled() {
        let mut config = RateLimitConfig::default();
        config.enabled = false;
        // Rate limiting is temporarily disabled, so this always returns None
        let result: Option<()> = build_rate_limit_layer(&config);
        assert!(result.is_none());
    }

    #[test]
    fn test_rate_limit_layer_enabled() {
        let config = RateLimitConfig::default();
        // Rate limiting is temporarily disabled, so this always returns None
        let result: Option<()> = build_rate_limit_layer(&config);
        assert!(result.is_none());
    }

    #[test]
    fn test_security_headers_disabled() {
        let mut config = SecurityHeadersConfig::default();
        config.enabled = false;
        let layers = build_security_headers_layers(&config);
        assert!(layers.is_empty());
    }

    #[test]
    fn test_security_headers_enabled() {
        let config = SecurityHeadersConfig::default();
        let layers = build_security_headers_layers(&config);
        assert!(!layers.is_empty());
    }
}
