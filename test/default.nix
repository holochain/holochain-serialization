{ pkgs }:
let
  name = "hcs-test";

  script = pkgs.writeShellScriptBin name
  ''
  RUST_BACKTRACE=1 \
  hn-rust-fmt-check \
  && hn-rust-clippy \
  && cargo test

  RUST_LOG=trace cargo test \
    --manifest-path="crates/holochain_serialized_bytes/Cargo.toml" \
    --features="trace" \
    -- --nocapture
  '';
in
{
 buildInputs = [ script ];
}
