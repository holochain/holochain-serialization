#! /usr/bin/env bash
cargo test

cargo test-fuzz "$FUZZ_TARGET"
