# Security Guide

Comprehensive security features and best practices for Crabrace.

## Table of Contents

- [Security Features](#security-features)
- [CORS Configuration](#cors-configuration)
- [Rate Limiting](#rate-limiting)
- [Security Headers](#security-headers)
- [Best Practices](#best-practices)
- [Security Checklist](#security-checklist)

---

## Security Features

Crabrace includes multiple layers of security protection:

| Feature | Status | Description |
|---------|--------|-------------|
| **CORS** | ✅ Enabled | Cross-Origin Resource Sharing control |
| **Rate Limiting** | ✅ Enabled | Request throttling to prevent abuse |
| **Security Headers** | ✅ Enabled | HTTP security headers (HSTS, CSP, etc.) |
| **Non-Root Container** | ✅ Enabled | Docker containers run as non-root user |
| **TLS Support** | ⚠️ External | Via reverse proxy (Nginx, Traefik) |
| **Request Validation** | ✅ Built-in | Input validation and sanitization |

---

## CORS Configuration

### Overview

Cross-Origin Resource Sharing (CORS) controls which domains can access your API.

### Configuration

```toml
[security.cors]
# Enable CORS
enabled = true

# Allowed origins
allowed_origins = ["https://yourdomain.com", "https://app.yourdomain.com"]

# Allowed HTTP methods
allowed_methods = ["GET", "POST", "OPTIONS"]

# Allowed headers
allowed_headers = ["Content-Type", "Authorization"]

# Max age for preflight cache (seconds)
max_age_seconds = 3600
```

###Environment Variables

```bash
CRABRACE_SECURITY__CORS__ENABLED=true
CRABRACE_SECURITY__CORS__ALLOWED_ORIGINS='["https://yourdomain.com"]'
CRABRACE_SECURITY__CORS__ALLOWED_METHODS='["GET","POST","OPTIONS"]'
```

### Common Configurations

**Development (Allow All):**
```toml
[security.cors]
enabled = true
allowed_origins = ["*"]
```

**Production (Specific Domains):**
```toml
[security.cors]
enabled = true
allowed_origins = [
    "https://yourdomain.com",
    "https://app.yourdomain.com"
]
```

**Disabled:**
```toml
[security.cors]
enabled = false
```

---

## Rate Limiting

### Overview

Rate limiting prevents abuse by limiting the number of requests from a single IP address.

### Configuration

```toml
[security.rate_limit]
# Enable rate limiting
enabled = true

# Requests allowed per period
requests_per_period = 100

# Period in seconds
period_seconds = 60
```

### Environment Variables

```bash
CRABRACE_SECURITY__RATE_LIMIT__ENABLED=true
CRABRACE_SECURITY__RATE_LIMIT__REQUESTS_PER_PERIOD=100
CRABRACE_SECURITY__RATE_LIMIT__PERIOD_SECONDS=60
```

### Common Configurations

**Strict (Low Traffic):**
```toml
[security.rate_limit]
enabled = true
requests_per_period = 10
period_seconds = 60  # 10 requests per minute
```

**Standard (Moderate Traffic):**
```toml
[security.rate_limit]
enabled = true
requests_per_period = 100
period_seconds = 60  # 100 requests per minute
```

**Generous (High Traffic):**
```toml
[security.rate_limit]
enabled = true
requests_per_period = 1000
period_seconds = 60  # 1000 requests per minute
```

**Disabled:**
```toml
[security.rate_limit]
enabled = false
```

### Rate Limit Response

When rate limit is exceeded, clients receive:

```http
HTTP/1.1 429 Too Many Requests
Content-Type: text/plain

Too many requests. Please try again later.
```

### Best Practices

1. **Monitor rate limit hits** - Track 429 responses in metrics
2. **Adjust based on usage** - Increase limits for legitimate high-traffic scenarios
3. **Document limits** - Inform API consumers of rate limits
4. **Consider user tiers** - Different limits for different user types (future enhancement)

---

## Security Headers

### Overview

HTTP security headers protect against common web vulnerabilities.

### Configuration

```toml
[security.headers]
# Enable security headers
enabled = true

# HTTP Strict Transport Security
hsts = true

# X-Content-Type-Options: nosniff
content_type_options = true

# X-Frame-Options: DENY
frame_options = true

# X-XSS-Protection: 1; mode=block
xss_protection = true
```

### Environment Variables

```bash
CRABRACE_SECURITY__HEADERS__ENABLED=true
CRABRACE_SECURITY__HEADERS__HSTS=true
CRABRACE_SECURITY__HEADERS__CONTENT_TYPE_OPTIONS=true
CRABRACE_SECURITY__HEADERS__FRAME_OPTIONS=true
CRABRACE_SECURITY__HEADERS__XSS_PROTECTION=true
```

### Headers Explained

#### 1. HSTS (HTTP Strict Transport Security)

**Header:** `Strict-Transport-Security: max-age=31536000; includeSubDomains`

**Purpose:** Forces HTTPS for one year

**Protection:** Prevents downgrade attacks and cookie hijacking

**Note:** Only effective when using HTTPS (via reverse proxy)

#### 2. X-Content-Type-Options

**Header:** `X-Content-Type-Options: nosniff`

**Purpose:** Prevents MIME type sniffing

**Protection:** Stops browsers from interpreting files as different MIME types

#### 3. X-Frame-Options

**Header:** `X-Frame-Options: DENY`

**Purpose:** Prevents clickjacking attacks

**Protection:** Disables embedding in iframes

**Alternatives:** Use `SAMEORIGIN` to allow same-origin framing

#### 4. X-XSS-Protection

**Header:** `X-XSS-Protection: 1; mode=block`

**Purpose:** Enables XSS filtering in browsers

**Protection:** Blocks page if XSS attack is detected

**Note:** Mostly superseded by CSP, but provides additional protection

---

## Best Practices

### 1. Production Deployment

**DO:**
- ✅ Use specific CORS origins (not "*")
- ✅ Enable all security headers
- ✅ Set appropriate rate limits
- ✅ Use HTTPS (via reverse proxy)
- ✅ Run containers as non-root user
- ✅ Keep dependencies updated
- ✅ Monitor security logs

**DON'T:**
- ❌ Allow CORS from "*" in production
- ❌ Disable security features
- ❌ Run as root user
- ❌ Expose directly to internet without reverse proxy

### 2. Reverse Proxy Setup

Always use a reverse proxy (Nginx, Traefik, Caddy) for:

- **TLS termination**
- **Additional security headers**
- **DDoS protection**
- **Load balancing**

**Example Nginx Configuration:**
```nginx
server {
    listen 443 ssl http2;
    server_name api.yourdomain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    # Additional security headers
    add_header Content-Security-Policy "default-src 'self'" always;
    add_header X-Content-Security-Policy "default-src 'self'" always;
    add_header Referrer-Policy "no-referrer-when-downgrade" always;

    location / {
        proxy_pass http://crabrace:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 3. Network Security

**Firewall Rules:**
```bash
# Allow only necessary ports
ufw allow 443/tcp  # HTTPS
ufw allow 22/tcp   # SSH (from specific IPs only)
ufw enable
```

**Docker Network Isolation:**
```yaml
networks:
  frontend:
    # Public-facing network
  backend:
    # Internal services only
    internal: true
```

### 4. Monitoring and Alerts

**Monitor:**
- Rate limit hits (429 responses)
- CORS violations
- Unusual traffic patterns
- Failed authentication attempts (if added)

**Alert on:**
- Sustained rate limit hits
- Traffic spikes
- Security header violations
- Suspicious patterns

### 5. Regular Updates

```bash
# Update Docker base images
docker pull debian:bookworm-slim

# Rebuild with latest dependencies
docker build --no-cache -t crabrace:latest .

# Update Rust dependencies
cargo update
cargo audit
```

---

## Security Checklist

### Pre-Production

- [ ] Configure specific CORS origins (not "*")
- [ ] Set appropriate rate limits
- [ ] Enable all security headers
- [ ] Setup HTTPS via reverse proxy
- [ ] Configure firewall rules
- [ ] Setup monitoring and alerting
- [ ] Review and update dependencies
- [ ] Perform security audit
- [ ] Test rate limiting
- [ ] Test CORS configuration
- [ ] Document security configuration
- [ ] Setup backup and recovery

### Regular Maintenance

- [ ] Review security logs weekly
- [ ] Update dependencies monthly
- [ ] Review and adjust rate limits
- [ ] Check for security advisories
- [ ] Rotate secrets/credentials
- [ ] Audit access logs
- [ ] Test disaster recovery
- [ ] Review CORS allowlist

### Incident Response

1. **Detect** - Monitor for security events
2. **Contain** - Rate limit or block malicious IPs
3. **Investigate** - Review logs and metrics
4. **Remediate** - Apply fixes and updates
5. **Document** - Record incident details
6. **Review** - Update security measures

---

## Testing Security Features

### Test CORS

```bash
# Test allowed origin
curl -H "Origin: https://yourdomain.com" \
     -H "Access-Control-Request-Method: GET" \
     -X OPTIONS \
     http://localhost:8080/providers

# Should return CORS headers
```

### Test Rate Limiting

```bash
# Send multiple requests quickly
for i in {1..150}; do
    curl http://localhost:8080/providers
done

# After limit, should receive 429 response
```

### Test Security Headers

```bash
# Check security headers
curl -I http://localhost:8080/health

# Should include:
# Strict-Transport-Security: max-age=31536000; includeSubDomains
# X-Content-Type-Options: nosniff
# X-Frame-Options: DENY
# X-XSS-Protection: 1; mode=block
```

---

## Vulnerability Reporting

If you discover a security vulnerability in Crabrace:

1. **DO NOT** open a public issue
2. Email security concerns to: [security@yourdomain.com]
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will respond within 48 hours and work with you to address the issue.

---

## Security Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Mozilla Web Security Guidelines](https://infosec.mozilla.org/guidelines/web_security)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [Rust Security Advisories](https://rustsec.org/)

---

## Compliance

Crabrace security features help meet requirements for:

- **GDPR** - Data protection through security measures
- **SOC 2** - Security controls and monitoring
- **HIPAA** - Technical safeguards (with additional measures)
- **PCI DSS** - Security requirements for payment systems

**Note:** Full compliance requires additional organizational and technical controls beyond Crabrace.

---

**Last Updated:** 2025-10-27
