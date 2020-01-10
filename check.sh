#!/bin/sh
set -e -u

cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
