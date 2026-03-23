#!/bin/bash

# Exit on error
set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting test coverage report generation...${NC}"

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null
then
    echo -e "${YELLOW}cargo-llvm-cov is not installed. Installing...${NC}"
    cargo install cargo-llvm-cov
fi

# Create coverage directory if it doesn't exist
mkdir -p target/coverage

echo -e "${GREEN}Running tests and generating coverage data...${NC}"

# Run tests and generate HTML report
cargo llvm-cov --all-features --workspace --html --output-dir target/coverage

echo -e "${GREEN}Coverage report generated successfully!${NC}"
echo -e "${YELLOW}You can find the report at: target/coverage/index.html${NC}"
