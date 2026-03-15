#!/bin/bash

# This script builds the bare metal Rust project

set -e

# Define the project directory
PROJECT_DIR=$(pwd)

# Build the project
cargo build --release

echo "Build completed successfully."