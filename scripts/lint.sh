#!/bin/bash
# Run all linting checks locally (same as CI)
#
# Prerequisites:
# - Rust toolchain installed (rustup install stable)
# - Clippy component installed (rustup component add clippy)
# - Rustfmt component installed (rustup component add rustfmt)

set -e

# Source cargo environment if available
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# Add cargo bin to PATH if not already there
export PATH="$HOME/.cargo/bin:$PATH"

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo not found. Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "Running Rust linting checks..."
echo ""

echo "1. Running clippy with default features..."
cargo clippy -- -D warnings

echo ""
echo "2. Running clippy with parallel feature..."
cargo clippy --features parallel -- -D warnings

echo ""
echo "3. Running clippy with simd feature..."
cargo clippy --features simd -- -D warnings

echo ""
echo "4. Running clippy with all features..."
cargo clippy --all-features -- -D warnings

echo ""
echo "5. Checking formatting..."
if ! cargo fmt -- --check; then
    echo ""
    echo "❌ Formatting check failed!"
    echo "Run 'cargo fmt' to automatically fix formatting issues."
    exit 1
fi

echo ""
echo "✓ All linting checks passed!"

