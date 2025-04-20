#!/usr/bin/env bash

# This script binds the TypeScript files in the src-tauri directory to the src/bindings directory.
# # Usage: ./scripts/bind_ts.sh

# Check if the required tools are installed

if ! command -v cargo &>/dev/null; then
    echo "❌ Cargo is not installed. Please install Cargo to run this script."
    exit 1
fi

cd src-tauri || exit

echo "🔧 Binding TypeScript files in src-tauri to src/bindings"

cargo test --workspace --features ts
SUCCESS=$?

if [ $SUCCESS -ne 0 ]; then
    echo "❌ Failed to bind TypeScript files. Please check the output for errors."
    exit 1
fi

echo "✅ Successfully bound TypeScript files to src/bindings"
