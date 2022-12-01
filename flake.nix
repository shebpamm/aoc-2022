{
  description = "Advent Of Code 2022";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs-unstable, flake-utils, ... }:
    let
      supportedSystems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
      forAllSystems = nixpkgs-unstable.lib.genAttrs supportedSystems;

      nixpkgsFor = forAllSystems (system: import nixpkgs-unstable { inherit system; });

    in
    {
      devShell = forAllSystems (system:
        let
          system = "x86_64-linux";
          pkgs = nixpkgsFor.${system};
        in
        pkgs.mkShell {
          packages = with pkgs; [
            rust-analyzer
            rustup
            bashInteractive
            pkg-config
          ];
        });
    };
}
