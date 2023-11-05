#!/bin/bash

if [ -z "$1" ]; then
    echo "Error: No version number provided."
    echo "Usage: $0 0.1.0"
    exit 1
fi

VERSION=$1

cd "$(dirname "$0")"
cargo clean

# Build and package for x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
tar -czvf target/x86_64-unknown-linux-gnu/mycokv-v${VERSION}.linux-amd64.tar.gz target/x86_64-unknown-linux-gnu/release/mycokv

# Build and package for aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
tar -czvf target/aarch64-unknown-linux-gnu/mycokv-v${VERSION}.linux-arm64.tar.gz target/aarch64-unknown-linux-gnu/release/mycokv

# Build and package for x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
tar -czvf target/x86_64-apple-darwin/mycokv-v${VERSION}.darwin-amd64.tar.gz target/x86_64-apple-darwin/release/mycokv

# Build and package for aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
tar -czvf target/aarch64-apple-darwin/mycokv-v${VERSION}.darwin-arm64.tar.gz target/aarch64-apple-darwin/release/mycokv

# Build and package for x86_64-pc-windows-gnu
# cargo build --release --target x86_64-pc-windows-gnu
# zip target/x86_64-pc-windows-gnu/mycokv-v${VERSION}.windows-amd64.zip target/x86_64-pc-windows-gnu/release/mycokv.exe
