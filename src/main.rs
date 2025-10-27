use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use prometheus::{Encoder, TextEncoder};
use std::sync::Arc;
use tokio::signal;
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::info;

use crabrace::{metrics, providers::registry::ProviderRegistry, Config};

/// Application state shared across handlers
#[derive(Clone)]
struct AppState {
    registry: Arc<ProviderRegistry>,
    config: Arc<Config>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load()?;
    config.validate()?;

    // Initialize tracing with configuration
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(config.tracing_level())
        .with_target(config.logging.show_target);

    if config.logging.json_format {
        subscriber.json().init();
    } else {
        subscriber.init();
    }

    info!("Starting Crabrace HTTP server...");
    info!(
        "Configuration loaded: host={}, port={}, log_level={}",
        config.server.host, config.server.port, config.logging.level
    );

    // Initialize provider registry
    let registry = Arc::new(ProviderRegistry::new()?);
    info!(
        "Provider registry loaded: {} providers with {} models",
        registry.count(),
        registry.model_count()
    );

    let config_arc = Arc::new(config.clone());
    let state = AppState {
        registry,
        config: config_arc,
    };

    // Build application routes
    let mut app = Router::new()
        .route("/providers", get(providers_handler))
        .route("/health", get(health_handler));

    // Add metrics endpoint if enabled
    if config.metrics.enabled {
        app = app.route(&config.metrics.path, get(metrics_handler));
        info!("Metrics endpoint enabled at {}", config.metrics.path);
    }

    app = app
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(config.tracing_level())),
        );

    // Add compression if enabled
    if config.server.compression {
        app = app.layer(CompressionLayer::new());
        info!("HTTP compression enabled");
    }

    // Start server
    let addr = config.socket_addr()?;
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// GET /providers - Returns all AI providers and their models
async fn providers_handler(State(state): State<AppState>) -> Response {
    // Increment Prometheus counter
    metrics::increment_providers_requests();

    match state.registry.get_all() {
        Ok(providers) => {
            info!(
                "Returned {} providers with {} total models",
                providers.len(),
                providers.iter().map(|p| p.models.len()).sum::<usize>()
            );
            (StatusCode::OK, Json(providers)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get providers: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to retrieve providers"
                })),
            )
                .into_response()
        }
    }
}

/// GET /health - Health check endpoint
async fn health_handler() -> Response {
    (StatusCode::OK, "OK").into_response()
}

/// GET /metrics - Prometheus metrics endpoint
async fn metrics_handler() -> Response {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();

    match encoder.encode(&metric_families, &mut buffer) {
        Ok(_) => (
            StatusCode::OK,
            [(
                axum::http::header::CONTENT_TYPE,
                "text/plain; version=0.0.4",
            )],
            buffer,
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to encode metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to encode metrics",
            )
                .into_response()
        }
    }
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal");
        },
        _ = terminate => {
            info!("Received terminate signal");
        },
    }

    info!("Shutting down gracefully...");
}
