#!/bin/bash
# Load testing script using Apache Bench (ab)
# Usually pre-installed on most systems, or: apt-get install apache2-utils

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
HOST="${CRABRACE_HOST:-http://localhost:8080}"
REQUESTS="${REQUESTS:-10000}"
CONCURRENCY="${CONCURRENCY:-100}"

echo -e "${GREEN}=== Crabrace Load Testing with Apache Bench ===${NC}"
echo "Host: $HOST"
echo "Total Requests: $REQUESTS"
echo "Concurrency: $CONCURRENCY"
echo ""

# Check if ab is installed
if ! command -v ab &> /dev/null; then
    echo -e "${RED}Error: Apache Bench (ab) is not installed${NC}"
    echo "Install with: apt-get install apache2-utils (Ubuntu) or brew install httpd (macOS)"
    exit 1
fi

# Check if server is running
echo -e "${YELLOW}Checking if server is running...${NC}"
if ! curl -s "$HOST/health" > /dev/null; then
    echo -e "${RED}Error: Server is not running at $HOST${NC}"
    echo "Start the server with: cargo run --release"
    exit 1
fi
echo -e "${GREEN}✓ Server is running${NC}"
echo ""

# Test 1: Health endpoint
echo -e "${YELLOW}Test 1: Health endpoint${NC}"
ab -n $REQUESTS -c $CONCURRENCY \
    -g health-results.tsv \
    "$HOST/health"
echo ""

# Test 2: Providers endpoint
echo -e "${YELLOW}Test 2: Providers endpoint (main workload)${NC}"
ab -n $REQUESTS -c $CONCURRENCY \
    -g providers-results.tsv \
    "$HOST/providers"
echo ""

# Test 3: Keep-alive test
echo -e "${YELLOW}Test 3: With keep-alive${NC}"
ab -n $REQUESTS -c $CONCURRENCY \
    -k \
    -g keepalive-results.tsv \
    "$HOST/providers"
echo ""

# Test 4: Low concurrency, many requests
echo -e "${YELLOW}Test 4: Low concurrency (10), many requests (50000)${NC}"
ab -n 50000 -c 10 \
    "$HOST/health"
echo ""

echo -e "${GREEN}=== Load testing complete ===${NC}"
echo "Results saved to: health-results.tsv, providers-results.tsv, keepalive-results.tsv"
