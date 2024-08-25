#!/bin/bash

set -euo pipefail

export RUSTFLAGS="-D warnings"
export RUSTFMT_CI=1

# Print vetion
rustc -Vv
cargo -V

# Build and test main crate
cargo build --locked --all-features
