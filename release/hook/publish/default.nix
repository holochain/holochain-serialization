{ pkgs, config }:
let
 name = "hcs-release-hook-publish";

 script = pkgs.writeShellScriptBin name ''
set -euo pipefail
echo "packaging for crates.io"
cargo package --manifest-path crates/holochain_json_derive/Cargo.toml
cargo package --manifest-path crates/holochain_json_api/Cargo.toml
'';
in
{
 buildInputs = [ script ];
}
