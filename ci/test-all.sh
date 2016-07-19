#!/bin/bash

# This file needs to be run from the git root directory!

# Exit script on the first error
set -o errexit -o nounset

export RUSTFLAGS="--deny warnings"

# Build the main crate which depends on all others
echo ""
echo "=== Building Plantex ==============="
cargo build

for crate in base client server plantex; do
    echo ""
    echo "=== Testing $crate... =============="
    cargo test -p $crate
done
