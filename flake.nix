{
  description = "An anyrun plugin that lets you switch sway windows";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs =
    {
      nixpkgs,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystem = nixpkgs.lib.genAttrs systems;
    in
    {
      packages = forAllSystem (system: rec {
        anyrun-swaywin = nixpkgs.legacyPackages.${system}.rustPlatform.buildRustPackage {
          pname = "anyrun-swaywin";
          version = "0.1.0";
          cargoLock.lockFile = ./Cargo.lock;
          src = ./.;
        };
        default = anyrun-swaywin;
      });
    };
}
