# Kubernetes Deployment Guide

Complete guide for deploying Crabrace on Kubernetes.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Deployment Methods](#deployment-methods)
- [Configuration](#configuration)
- [Monitoring](#monitoring)
- [Scaling](#scaling)
- [Security](#security)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required

- Kubernetes cluster (v1.24+)
- `kubectl` configured
- Container registry access

### Optional

- Helm 3.x (for Helm deployment)
- Prometheus Operator (for ServiceMonitor)
- cert-manager (for TLS certificates)
- Nginx Ingress Controller (for Ingress)

---

## Quick Start

### Using kubectl

```bash
# Apply all manifests
kubectl apply -f k8s/

# Check deployment status
kubectl get pods -n crabrace

# Get service information
kubectl get svc -n crabrace

# Check logs
kubectl logs -n crabrace -l app=crabrace

# Port forward for local access
kubectl port-forward -n crabrace svc/crabrace 8080:80
```

### Using Kustomize

```bash
# Build and apply with kustomize
kubectl apply -k k8s/

# Or use kubectl's built-in kustomize
kubectl kustomize k8s/ | kubectl apply -f -
```

### Using Helm

```bash
# Install the chart
helm install crabrace ./helm/crabrace -n crabrace --create-namespace

# Upgrade
helm upgrade crabrace ./helm/crabrace -n crabrace

# Uninstall
helm uninstall crabrace -n crabrace
```

---

## Deployment Methods

### Method 1: Plain Manifests

**Pros:** Simple, direct, no dependencies
**Cons:** Manual configuration management

```bash
# 1. Create namespace
kubectl apply -f k8s/namespace.yaml

# 2. Create configuration
kubectl apply -f k8s/configmap.yaml

# 3. Deploy application
kubectl apply -f k8s/deployment.yaml

# 4. Create service
kubectl apply -f k8s/service.yaml

# 5. Create ingress (optional)
kubectl apply -f k8s/ingress.yaml

# 6. Enable autoscaling (optional)
kubectl apply -f k8s/hpa.yaml

# 7. Add pod disruption budget (optional)
kubectl apply -f k8s/poddisruptionbudget.yaml
```

### Method 2: Kustomize

**Pros:** Configuration management, overlays
**Cons:** Learning curve

```bash
# Apply all resources
kubectl apply -k k8s/

# With custom overlays
kubectl apply -k k8s/overlays/production/
```

**Create overlays:**
```bash
# k8s/overlays/production/kustomization.yaml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization

bases:
  - ../../

patchesStrategicMerge:
  - replica-count.yaml
  - resources.yaml

configMapGenerator:
  - name: crabrace-config
    behavior: merge
    literals:
      - CRABRACE_LOGGING__LEVEL=warn
      - CRABRACE_SECURITY__CORS__ALLOWED_ORIGINS=["https://yourdomain.com"]
```

### Method 3: Helm

**Pros:** Templating, versioning, easy upgrades
**Cons:** Additional tool requirement

```bash
# Install with custom values
helm install crabrace ./helm/crabrace \
  --namespace crabrace \
  --create-namespace \
  --set image.tag=v0.1.0 \
  --set ingress.hosts[0].host=api.yourdomain.com

# Or with values file
helm install crabrace ./helm/crabrace \
  --namespace crabrace \
  --create-namespace \
  --values custom-values.yaml
```

**Custom values file:**
```yaml
# custom-values.yaml
replicaCount: 5

image:
  repository: ghcr.io/yourusername/crabrace
  tag: "v0.1.0"

ingress:
  hosts:
    - host: api.yourdomain.com
      paths:
        - path: /
          pathType: Prefix

config:
  logging:
    level: "warn"
    jsonFormat: "true"
```

---

## Configuration

### ConfigMap

The ConfigMap contains all environment variables for configuration:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: crabrace-config
  namespace: crabrace
data:
  # Example: Change log level
  CRABRACE_LOGGING__LEVEL: "debug"

  # Example: Configure CORS
  CRABRACE_SECURITY__CORS__ALLOWED_ORIGINS: '["https://yourdomain.com"]'

  # Example: Increase rate limit
  CRABRACE_SECURITY__RATE_LIMIT__REQUESTS_PER_PERIOD: "5000"
```

### Updating Configuration

```bash
# Edit ConfigMap
kubectl edit configmap crabrace-config -n crabrace

# Or apply updated file
kubectl apply -f k8s/configmap.yaml

# Restart pods to pick up changes
kubectl rollout restart deployment/crabrace -n crabrace
```

### Secrets (if needed)

For sensitive data:

```bash
# Create secret
kubectl create secret generic crabrace-secrets \
  --from-literal=api-key=your-secret-key \
  -n crabrace

# Reference in deployment
envFrom:
- secretRef:
    name: crabrace-secrets
```

---

## Monitoring

### Prometheus Integration

#### With Prometheus Operator

```bash
# Apply ServiceMonitor
kubectl apply -f k8s/servicemonitor.yaml

# Verify
kubectl get servicemonitor -n crabrace
```

#### With Standard Prometheus

Add to Prometheus configuration:

```yaml
scrape_configs:
  - job_name: 'crabrace'
    kubernetes_sd_configs:
      - role: pod
        namespaces:
          names:
            - crabrace
    relabel_configs:
      - source_labels: [__meta_kubernetes_pod_annotation_prometheus_io_scrape]
        action: keep
        regex: true
      - source_labels: [__meta_kubernetes_pod_annotation_prometheus_io_path]
        action: replace
        target_label: __metrics_path__
        regex: (.+)
      - source_labels: [__address__, __meta_kubernetes_pod_annotation_prometheus_io_port]
        action: replace
        regex: ([^:]+)(?::\d+)?;(\d+)
        replacement: $1:$2
        target_label: __address__
```

### Viewing Metrics

```bash
# Port forward to Prometheus
kubectl port-forward -n monitoring svc/prometheus 9090:9090

# Access Prometheus UI
open http://localhost:9090

# Query examples:
# - rate(crabrace_providers_requests_total[5m])
# - up{job="crabrace"}
```

### Grafana Dashboard

Import dashboard using metrics:
- `crabrace_providers_requests_total`
- `http_request_duration_seconds`
- CPU/Memory from Kubernetes metrics

---

## Scaling

### Manual Scaling

```bash
# Scale to 5 replicas
kubectl scale deployment/crabrace --replicas=5 -n crabrace

# Verify
kubectl get pods -n crabrace
```

### Horizontal Pod Autoscaler (HPA)

```bash
# Apply HPA
kubectl apply -f k8s/hpa.yaml

# Check HPA status
kubectl get hpa -n crabrace

# Describe HPA
kubectl describe hpa crabrace -n crabrace
```

**HPA Configuration:**
- Min replicas: 3
- Max replicas: 10
- Target CPU: 70%
- Target Memory: 80%

### Vertical Pod Autoscaler (VPA)

```yaml
apiVersion: autoscaling.k8s.io/v1
kind: VerticalPodAutoscaler
metadata:
  name: crabrace
  namespace: crabrace
spec:
  targetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: crabrace
  updatePolicy:
    updateMode: "Auto"
```

---

## Security

### Pod Security

**Security Context:**
- Non-root user (UID 1000)
- Read-only root filesystem
- No privilege escalation
- Capabilities dropped
- Seccomp profile: RuntimeDefault

### Network Policies

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: crabrace
  namespace: crabrace
spec:
  podSelector:
    matchLabels:
      app: crabrace
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
    ports:
    - protocol: TCP
      port: 8080
  egress:
  - to:
    - namespaceSelector: {}
    ports:
    - protocol: TCP
      port: 53  # DNS
```

### RBAC

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: crabrace
  namespace: crabrace
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: crabrace
  namespace: crabrace
rules:
- apiGroups: [""]
  resources: ["configmaps"]
  verbs: ["get", "list", "watch"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: crabrace
  namespace: crabrace
subjects:
- kind: ServiceAccount
  name: crabrace
  namespace: crabrace
roleRef:
  kind: Role
  name: crabrace
  apiGroup: rbac.authorization.k8s.io
```

### TLS/HTTPS

#### Using cert-manager

```bash
# Install cert-manager
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.13.0/cert-manager.yaml

# Create ClusterIssuer
cat <<EOF | kubectl apply -f -
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt-prod
spec:
  acme:
    server: https://acme-v02.api.letsencrypt.org/directory
    email: your-email@example.com
    privateKeySecretRef:
      name: letsencrypt-prod
    solvers:
    - http01:
        ingress:
          class: nginx
EOF
```

The Ingress manifest will automatically request a certificate.

---

## High Availability

### Multi-Zone Deployment

```yaml
# Add topology spread constraints
topologySpreadConstraints:
- maxSkew: 1
  topologyKey: topology.kubernetes.io/zone
  whenUnsatisfiable: DoNotSchedule
  labelSelector:
    matchLabels:
      app: crabrace
```

### Pod Disruption Budget

Ensures minimum availability during voluntary disruptions:

```bash
kubectl apply -f k8s/poddisruptionbudget.yaml

# Verify
kubectl get pdb -n crabrace
```

**Configuration:**
- Min available: 2 pods
- Prevents simultaneous pod termination

---

## Troubleshooting

### Pod Not Starting

```bash
# Check pod status
kubectl get pods -n crabrace

# Describe pod
kubectl describe pod <pod-name> -n crabrace

# Check events
kubectl get events -n crabrace --sort-by='.lastTimestamp'

# Check logs
kubectl logs <pod-name> -n crabrace
```

**Common Issues:**
- Image pull errors: Check image name and registry access
- CrashLoopBackOff: Check logs for application errors
- Pending: Check resource availability and node selectors

### Service Not Accessible

```bash
# Check service
kubectl get svc -n crabrace
kubectl describe svc crabrace -n crabrace

# Check endpoints
kubectl get endpoints crabrace -n crabrace

# Test from within cluster
kubectl run -it --rm debug --image=busybox --restart=Never -- sh
wget -O- http://crabrace.crabrace.svc.cluster.local/health
```

### Ingress Issues

```bash
# Check ingress
kubectl get ingress -n crabrace
kubectl describe ingress crabrace -n crabrace

# Check ingress controller logs
kubectl logs -n ingress-nginx -l app.kubernetes.io/name=ingress-nginx

# Test DNS resolution
nslookup api.yourdomain.com
```

### High Memory/CPU Usage

```bash
# Check resource usage
kubectl top pods -n crabrace

# Check HPA status
kubectl get hpa -n crabrace

# Review metrics
kubectl describe hpa crabrace -n crabrace

# Adjust resources
kubectl set resources deployment/crabrace \
  --limits=cpu=1000m,memory=1Gi \
  --requests=cpu=200m,memory=256Mi \
  -n crabrace
```

### Configuration Not Applied

```bash
# Verify ConfigMap
kubectl get configmap crabrace-config -n crabrace -o yaml

# Restart deployment
kubectl rollout restart deployment/crabrace -n crabrace

# Check pod environment
kubectl exec <pod-name> -n crabrace -- env | grep CRABRACE
```

---

## Maintenance

### Updates

```bash
# Update image
kubectl set image deployment/crabrace crabrace=crabrace:v0.2.0 -n crabrace

# Check rollout status
kubectl rollout status deployment/crabrace -n crabrace

# Rollback if needed
kubectl rollout undo deployment/crabrace -n crabrace
```

### Backup

```bash
# Backup all resources
kubectl get all,cm,secret,ing,pdb,hpa -n crabrace -o yaml > crabrace-backup.yaml

# Restore
kubectl apply -f crabrace-backup.yaml
```

### Cleanup

```bash
# Delete all resources
kubectl delete -f k8s/

# Or delete namespace (removes everything)
kubectl delete namespace crabrace
```

---

## Production Checklist

- [ ] Image stored in reliable container registry
- [ ] Resource limits configured appropriately
- [ ] HPA enabled and tested
- [ ] Pod Disruption Budget applied
- [ ] Monitoring and alerting configured
- [ ] TLS certificates configured (cert-manager)
- [ ] Network policies applied
- [ ] RBAC configured with least privilege
- [ ] ConfigMap reviewed for production settings
- [ ] Backup strategy in place
- [ ] Rolling update strategy tested
- [ ] Ingress configured with rate limiting
- [ ] Logs aggregated to central location
- [ ] Health checks tuned appropriately

---

## Examples

### Development Environment

```bash
# Minimal deployment
kubectl create namespace crabrace-dev
kubectl apply -f k8s/configmap.yaml -n crabrace-dev
kubectl apply -f k8s/deployment.yaml -n crabrace-dev
kubectl apply -f k8s/service.yaml -n crabrace-dev

# Port forward
kubectl port-forward -n crabrace-dev svc/crabrace 8080:80
```

### Production Environment

```bash
# Full deployment with Helm
helm install crabrace ./helm/crabrace \
  --namespace crabrace \
  --create-namespace \
  --values production-values.yaml \
  --wait

# Verify
helm status crabrace -n crabrace
kubectl get all -n crabrace
```

---

## Support

For issues or questions:
- GitHub Issues: https://github.com/jyjeanne/crabrace/issues
- Documentation: See README.md

---

**Last Updated:** 2025-10-27
