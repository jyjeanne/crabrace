#!/bin/bash
# Load testing script using wrk (https://github.com/wg/wrk)
# Install: brew install wrk (macOS) or apt-get install wrk (Ubuntu)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
HOST="${CRABRACE_HOST:-http://localhost:8080}"
DURATION="${DURATION:-30s}"
THREADS="${THREADS:-4}"
CONNECTIONS="${CONNECTIONS:-100}"

echo -e "${GREEN}=== Crabrace Load Testing with wrk ===${NC}"
echo "Host: $HOST"
echo "Duration: $DURATION"
echo "Threads: $THREADS"
echo "Connections: $CONNECTIONS"
echo ""

# Check if wrk is installed
if ! command -v wrk &> /dev/null; then
    echo -e "${RED}Error: wrk is not installed${NC}"
    echo "Install with: brew install wrk (macOS) or apt-get install wrk (Ubuntu)"
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
wrk -t$THREADS -c$CONNECTIONS -d$DURATION \
    --latency \
    "$HOST/health"
echo ""

# Test 2: Providers endpoint
echo -e "${YELLOW}Test 2: Providers endpoint (main workload)${NC}"
wrk -t$THREADS -c$CONNECTIONS -d$DURATION \
    --latency \
    "$HOST/providers"
echo ""

# Test 3: Metrics endpoint
echo -e "${YELLOW}Test 3: Metrics endpoint${NC}"
wrk -t$THREADS -c$CONNECTIONS -d$DURATION \
    --latency \
    "$HOST/metrics"
echo ""

# Test 4: High concurrency test
echo -e "${YELLOW}Test 4: High concurrency (500 connections)${NC}"
wrk -t$THREADS -c500 -d30s \
    --latency \
    "$HOST/providers"
echo ""

echo -e "${GREEN}=== Load testing complete ===${NC}"
