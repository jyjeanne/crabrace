# Performance Testing Guide

Comprehensive guide for benchmarking and load testing Crabrace.

## Table of Contents

- [Overview](#overview)
- [Benchmark Types](#benchmark-types)
- [Running Benchmarks](#running-benchmarks)
- [Load Testing](#load-testing)
- [Interpreting Results](#interpreting-results)
- [Performance Targets](#performance-targets)
- [Optimization Tips](#optimization-tips)
- [Continuous Performance Testing](#continuous-performance-testing)

---

## Overview

Crabrace includes multiple types of performance tests:

1. **Microbenchmarks** - Unit-level performance tests using Criterion
2. **Load Tests** - HTTP endpoint performance under various loads
3. **Stress Tests** - System behavior under increasing pressure
4. **Comparison Tests** - Performance vs. Catwalk (Go implementation)

---

## Benchmark Types

### 1. Microbenchmarks (Criterion)

Located in `benches/`, these test individual component performance:

#### Provider Benchmarks
- **load_providers**: Provider data loading time
- **find_provider_by_id**: Provider lookup performance
- **find_model_across_providers**: Model search across all providers
- **serialize_all_providers**: JSON serialization of all data
- **serialize_single_provider**: JSON serialization of one provider
- **count_operations**: Provider and model counting

#### HTTP Benchmarks
- **create_client**: Client instantiation overhead
- **create_client_with_url**: Client with custom base URL
- **http_get_providers**: End-to-end HTTP GET (requires running server)
- **http_health_check**: Health check endpoint (requires running server)

### 2. Load Tests

Located in `perf-tests/`, these test HTTP endpoint throughput:

- **wrk**: High-performance HTTP benchmarking tool
- **bombardier**: Go-based load generator with detailed stats
- **Apache Bench (ab)**: Classic benchmarking tool
- **stress-test.sh**: Gradually increasing load test

---

## Running Benchmarks

### Prerequisites

```bash
# For Criterion benchmarks (included)
cargo install cargo-criterion  # Optional, for advanced features

# For load testing (choose one or more)
# wrk (recommended for Linux/macOS)
brew install wrk  # macOS
apt-get install wrk  # Ubuntu

# bombardier (cross-platform, recommended)
go install github.com/codesenberg/bombardier@latest

# Apache Bench (usually pre-installed)
apt-get install apache2-utils  # Ubuntu
brew install httpd  # macOS
```

### Microbenchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench provider_benchmarks
cargo bench http_benchmarks

# Save baseline for comparison
cargo bench --bench provider_benchmarks -- --save-baseline main

# Compare against baseline
cargo bench --bench provider_benchmarks -- --baseline main

# Generate detailed report
cargo bench -- --verbose
```

**Output Location**: `target/criterion/`

**HTML Reports**: `target/criterion/report/index.html`

### Load Testing

#### Step 1: Start the Server

```bash
# Build release version (optimized)
cargo build --release

# Run server
./target/release/crabrace

# Or with custom configuration
CRABRACE_SERVER__PORT=8080 ./target/release/crabrace
```

#### Step 2: Run Load Tests

**Using wrk:**
```bash
cd perf-tests
chmod +x load-test-wrk.sh

# Default test (30s, 4 threads, 100 connections)
./load-test-wrk.sh

# Custom parameters
DURATION=60s THREADS=8 CONNECTIONS=200 ./load-test-wrk.sh

# Test specific host
CRABRACE_HOST=http://production-server:8080 ./load-test-wrk.sh
```

**Using bombardier:**
```bash
cd perf-tests
chmod +x load-test-bombardier.sh

# Default test
./load-test-bombardier.sh

# Custom parameters
DURATION=60s CONNECTIONS=200 ./load-test-bombardier.sh

# Rate-limited test
RATE=1000 ./load-test-bombardier.sh
```

**Using Apache Bench:**
```bash
cd perf-tests
chmod +x load-test-ab.sh

# Default test (10,000 requests, 100 concurrency)
./load-test-ab.sh

# Custom parameters
REQUESTS=50000 CONCURRENCY=200 ./load-test-ab.sh
```

#### Step 3: Stress Testing

```bash
cd perf-tests
chmod +x stress-test.sh

# Run stress test with bombardier
./stress-test.sh

# Or with wrk
TOOL=wrk ./stress-test.sh
```

---

## Interpreting Results

### Criterion Output

```
load_providers          time:   [1.2345 ms 1.2456 ms 1.2567 ms]
                        change: [-2.3456% -1.2345% +0.1234%] (p = 0.05)
                        Performance has improved.
```

**Key Metrics:**
- **time**: Mean execution time with confidence interval
- **change**: Performance change vs. previous run
- **throughput**: Operations per second (for some benchmarks)

### Load Test Output

#### wrk Output
```
Running 30s test @ http://localhost:8080/providers
  4 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.23ms    2.45ms  35.67ms   89.23%
    Req/Sec     4.85k   450.23     6.12k    78.45%
  Latency Distribution
     50%    4.89ms
     75%    6.23ms
     90%    8.45ms
     99%   15.67ms
  581234 requests in 30.10s, 1.23GB read
Requests/sec:  19321.45
Transfer/sec:     41.89MB
```

**Key Metrics:**
- **Latency Avg**: Average request latency
- **Req/Sec**: Requests per second per thread
- **Latency Distribution**: Percentile breakdown
- **Requests/sec**: Total throughput
- **Transfer/sec**: Data transfer rate

#### bombardier Output
```
Statistics        Avg      Stdev        Max
  Reqs/sec     19543.21    1234.56   21234.45
  Latency        5.12ms     2.34ms     45.67ms
  Latency Distribution
     50%    4.89ms
     75%    6.12ms
     90%    8.45ms
     95%   10.23ms
     99%   18.45ms
  HTTP codes:
    1xx - 0, 2xx - 583421, 3xx - 0, 4xx - 0, 5xx - 0
```

**Key Metrics:**
- **Reqs/sec**: Requests per second (total)
- **Latency**: Response time statistics
- **HTTP codes**: Response code distribution (should be all 2xx)

### What to Look For

**Good Performance Indicators:**
- ✅ P99 latency < 50ms
- ✅ No 5xx errors
- ✅ Consistent throughput across test duration
- ✅ Low standard deviation in latency
- ✅ No connection errors

**Warning Signs:**
- ⚠️ P99 latency > 100ms
- ⚠️ 5xx errors present
- ⚠️ Throughput drops over time
- ⚠️ High latency standard deviation
- ⚠️ Connection timeouts

**Critical Issues:**
- ❌ P99 latency > 1s
- ❌ High rate of errors (>1%)
- ❌ Crash or restart during test
- ❌ Memory leaks (increasing memory over time)
- ❌ CPU pegged at 100%

---

## Performance Targets

### Microbenchmark Targets

| Operation | Target | Excellent |
|-----------|--------|-----------|
| Load all providers | < 5ms | < 2ms |
| Find provider by ID | < 100μs | < 50μs |
| Serialize all providers | < 10ms | < 5ms |
| Serialize single provider | < 1ms | < 500μs |

### Load Test Targets

| Metric | Minimum | Target | Excellent |
|--------|---------|--------|-----------|
| **Throughput** | 10k req/s | 20k req/s | 30k+ req/s |
| **P50 Latency** | < 10ms | < 5ms | < 2ms |
| **P99 Latency** | < 100ms | < 50ms | < 25ms |
| **P99.9 Latency** | < 500ms | < 200ms | < 100ms |
| **Error Rate** | < 0.1% | 0% | 0% |

### Resource Usage Targets

| Resource | Idle | Under Load | Maximum |
|----------|------|------------|---------|
| **Memory** | < 10MB | < 50MB | < 100MB |
| **CPU** | < 1% | < 80% | < 95% |
| **File Descriptors** | < 100 | < 1000 | < 10000 |

---

## Comparison: Crabrace vs. Catwalk

Expected performance improvements over Catwalk (Go):

| Metric | Catwalk (Go) | Crabrace (Rust) | Improvement |
|--------|--------------|-----------------|-------------|
| Startup Time | ~100ms | ~50ms | **2x faster** |
| Memory (idle) | ~10MB | ~5MB | **50% less** |
| Throughput | ~10k req/s | ~25k req/s | **2.5x higher** |
| P99 Latency | ~20ms | ~8ms | **2.5x faster** |
| Binary Size | ~15MB | ~8MB | **47% smaller** |
| Cold Start | ~150ms | ~75ms | **2x faster** |

*Note: Results may vary based on hardware and configuration*

---

## Optimization Tips

### 1. Build Configuration

```bash
# Use release profile with optimizations
cargo build --release

# For maximum performance (larger binary)
RUSTFLAGS="-C target-cpu=native" cargo build --release

# For smaller binary (slightly slower)
cargo build --profile release-small
```

### 2. Runtime Configuration

```toml
# config.toml
[server]
compression = true  # Enable gzip compression
timeout_seconds = 30  # Adjust based on needs

[logging]
level = "warn"  # Reduce logging overhead in production
json_format = true  # More efficient than text format

[security.rate_limit]
enabled = true
requests_per_period = 1000  # Adjust based on capacity
period_seconds = 60
```

### 3. System Tuning (Linux)

```bash
# Increase open file limit
ulimit -n 65536

# TCP tuning for high throughput
sysctl -w net.core.somaxconn=65536
sysctl -w net.ipv4.tcp_max_syn_backlog=65536
sysctl -w net.ipv4.ip_local_port_range="1024 65535"
```

### 4. Container Optimization

```dockerfile
# Use minimal base image
FROM debian:bookworm-slim

# Set resource limits
RESOURCES:
  limits:
    memory: 512Mi
    cpu: 500m
  requests:
    memory: 128Mi
    cpu: 100m
```

---

## Continuous Performance Testing

### In CI/CD

Add to `.github/workflows/performance.yml`:

```yaml
name: Performance Tests

on:
  push:
    branches: [main]
  pull_request:

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run benchmarks
        run: cargo bench --no-fail-fast
      - name: Archive results
        uses: actions/upload-artifact@v3
        with:
          name: benchmark-results
          path: target/criterion
```

### Performance Regression Detection

```bash
# Save baseline from main branch
git checkout main
cargo bench -- --save-baseline main

# Test feature branch
git checkout feature-branch
cargo bench -- --baseline main

# Fail if performance degrades > 10%
cargo bench -- --baseline main --significance-level 0.05
```

### Automated Load Testing

```bash
# Start server in background
cargo run --release &
SERVER_PID=$!
sleep 2

# Run load test
./perf-tests/load-test-bombardier.sh

# Stop server
kill $SERVER_PID
```

---

## Profiling

### CPU Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Profile with perf (Linux)
cargo flamegraph --bin crabrace

# Or with dtrace (macOS)
cargo flamegraph --bin crabrace -- --release
```

### Memory Profiling

```bash
# Install valgrind
sudo apt-get install valgrind

# Run with valgrind
valgrind --tool=massif --massif-out-file=massif.out \
    ./target/release/crabrace

# Analyze results
ms_print massif.out
```

### Async Profiling

```bash
# Enable tokio-console support
cargo build --features tokio-console

# Run with console
tokio-console
```

---

## Troubleshooting

### Benchmarks Running Slow

1. **Ensure release build**: `cargo bench` uses release by default
2. **Close other applications**: Free up system resources
3. **Check for thermal throttling**: Monitor CPU temperature
4. **Disable power saving**: Use high-performance power mode

### Load Tests Failing

1. **Check file descriptor limits**: `ulimit -n`
2. **Increase TIME_WAIT**: Adjust TCP settings
3. **Monitor system resources**: `htop`, `vmstat`
4. **Check for port exhaustion**: `netstat -an | grep TIME_WAIT | wc -l`

### Inconsistent Results

1. **Run multiple iterations**: Average over 3-5 runs
2. **Warm up the system**: Run a short test first
3. **Control for external factors**: Same hardware, same load
4. **Check for background processes**: `top`, `ps aux`

---

## Resources

### Tools
- [Criterion](https://github.com/bheisler/criterion.rs) - Rust benchmarking framework
- [wrk](https://github.com/wg/wrk) - Modern HTTP benchmarking tool
- [bombardier](https://github.com/codesenberg/bombardier) - Fast cross-platform HTTP benchmarking tool
- [Apache Bench](https://httpd.apache.org/docs/2.4/programs/ab.html) - Classic HTTP server benchmarking
- [flamegraph](https://github.com/flamegraph-rs/flamegraph) - Rust flamegraph profiler

### Guides
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Criterion User Guide](https://bheisler.github.io/criterion.rs/book/)
- [wrk Documentation](https://github.com/wg/wrk/wiki)

---

## Contributing

When submitting performance-related changes:

1. **Run benchmarks before and after**: `cargo bench`
2. **Include results in PR**: Copy Criterion output
3. **Document optimization**: Explain what changed and why
4. **Test under load**: Run load tests to verify
5. **Profile if needed**: Include flamegraphs for significant changes

---

**Last Updated:** 2025-10-27
