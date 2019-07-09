{ pkgs, config }:
let
 name = "hcs-release-hook-version";

 script = pkgs.writeShellScriptBin name ''
echo "bumping holochain_json_derive dependency versions to ${config.release.version.current} in all Cargo.toml"
find . \
 -name "Cargo.toml" \
 -not -path "**/target/**" \
 -not -path "**/.git/**" \
 -not -path "**/.cargo/**" | xargs -I {} \
 sed -i 's/^holochain_json_derive = { version = "=[0-9]\+.[0-9]\+.[0-9]\+\(-alpha[0-9]\+\)\?"/holochain_json_derive = { version = "=${config.release.version.current}"/g' {}
'';
in
{
 buildInputs = [ script ];
}
