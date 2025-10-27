use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use prometheus::{Encoder, TextEncoder};
use std::{net::SocketAddr, sync::Arc};
use tokio::signal;
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::{info, Level};

use crabrace::{metrics, providers::registry::ProviderRegistry};

/// Application state shared across handlers
#[derive(Clone)]
struct AppState {
    registry: Arc<ProviderRegistry>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    info!("Starting Crabrace HTTP server...");

    // Initialize provider registry
    let registry = Arc::new(ProviderRegistry::new()?);
    let state = AppState { registry };

    // Build application routes
    let app = Router::new()
        .route("/providers", get(providers_handler))
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler))
        .with_state(state)
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().level(Level::INFO)),
        );

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
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
