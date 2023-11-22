{
  inputs = {
    nixpkgs.follows = "holonix/nixpkgs";

    versions.url = "github:holochain/holochain?dir=versions/weekly";
    holonix.url = "github:holochain/holochain";
    holonix.inputs.versions.follows = "versions";
  };

  outputs = inputs@{ holonix, ... }:
    holonix.inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = builtins.attrNames holonix.devShells;

      perSystem = { config, system, pkgs, ... }:
        {
          devShells.default = pkgs.mkShell {
            inputsFrom = [ holonix.devShells.${system}.coreDev ];
            packages = with pkgs; [
              # add further packages from nixpkgs
            ];
          };
        };
    };
}
