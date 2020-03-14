{ pkgs, config }:
let
 name = "hcs-release-hook-version";

 script = pkgs.writeShellScriptBin name ''
echo "bumping holochain_json_derive dependency versions to ${config.release.version.current} in all Cargo.toml"
for $crate in holochain_json_derive holochain_serialized_bytes
do
 find . \
  -name "Cargo.toml" \
  -not -path "**/target/**" \
  -not -path "**/.git/**" \
  -not -path "**/.cargo/**" | xargs -I {} \
  sed -i 's/^''${crate} = { version = "=[0-9]\+.[0-9]\+.[0-9]\+\(-alpha[0-9]\+\)\?"/''${crate} = { version = "=${config.release.version.current}"/g' {}
done
'';
in
{
 buildInputs = [ script ];
}
