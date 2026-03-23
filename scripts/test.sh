#!/bin/bash
# Script to run all project tests

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Running Backend Tests...${NC}"
cargo test --workspace

echo -e "${YELLOW}To generate a coverage report, run: ./scripts/coverage.sh${NC}"
