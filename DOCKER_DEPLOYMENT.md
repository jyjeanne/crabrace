# Docker Deployment Guide

Complete guide for deploying Crabrace using Docker.

## Table of Contents

- [Quick Start](#quick-start)
- [Docker Compose](#docker-compose)
- [Production Deployment](#production-deployment)
- [Monitoring Stack](#monitoring-stack)
- [Troubleshooting](#troubleshooting)

---

## Quick Start

### Build and Run

```bash
# Build the Docker image
docker build -t crabrace:latest .

# Run the container
docker run -d \
  --name crabrace \
  -p 8080:8080 \
  -e RUST_LOG=info \
  --restart unless-stopped \
  crabrace:latest

# Verify it's running
curl http://localhost:8080/health
```

### Check Logs

```bash
# Follow logs
docker logs -f crabrace

# Last 100 lines
docker logs --tail 100 crabrace
```

### Stop and Remove

```bash
# Stop container
docker stop crabrace

# Remove container
docker rm crabrace

# Remove image
docker rmi crabrace:latest
```

---

## Docker Compose

### Basic Deployment

Create or use the included `docker-compose.yml`:

```bash
# Start services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### With Monitoring Stack

Deploy Crabrace with Prometheus and Grafana:

```bash
# Start all services including monitoring
docker-compose --profile monitoring up -d

# Access services:
# - Crabrace: http://localhost:8080
# - Prometheus: http://localhost:9090
# - Grafana: http://localhost:3000 (admin/admin)
```

### Configuration

Environment variables in `docker-compose.yml`:

```yaml
environment:
  - RUST_LOG=info          # Log level
  - PORT=8080              # Server port
```

---

## Production Deployment

### Best Practices

1. **Use specific version tags**
   ```bash
   docker build -t crabrace:v0.1.0 .
   docker tag crabrace:v0.1.0 crabrace:latest
   ```

2. **Set resource limits**
   ```bash
   docker run -d \
     --name crabrace \
     -p 8080:8080 \
     --memory="512m" \
     --cpus="1.0" \
     --restart unless-stopped \
     crabrace:latest
   ```

3. **Use health checks**
   ```bash
   docker run -d \
     --name crabrace \
     -p 8080:8080 \
     --health-cmd="curl -f http://localhost:8080/health || exit 1" \
     --health-interval=30s \
     --health-timeout=3s \
     --health-retries=3 \
     crabrace:latest
   ```

### Docker Compose Production

```yaml
version: '3.8'

services:
  crabrace:
    image: crabrace:v0.1.0
    container_name: crabrace-prod
    restart: always
    ports:
      - "8080:8080"
    environment:
      - RUST_LOG=info
      - PORT=8080
    deploy:
      resources:
        limits:
          cpus: '1.0'
          memory: 512M
        reservations:
          cpus: '0.5'
          memory: 256M
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 3s
      retries: 3
      start_period: 5s
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

### Behind a Reverse Proxy (Nginx)

```nginx
server {
    listen 80;
    server_name api.example.com;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Health check endpoint
    location /health {
        proxy_pass http://localhost:8080/health;
        access_log off;
    }
}
```

---

## Monitoring Stack

### Prometheus Configuration

The included `prometheus.yml` scrapes metrics from Crabrace:

```yaml
scrape_configs:
  - job_name: 'crabrace'
    static_configs:
      - targets: ['crabrace:8080']
    metrics_path: '/metrics'
    scrape_interval: 10s
```

### Grafana Dashboard

1. Access Grafana at http://localhost:3000 (admin/admin)
2. Add Prometheus data source:
   - URL: http://prometheus:9090
3. Import or create dashboard with queries:
   ```promql
   # Request rate
   rate(crabrace_providers_requests_total[5m])

   # Request duration
   histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))
   ```

### Custom Alerts

Add to `prometheus.yml`:

```yaml
rule_files:
  - 'alerts.yml'

alerting:
  alertmanagers:
    - static_configs:
        - targets: ['alertmanager:9093']
```

---

## Troubleshooting

### Container Won't Start

```bash
# Check logs
docker logs crabrace

# Inspect container
docker inspect crabrace

# Check if port is in use
netstat -tlnp | grep 8080
```

### High Memory Usage

```bash
# Check container stats
docker stats crabrace

# Set memory limit
docker update --memory="512m" crabrace
```

### Networking Issues

```bash
# Check container network
docker network inspect bridge

# Test from another container
docker run --rm curlimages/curl:latest curl http://crabrace:8080/health
```

### Image Size Too Large

```bash
# Check image size
docker images crabrace

# Use multi-stage build (already implemented)
# Current size should be ~80MB

# Remove unused layers
docker image prune -a
```

### Debugging

```bash
# Run with shell access
docker run -it --rm \
  --entrypoint /bin/bash \
  crabrace:latest

# Execute command in running container
docker exec -it crabrace /bin/bash

# Check process
docker exec crabrace ps aux
```

### Logs Not Showing

```bash
# Ensure RUST_LOG is set
docker run -d \
  --name crabrace \
  -p 8080:8080 \
  -e RUST_LOG=debug \
  crabrace:latest

# Check stdout/stderr
docker logs crabrace 2>&1
```

---

## Multi-Platform Builds

### Build for Multiple Architectures

```bash
# Setup buildx
docker buildx create --name multiarch --use

# Build for multiple platforms
docker buildx build \
  --platform linux/amd64,linux/arm64,linux/arm/v7 \
  --tag crabrace:latest \
  --push \
  .
```

### Platform-Specific Images

```bash
# For AMD64 (x86_64)
docker build --platform linux/amd64 -t crabrace:amd64 .

# For ARM64 (aarch64)
docker build --platform linux/arm64 -t crabrace:arm64 .

# For ARM v7 (32-bit ARM)
docker build --platform linux/arm/v7 -t crabrace:armv7 .
```

---

## Container Registry

### Push to Docker Hub

```bash
# Tag image
docker tag crabrace:latest yourusername/crabrace:latest
docker tag crabrace:latest yourusername/crabrace:v0.1.0

# Login
docker login

# Push
docker push yourusername/crabrace:latest
docker push yourusername/crabrace:v0.1.0
```

### Push to GitHub Container Registry

```bash
# Tag image
docker tag crabrace:latest ghcr.io/yourusername/crabrace:latest

# Login
echo $GITHUB_TOKEN | docker login ghcr.io -u yourusername --password-stdin

# Push
docker push ghcr.io/yourusername/crabrace:latest
```

---

## Performance Tuning

### Optimize Build

```bash
# Use build cache
docker build -t crabrace:latest .

# Parallel builds
docker build --build-arg CARGO_BUILD_JOBS=4 -t crabrace:latest .
```

### Runtime Optimization

```bash
docker run -d \
  --name crabrace \
  -p 8080:8080 \
  --ulimit nofile=65536:65536 \
  --sysctl net.ipv4.tcp_keepalive_time=600 \
  crabrace:latest
```

---

## Security

### Scan for Vulnerabilities

```bash
# Using Docker Scout
docker scout cves crabrace:latest

# Using Trivy
trivy image crabrace:latest
```

### Security Best Practices

1. ✅ Runs as non-root user (implemented)
2. ✅ Minimal base image (Debian slim)
3. ✅ No unnecessary dependencies
4. ✅ Read-only filesystem (can be added)
5. ✅ Health checks included

### Run with Additional Security

```bash
docker run -d \
  --name crabrace \
  -p 8080:8080 \
  --read-only \
  --security-opt=no-new-privileges:true \
  --cap-drop=ALL \
  crabrace:latest
```

---

## Support

For issues or questions:
- GitHub Issues: https://github.com/jyjeanne/crabrace/issues
- Documentation: See README.md

---

**Last Updated:** 2025-10-27
