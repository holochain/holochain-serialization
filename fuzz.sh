#! /usr/bin/env bash
set -eu
cargo test

cargo test-fuzz "$FUZZ_TARGET"
