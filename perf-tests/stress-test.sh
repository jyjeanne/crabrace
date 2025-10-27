#!/bin/bash
# Stress testing script - gradually increases load
# Tests system behavior under increasing pressure

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
HOST="${CRABRACE_HOST:-http://localhost:8080}"
TOOL="${TOOL:-bombardier}" # wrk or bombardier

echo -e "${GREEN}=== Crabrace Stress Testing ===${NC}"
echo "Host: $HOST"
echo "Tool: $TOOL"
echo ""

# Check if tool is installed
if ! command -v $TOOL &> /dev/null; then
    echo -e "${RED}Error: $TOOL is not installed${NC}"
    exit 1
fi

# Check if server is running
echo -e "${YELLOW}Checking if server is running...${NC}"
if ! curl -s "$HOST/health" > /dev/null; then
    echo -e "${RED}Error: Server is not running at $HOST${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Server is running${NC}"
echo ""

# Stress test with increasing load
CONNECTIONS=(10 50 100 200 500 1000)

for CONN in "${CONNECTIONS[@]}"; do
    echo -e "${BLUE}=== Testing with $CONN connections ===${NC}"

    if [ "$TOOL" == "bombardier" ]; then
        bombardier -c $CONN -d 10s \
            --print=result \
            "$HOST/providers" || echo -e "${RED}Failed at $CONN connections${NC}"
    elif [ "$TOOL" == "wrk" ]; then
        wrk -t4 -c$CONN -d10s \
            --latency \
            "$HOST/providers" || echo -e "${RED}Failed at $CONN connections${NC}"
    fi

    echo ""
    sleep 2  # Cool-down period
done

echo -e "${GREEN}=== Stress testing complete ===${NC}"
echo ""
echo "Summary:"
echo "- Tested connection levels: ${CONNECTIONS[*]}"
echo "- Check for performance degradation at higher connection counts"
echo "- Look for error rates and timeout increases"
