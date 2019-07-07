{ pkgs }:
let
  name = "hcs-test";

  script = pkgs.writeShellScriptBin name
  ''
  RUST_BACKTRACE=1 \
  hn-rust-fmt-check \
  && hn-rust-clippy \
  && cargo test
  '';
in
{
 buildInputs = [ script ];
}
