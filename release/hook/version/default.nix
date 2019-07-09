{ pkgs, config }:
let
 name = "hcs-release-hook-version";

 script = pkgs.writeShellScriptBin name ''
echo "bumping holochain_json_derive dependency versions to ${config.release.version.current} in all Cargo.toml"
find . \
 -name "Cargo.toml" \
 -not -path "**/.git/**" \
 -not -path "**/.cargo/**" | xargs -I {} \
 sed -i 's/^\s*holochain_json_derive\s*=\s*{\s*version\s*=\s*"=\d+\.\d+\.\d+(-alpha\d*)?"/holochain_json_derive = {version = "=${config.release.version.current}"/g' {}
'';
in
{
 buildInputs = [ script ];
}
