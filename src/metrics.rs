//! Prometheus metrics for Crabrace
//!
//! This module defines and exports Prometheus metrics used throughout the application.

use once_cell::sync::Lazy;
use prometheus::{register_int_counter, IntCounter};

/// Total number of requests to the /providers endpoint
pub static PROVIDERS_REQUESTS_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "crabrace_providers_requests_total",
        "Total number of requests to the providers endpoint"
    )
    .expect("Failed to register providers_requests_total counter")
});

/// Increment the providers request counter
#[inline]
pub fn increment_providers_requests() {
    PROVIDERS_REQUESTS_TOTAL.inc();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_registration() {
        // Just accessing the lazy static will register the metric
        let initial = PROVIDERS_REQUESTS_TOTAL.get();

        // Increment
        increment_providers_requests();

        // Verify increment
        assert_eq!(PROVIDERS_REQUESTS_TOTAL.get(), initial + 1);
    }

    #[test]
    fn test_multiple_increments() {
        let initial = PROVIDERS_REQUESTS_TOTAL.get();

        for _ in 0..5 {
            increment_providers_requests();
        }

        assert_eq!(PROVIDERS_REQUESTS_TOTAL.get(), initial + 5);
    }
}
