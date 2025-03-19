.PHONY: all static test

all: static test bench

static:
	cargo fmt -- --check
	cargo clippy --all-targets -- --deny warnings

test:
	cargo test

bench:
	cargo bench
