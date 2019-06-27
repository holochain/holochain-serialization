{ pkgs }:
let
  name = "hcs-test";

  script = pkgs.writeShellScriptBin name
  ''
  hn-rust-fmt-check
  '';
in
{
 buildInputs = [ script ];
}
