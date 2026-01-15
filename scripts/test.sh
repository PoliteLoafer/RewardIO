#!/bin/bash
# Script to run all project tests

echo "Running Backend Tests..."
cargo test

# If frontend tests are added later, they can be included here
# echo "Running Frontend Tests..."
# cd frontend && npm test
