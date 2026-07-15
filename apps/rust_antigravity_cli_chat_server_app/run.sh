#!/bin/bash
# Exit on error
set -e

# Get script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

echo "=============================================="
echo "          Building React Frontend            "
echo "=============================================="
cd frontend
if [ -d "node_modules" ]; then
    node ./node_modules/vite/bin/vite.js build
else
    echo "Dependencies not found in frontend. Installing first..."
    npm install
    node ./node_modules/vite/bin/vite.js build
fi
cd ..

echo "=============================================="
echo "          Starting Rust Backend              "
echo "=============================================="
cd backend
cargo run
