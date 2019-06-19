.PHONY: all test fmt clean tools tool_rust tool_fmt

RUSTFLAGS += -D warnings

SHELL = /usr/bin/env sh
RUST_VER_WANT = "rustc 1.33.0-nightly (19f8958f8 2019-01-23)"
RUST_TAG_WANT = "nightly-2019-01-24"
FMT_VER_WANT = "rustfmt 1.0.1-nightly (be13559 2018-12-10)"
CLP_VER_WANT = "clippy 0.0.212 (280069d 2019-01-22)"

all: test

test: tools
	RUSTFLAGS='$(RUSTFLAGS)' cargo fmt -- --check
	RUSTFLAGS='$(RUSTFLAGS)' cargo clippy -- \
		-A clippy::nursery -A clippy::style -A clippy::cargo \
		-A clippy::pedantic -A clippy::restriction \
		-D clippy::complexity -D clippy::perf -D clippy::correctness
	RUSTFLAGS='$(RUSTFLAGS)' RUST_BACKTRACE=1 cargo test

fmt: tools
	cargo fmt

clean:
	rm -rf target

tools: tool_rust tool_fmt tool_clippy

tool_rust:
	@if [ "$$(rustc --version 2>/dev/null || true)" != ${RUST_VER_WANT} ]; \
	then \
		echo "# Makefile # incorrect rust toolchain version"; \
		echo "# Makefile #   want:" ${RUST_VER_WANT}; \
		if rustup --version >/dev/null 2>&1; then \
			echo "# Makefile # found rustup, setting override"; \
			rustup override set ${RUST_TAG_WANT}; \
		else \
			echo "# Makefile # rustup not found, cannot install toolchain"; \
			exit 1; \
		fi \
	else \
		echo "# Makefile # rust toolchain ok:" ${RUST_VER_WANT}; \
	fi;

tool_fmt: tool_rust
	@if [ "$$(cargo fmt --version 2>/dev/null || true)" != ${FMT_VER_WANT} ]; \
	then \
		if rustup --version >/dev/null 2>&1; then \
			echo "# Makefile # installing rustfmt with rustup"; \
			rustup component add rustfmt-preview; \
		else \
			echo "# Makefile # rustup not found, cannot install rustfmt"; \
			exit 1; \
		fi; \
	else \
		echo "# Makefile # rustfmt ok:" ${FMT_VER_WANT}; \
	fi;

tool_clippy: tool_rust
	@if [ "$$(cargo clippy --version 2>/dev/null || true)" != ${CLP_VER_WANT} ]; \
	then \
		if rustup --version >/dev/null 2>&1; then \
			echo "# Makefile # installing clippy with rustup"; \
			rustup component add clippy-preview; \
		else \
			echo "# Makefile # rustup not found, cannot install rustfmt"; \
			exit 1; \
		fi; \
	else \
		echo "# Makefile # clippy ok:" ${CLP_VER_WANT}; \
	fi;
