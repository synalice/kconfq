# SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
#
# SPDX-License-Identifier: MIT

{
  # For VS Code to use the right version of rust-analyzer, add this to your settings:
  # "rust-analyzer.server.path": "rust-analyzer"

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        lib = pkgs.lib;
        rustToolchain =
          (fenix.packages.${system}.fromToolchainName {
            name = (pkgs.lib.importTOML ./rust-toolchain.toml).toolchain.channel;
            sha256 = "sha256-sqSWJDUxc+zaz1nBWMAJKTAGBuGWP25GCftIOlCEAtA=";
          }).toolchain;
      in
      {
        formatter = pkgs.nixfmt-rfc-style;

        apps.default = {
          type = "app";
          program = lib.getExe self.packages.${system}.kconfq;
        };

        packages.default = self.packages.${system}.kconfq;

        packages.kconfq =
          (pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          }).buildRustPackage
            {
              pname = "kconfq";
              version = "0.1.0";

              src = ./.;

              cargoLock.lockFile = ./Cargo.lock;

              meta = {
                description = "A portable way to query kernel configuration on a live system";
                homepage = "https://github.com/synalice/kconfq";
                license = lib.licenses.mit;
                mainProgram = "kconfq";
              };
            };

        packages.libkconfq = pkgs.stdenv.mkDerivation {
          pname = "libkconfq";
          version = "0.1.0";
          src = ./.;

          doCheck = true;

          outputs = [
            "out"
            "dev"
          ];

          nativeBuildInputs = [
            rustToolchain
            pkgs.meson
            pkgs.ninja
            pkgs.pkg-config
            pkgs.rust-cbindgen
          ];

          meta = {
            description = "A portable way to query kernel configuration on a live system";
            homepage = "https://github.com/synalice/kconfq";
            license = lib.licenses.mit;
          };
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.libkconfq ];

          buildInputs = [
            rustToolchain
            pkgs.bash
            pkgs.mesonlsp
            pkgs.prek
            pkgs.reuse
            pkgs.jq
            pkgs.tree
            # pre-commit hooks from https://github.com/pre-commit/pre-commit-hooks repo invoke it
            pkgs.uv
          ];

          shellHook = ''
            prek install
          '';
        };
      }
    );
}
