use crate::config::{CorsConfig, RateLimitConfig, SecurityHeadersConfig};
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::GovernorLayer;
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
pub fn build_rate_limit_layer(config: &RateLimitConfig) -> Option<GovernorLayer> {
    if !config.enabled {
        return None;
    }

    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(config.period_seconds)
            .burst_size(config.requests_per_period)
            .finish()
            .expect("Failed to build rate limiter config"),
    );

    Some(GovernorLayer {
        config: Box::leak(governor_conf),
    })
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
        if let Ok(value) = HeaderValue::from_static("max-age=31536000; includeSubDomains") {
            layers.push(SetResponseHeaderLayer::overriding(
                header::STRICT_TRANSPORT_SECURITY,
                value,
            ));
        }
    }

    // X-Content-Type-Options header
    if config.content_type_options {
        if let Ok(value) = HeaderValue::from_static("nosniff") {
            layers.push(SetResponseHeaderLayer::overriding(
                header::X_CONTENT_TYPE_OPTIONS,
                value,
            ));
        }
    }

    // X-Frame-Options header
    if config.frame_options {
        if let Ok(value) = HeaderValue::from_static("DENY") {
            layers.push(SetResponseHeaderLayer::overriding(
                header::X_FRAME_OPTIONS,
                value,
            ));
        }
    }

    // X-XSS-Protection header
    if config.xss_protection {
        if let Ok(value) = HeaderValue::from_static("1; mode=block") {
            layers.push(SetResponseHeaderLayer::overriding(
                header::X_XSS_PROTECTION,
                value,
            ));
        }
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
        assert!(build_rate_limit_layer(&config).is_none());
    }

    #[test]
    fn test_rate_limit_layer_enabled() {
        let config = RateLimitConfig::default();
        assert!(build_rate_limit_layer(&config).is_some());
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
