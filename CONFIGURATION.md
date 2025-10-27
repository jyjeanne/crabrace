# Configuration Guide

Complete guide for configuring Crabrace.

## Table of Contents

- [Overview](#overview)
- [Configuration Methods](#configuration-methods)
- [Configuration Options](#configuration-options)
- [Examples](#examples)
- [Environment Variables](#environment-variables)
- [Docker Configuration](#docker-configuration)
- [Best Practices](#best-practices)

---

## Overview

Crabrace supports multiple configuration methods with the following precedence (highest to lowest):

1. **Environment Variables** (highest priority)
2. **Configuration File** (TOML)
3. **Default Values** (lowest priority)

This allows for flexible deployment scenarios while maintaining secure defaults.

---

## Configuration Methods

### 1. Environment Variables

The most flexible method, ideal for containerized deployments and CI/CD pipelines.

```bash
# Server configuration
export CRABRACE_SERVER__HOST=0.0.0.0
export CRABRACE_SERVER__PORT=8080

# Logging configuration
export CRABRACE_LOGGING__LEVEL=debug

# Start the server
./crabrace
```

**Note:** Use double underscore (`__`) to separate nested configuration keys.

### 2. Configuration File (TOML)

Create a `config.toml` file for persistent configuration:

```toml
[server]
host = "0.0.0.0"
port = 8080

[logging]
level = "info"
```

**Custom config file location:**
```bash
export CRABRACE_CONFIG=/path/to/config.toml
./crabrace
```

### 3. .env File

For local development, use a `.env` file:

```bash
# Copy example file
cp .env.example .env

# Edit as needed
nano .env

# Run (dotenvy will load .env automatically)
./crabrace
```

---

## Configuration Options

### Server Configuration

```toml
[server]
# Host address to bind to
# Default: "0.0.0.0" (all interfaces)
# Examples: "0.0.0.0", "127.0.0.1", "::1"
host = "0.0.0.0"

# Port to listen on
# Default: 8080
# Valid range: 1-65535
port = 8080

# Enable HTTP compression (gzip)
# Default: true
# Reduces bandwidth usage by ~70-90%
compression = true

# Request timeout in seconds
# Default: 30
# Prevents hanging requests
timeout_seconds = 30
```

**Environment Variables:**
```bash
CRABRACE_SERVER__HOST=0.0.0.0
CRABRACE_SERVER__PORT=8080
CRABRACE_SERVER__COMPRESSION=true
CRABRACE_SERVER__TIMEOUT_SECONDS=30
```

### Logging Configuration

```toml
[logging]
# Log level
# Default: "info"
# Options: "trace", "debug", "info", "warn", "error"
level = "info"

# Use JSON format for structured logging
# Default: false
# Recommended for production monitoring
json_format = false

# Show target (module path) in logs
# Default: false
# Useful for debugging
show_target = false
```

**Environment Variables:**
```bash
CRABRACE_LOGGING__LEVEL=info
CRABRACE_LOGGING__JSON_FORMAT=false
CRABRACE_LOGGING__SHOW_TARGET=false
```

**Log Level Guide:**
- `trace`: Very detailed, includes all events
- `debug`: Detailed diagnostic information
- `info`: General informational messages (default)
- `warn`: Warning messages for potentially harmful situations
- `error`: Error messages for serious problems

### Metrics Configuration

```toml
[metrics]
# Enable Prometheus metrics endpoint
# Default: true
enabled = true

# Metrics endpoint path
# Default: "/metrics"
path = "/metrics"
```

**Environment Variables:**
```bash
CRABRACE_METRICS__ENABLED=true
CRABRACE_METRICS__PATH=/metrics
```

---

## Examples

### Development Configuration

`config.development.toml`:
```toml
[server]
host = "127.0.0.1"
port = 3000
compression = false
timeout_seconds = 60

[logging]
level = "debug"
json_format = false
show_target = true

[metrics]
enabled = true
path = "/metrics"
```

Usage:
```bash
export CRABRACE_CONFIG=config.development.toml
cargo run
```

### Production Configuration

`config.production.toml`:
```toml
[server]
host = "0.0.0.0"
port = 8080
compression = true
timeout_seconds = 30

[logging]
level = "warn"
json_format = true
show_target = false

[metrics]
enabled = true
path = "/metrics"
```

Usage:
```bash
export CRABRACE_CONFIG=config.production.toml
./crabrace
```

### Testing Configuration

For automated testing:
```bash
export CRABRACE_SERVER__PORT=0  # Random available port
export CRABRACE_LOGGING__LEVEL=error
export CRABRACE_METRICS__ENABLED=false
cargo test
```

---

## Environment Variables

### Complete Environment Variable List

```bash
# Configuration file path (optional)
CRABRACE_CONFIG=/path/to/config.toml

# Server
CRABRACE_SERVER__HOST=0.0.0.0
CRABRACE_SERVER__PORT=8080
CRABRACE_SERVER__COMPRESSION=true
CRABRACE_SERVER__TIMEOUT_SECONDS=30

# Logging
CRABRACE_LOGGING__LEVEL=info
CRABRACE_LOGGING__JSON_FORMAT=false
CRABRACE_LOGGING__SHOW_TARGET=false

# Metrics
CRABRACE_METRICS__ENABLED=true
CRABRACE_METRICS__PATH=/metrics
```

### Environment Variable Precedence

Environment variables **always override** configuration file settings:

```toml
# config.toml
[server]
port = 8080
```

```bash
# This will use port 9000, not 8080
export CRABRACE_SERVER__PORT=9000
./crabrace
```

---

## Docker Configuration

### Using Environment Variables

```bash
docker run -d \
  --name crabrace \
  -p 8080:8080 \
  -e CRABRACE_LOGGING__LEVEL=debug \
  -e CRABRACE_SERVER__COMPRESSION=true \
  crabrace:latest
```

### Using Configuration File

```bash
# Mount custom config file
docker run -d \
  --name crabrace \
  -p 8080:8080 \
  -v $(pwd)/config.toml:/app/config.toml:ro \
  crabrace:latest
```

### Docker Compose

```yaml
services:
  crabrace:
    image: crabrace:latest
    environment:
      - CRABRACE_LOGGING__LEVEL=info
      - CRABRACE_SERVER__PORT=8080
    # Or mount config file
    volumes:
      - ./config.production.toml:/app/config.toml:ro
```

**Using .env file with Docker Compose:**

```bash
# .env file for Docker Compose
CRABRACE_PORT=8080
LOG_LEVEL=info
```

```yaml
# docker-compose.yml
services:
  crabrace:
    ports:
      - "${CRABRACE_PORT:-8080}:8080"
    environment:
      - CRABRACE_LOGGING__LEVEL=${LOG_LEVEL:-info}
```

---

## Best Practices

### 1. Security

**DO:**
- Use environment variables for sensitive configuration in production
- Restrict file permissions on config files: `chmod 600 config.toml`
- Never commit `.env` or `config.toml` to version control
- Use `.env.example` and `config.toml.example` for templates

**DON'T:**
- Store secrets in configuration files
- Use default configurations in production
- Expose configuration files via web server

### 2. Performance

**Recommended Settings:**

```toml
[server]
compression = true          # Enable compression
timeout_seconds = 30        # Reasonable timeout

[logging]
level = "warn"              # Reduce log verbosity in production
json_format = true          # Structured logs for parsing
```

### 3. Monitoring

**For Production:**
```toml
[logging]
level = "info"              # Balance between verbosity and usefulness
json_format = true          # Enable for log aggregation

[metrics]
enabled = true              # Always enable metrics
path = "/metrics"           # Standard Prometheus path
```

### 4. Development

**For Local Development:**
```toml
[server]
host = "127.0.0.1"          # Only local access
port = 3000                 # Non-standard port

[logging]
level = "debug"             # Verbose logging
json_format = false         # Human-readable logs
show_target = true          # Show module paths
```

### 5. High Availability

**For HA Deployments:**
```toml
[server]
host = "0.0.0.0"            # Listen on all interfaces
port = 8080                 # Standard port
compression = true          # Reduce bandwidth
timeout_seconds = 15        # Fail fast
```

---

## Configuration Validation

Crabrace validates configuration on startup. Invalid configurations will cause the server to fail with a descriptive error message.

### Common Validation Errors

**Invalid port:**
```
Error: Server port cannot be 0
```

**Invalid log level:**
```
Error: Invalid log level 'invalid'. Valid levels: trace, debug, info, warn, error
```

**Invalid timeout:**
```
Error: Server timeout cannot be 0
```

### Testing Configuration

Test your configuration before deploying:

```bash
# Dry-run (validates config and exits)
./crabrace --check-config

# Or run and check logs
./crabrace 2>&1 | head -20
```

---

## Troubleshooting

### Configuration Not Loading

1. **Check environment variable names** - Must use `CRABRACE_` prefix and `__` separator
2. **Verify config file path** - Use `CRABRACE_CONFIG` to specify custom location
3. **Check file permissions** - Config file must be readable
4. **Validate TOML syntax** - Use `toml-cli` or online validator

### Environment Variables Not Working

```bash
# Print all environment variables
env | grep CRABRACE

# Test with explicit values
CRABRACE_LOGGING__LEVEL=debug ./crabrace
```

### Port Already in Use

```bash
# Change port via environment variable
CRABRACE_SERVER__PORT=9000 ./crabrace

# Or in config file
[server]
port = 9000
```

### Log Level Not Changing

Environment variables override config files:

```bash
# Remove conflicting environment variable
unset CRABRACE_LOGGING__LEVEL

# Or use environment variable
export CRABRACE_LOGGING__LEVEL=debug
```

---

## Migration Guide

### From Command-Line Arguments (if migrating)

If you previously used:
```bash
./crabrace --port 8080 --log-level debug
```

Now use:
```bash
export CRABRACE_SERVER__PORT=8080
export CRABRACE_LOGGING__LEVEL=debug
./crabrace
```

### From Old Environment Variables

If you used `PORT` and `RUST_LOG`:
```bash
# Old
PORT=8080 RUST_LOG=debug ./crabrace

# New
CRABRACE_SERVER__PORT=8080 CRABRACE_LOGGING__LEVEL=debug ./crabrace
```

---

## Configuration Schema

Complete configuration schema in TOML format:

```toml
[server]
host = "string"                 # IP address or hostname
port = 1-65535                  # Port number
compression = true|false        # Boolean
timeout_seconds = 1-3600        # Positive integer

[logging]
level = "trace|debug|info|warn|error"
json_format = true|false
show_target = true|false

[metrics]
enabled = true|false
path = "string"                 # URL path
```

---

## Support

For configuration issues:
- Check logs: `./crabrace 2>&1 | less`
- Validate config: See [Configuration Validation](#configuration-validation)
- GitHub Issues: https://github.com/jyjeanne/crabrace/issues

---

**Last Updated:** 2025-10-27
