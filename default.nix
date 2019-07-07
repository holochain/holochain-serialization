let
 holonix-release-tag = "0.0.4";
 holonix-release-sha256 = "04xbhr0w0fc911vjd4f42shmy3vlnvwpicjnsk0q8gr4prkpz74h";

 holonix = import (fetchTarball {
  url = "https://github.com/holochain/holonix/tarball/${holonix-release-tag}";
  sha256 = "${holonix-release-sha256}";
 });
 # uncomment to work locally
 # holonix = import ../holonix;
in
with holonix.pkgs;
{
 core-shell = stdenv.mkDerivation (holonix.shell // {
  name = "holochain-serialization-shell";

  buildInputs = []
   ++ holonix.shell.buildInputs
   ++ (holonix.pkgs.callPackage ./test {
    pkgs = holonix.pkgs;
   }).buildInputs
  ;
 });
}
