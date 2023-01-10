#! /usr/bin/env bash
cargo test
cargo test --manifest-path crates/holochain_serialized_bytes/Cargo.toml

cargo test-fuzz
cargo test-fuzz --manifest-path crates/holochain_serialized_bytes/Cargo.toml