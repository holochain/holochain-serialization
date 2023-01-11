.PHONY: all test fmt clean

all: test

fmt:
	nix-shell --run "hn-rust-fmt && hn-rust-clippy"

clean:
	nix-shell --run hn-flush

test:
	nix-shell --run "cargo test"