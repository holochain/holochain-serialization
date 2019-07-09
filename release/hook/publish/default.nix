{ pkgs, config }:
let
 name = "hcs-release-hook-publish";

 script = pkgs.writeShellScriptBin name ''
set -euo pipefail
echo "packaging for crates.io"
# order is important here due to dependencies
for crate in holochain_json_derive holochain_json_api
do
 cargo package --manifest-path "crates/$crate/Cargo.toml"
 cargo publish --manifest-path "crates/$crate/Cargo.toml"
done
'';
in
{
 buildInputs = [ script ];
}