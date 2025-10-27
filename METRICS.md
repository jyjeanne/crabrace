# Crabrace Metrics

## Overview

Crabrace includes built-in Prometheus metrics for monitoring and observability.

## Available Metrics

### `crabrace_providers_requests_total`

**Type:** Counter
**Description:** Total number of requests to the `/providers` endpoint

This counter is incremented every time the `/providers` endpoint is called, regardless of whether the request succeeds or fails.

**Labels:** None

**Example:**
```
# HELP crabrace_providers_requests_total Total number of requests to the providers endpoint
# TYPE crabrace_providers_requests_total counter
crabrace_providers_requests_total 1234
```

## Accessing Metrics

### HTTP Endpoint

Metrics are exposed via the `/metrics` endpoint in Prometheus text format:

```bash
curl http://localhost:8080/metrics
```

### Example Output

```
# HELP crabrace_providers_requests_total Total number of requests to the providers endpoint
# TYPE crabrace_providers_requests_total counter
crabrace_providers_requests_total 42

# HELP process_cpu_seconds_total Total user and system CPU time spent in seconds
# TYPE process_cpu_seconds_total counter
process_cpu_seconds_total 0.15

# HELP process_resident_memory_bytes Resident memory size in bytes
# TYPE process_resident_memory_bytes gauge
process_resident_memory_bytes 8388608
```

## Prometheus Configuration

To scrape metrics from Crabrace, add this job to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'crabrace'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/metrics'
    scrape_interval: 15s
```

## Grafana Dashboard

### Example Queries

**Request Rate (per minute):**
```promql
rate(crabrace_providers_requests_total[1m]) * 60
```

**Total Requests:**
```promql
crabrace_providers_requests_total
```

**Requests in Last Hour:**
```promql
increase(crabrace_providers_requests_total[1h])
```

### Sample Dashboard Panel

```json
{
  "title": "Provider Requests per Minute",
  "targets": [
    {
      "expr": "rate(crabrace_providers_requests_total[1m]) * 60",
      "legendFormat": "Requests/min"
    }
  ],
  "type": "graph"
}
```

## Usage in Code

The metrics module is exposed in the library for custom instrumentation:

```rust
use crabrace::metrics;

// Increment the providers request counter
metrics::increment_providers_requests();

// Access the counter directly
use crabrace::metrics::PROVIDERS_REQUESTS_TOTAL;
let count = PROVIDERS_REQUESTS_TOTAL.get();
println!("Total requests: {}", count);
```

## Adding New Metrics

To add a new metric:

1. Define it in `src/metrics.rs`:

```rust
use prometheus::{register_int_counter, IntCounter};

pub static MY_METRIC: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "crabrace_my_metric_total",
        "Description of my metric"
    )
    .expect("Failed to register my_metric counter")
});
```

2. Create a helper function:

```rust
#[inline]
pub fn increment_my_metric() {
    MY_METRIC.inc();
}
```

3. Use it in your code:

```rust
use crabrace::metrics;

metrics::increment_my_metric();
```

## Metric Naming Conventions

Following Prometheus best practices:

- **Namespace:** `crabrace_` prefix for all metrics
- **Subsystem:** Group related metrics (e.g., `crabrace_http_*`, `crabrace_registry_*`)
- **Suffix:**
  - `_total` for counters
  - `_seconds` for durations
  - `_bytes` for sizes
  - No suffix for gauges

## Future Metrics (Planned)

- `crabrace_registry_providers_total` - Number of providers in registry
- `crabrace_registry_models_total` - Number of models in registry
- `crabrace_http_request_duration_seconds` - Request duration histogram
- `crabrace_http_requests_total{endpoint, status}` - All HTTP requests with labels
- `crabrace_errors_total{type}` - Error counter by type

## Alerting

### Example Prometheus Alerts

```yaml
groups:
  - name: crabrace
    rules:
      - alert: CrabraceHighRequestRate
        expr: rate(crabrace_providers_requests_total[5m]) > 100
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High request rate detected"
          description: "Crabrace is receiving {{ $value }} requests/second"

      - alert: CrabraceDown
        expr: up{job="crabrace"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Crabrace is down"
          description: "Crabrace has been down for more than 1 minute"
```

## Testing Metrics

### Manual Testing

```bash
# Start the server
cargo run

# In another terminal, make some requests
for i in {1..10}; do
  curl http://localhost:8080/providers > /dev/null 2>&1
done

# Check metrics
curl http://localhost:8080/metrics | grep crabrace_providers
```

Expected output:
```
crabrace_providers_requests_total 10
```

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use crate::metrics;

    #[test]
    fn test_metrics_increment() {
        let before = metrics::PROVIDERS_REQUESTS_TOTAL.get();
        metrics::increment_providers_requests();
        let after = metrics::PROVIDERS_REQUESTS_TOTAL.get();

        assert_eq!(after, before + 1);
    }
}
```

## Performance Considerations

- Metrics are incremented with atomic operations (lock-free)
- Minimal overhead per request (~nanoseconds)
- Metrics collection happens only on `/metrics` endpoint access
- No impact on request latency

## Security

The `/metrics` endpoint is exposed without authentication by default. In production:

1. **Restrict access** via firewall or reverse proxy
2. **Use network policies** in Kubernetes
3. **Implement authentication** if needed

Example Nginx configuration:

```nginx
location /metrics {
    allow 10.0.0.0/8;  # Internal monitoring network
    deny all;
    proxy_pass http://localhost:8080/metrics;
}
```

## References

- [Prometheus Documentation](https://prometheus.io/docs/)
- [Prometheus Rust Client](https://docs.rs/prometheus/)
- [Metric Naming Best Practices](https://prometheus.io/docs/practices/naming/)
