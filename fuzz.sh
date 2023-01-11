#! /usr/bin/env bash
cargo test

cargo test-fuzz "$1"
