{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    kconfq = {
      url = "../..";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      kconfq,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.clangStdenv.mkDerivation {
          pname = "example";
          version = "0.1.0";
          src = ./.;

          nativeBuildInputs = [
            pkgs.meson
            pkgs.ninja
            pkgs.pkg-config
          ];

          buildInputs = [ kconfq.packages.${system}.libkconfq ];
        };
      }
    );
}
