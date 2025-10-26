{
  description = "An anyrun plugin that lets you switch sway windows";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      nixpkgs,
      fenix,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    {
      packages = nixpkgs.lib.genAttrs systems (
        system:
        let
          toolchain = fenix.packages.${system}.complete.toolchain;
          pkgs = nixpkgs.legacyPackages.${system};
          rustPlatform = pkgs.makeRustPlatform {
            rustc = toolchain;
            cargo = toolchain;
            rustrc = toolchain;
          };
        in
        rec {
          anyrun-swaywin = rustPlatform.buildRustPackage {
            pname = "anyrun-swaywin";
            version = "0.1.0";
            cargoLock.lockFile = ./Cargo.lock;
            cargoLock.outputHashes = {
              "anyrun-interface-25.9.3" = "sha256-ynLb+3Y+sbrNc2HD1VTRNBj2GKm44CGENTJZwvn0Xt0=";
              "anyrun-macros-25.9.3" = "sha256-WGoqR+ULsh1w7yDNAtJiE27HX6zSlGPR4I2pgjAU/SA=";
            };
            src = ./.;
          };
          default = anyrun-swaywin;
        }
      );
    };
}
