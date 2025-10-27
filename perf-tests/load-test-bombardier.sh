#!/bin/bash
# Load testing script using bombardier (https://github.com/codesenberg/bombardier)
# Install: go install github.com/codesenberg/bombardier@latest

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
HOST="${CRABRACE_HOST:-http://localhost:8080}"
DURATION="${DURATION:-30s}"
CONNECTIONS="${CONNECTIONS:-100}"
RATE="${RATE:-0}" # 0 = unlimited

echo -e "${GREEN}=== Crabrace Load Testing with bombardier ===${NC}"
echo "Host: $HOST"
echo "Duration: $DURATION"
echo "Connections: $CONNECTIONS"
echo "Rate: $RATE (0 = unlimited)"
echo ""

# Check if bombardier is installed
if ! command -v bombardier &> /dev/null; then
    echo -e "${RED}Error: bombardier is not installed${NC}"
    echo "Install with: go install github.com/codesenberg/bombardier@latest"
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
bombardier -c $CONNECTIONS -d $DURATION \
    --latencies \
    --print=result \
    "$HOST/health"
echo ""

# Test 2: Providers endpoint
echo -e "${YELLOW}Test 2: Providers endpoint (main workload)${NC}"
bombardier -c $CONNECTIONS -d $DURATION \
    --latencies \
    --print=result \
    "$HOST/providers"
echo ""

# Test 3: Metrics endpoint
echo -e "${YELLOW}Test 3: Metrics endpoint${NC}"
bombardier -c $CONNECTIONS -d $DURATION \
    --latencies \
    --print=result \
    "$HOST/metrics"
echo ""

# Test 4: Rate limiting test (1000 req/s)
echo -e "${YELLOW}Test 4: Rate limiting (1000 req/s)${NC}"
bombardier -c $CONNECTIONS -d 30s \
    --rate=1000 \
    --latencies \
    --print=result \
    "$HOST/providers"
echo ""

# Test 5: High throughput test
echo -e "${YELLOW}Test 5: Maximum throughput test${NC}"
bombardier -c 500 -d 30s \
    --fasthttp \
    --latencies \
    --print=result \
    "$HOST/providers"
echo ""

echo -e "${GREEN}=== Load testing complete ===${NC}"
