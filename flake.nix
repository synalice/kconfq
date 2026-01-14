# SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
#
# SPDX-License-Identifier: MIT

{
  # For VS Code to use the right version of rust-analyzer, add this to your settings:
  # "rust-analyzer.server.path": "rust-analyzer"

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
        pkgs = nixpkgs.legacyPackages.${system};
        lib = pkgs.lib;
      in
      {
        formatter = pkgs.nixfmt-rfc-style;

        apps.default = {
          type = "app";
          program = lib.getExe self.packages.${system}.kconfq;
        };

        packages.default = self.packages.${system}.kconfq;

        packages.kconfq = pkgs.callPackage ./nix/kconfq.nix { inherit version; };

        packages.libkconfq = pkgs.callPackage ./nix/libkconfq.nix {
          inherit version;
          stdenv = pkgs.clangStdenv;
        };

        devShells.default =
          pkgs.mkShell.override
            {
              stdenv = pkgs.clangStdenv;
            }
            {
              inputsFrom = [
                self.packages.${system}.kconfq
                self.packages.${system}.libkconfq
              ];

              packages = [
                pkgs.bash
                pkgs.rust-analyzer
                pkgs.rustfmt
                pkgs.clippy
                pkgs.prek
                pkgs.reuse
                pkgs.jq
                pkgs.tree
                # pre-commit hooks from https://github.com/pre-commit/pre-commit-hooks repo invoke it
                pkgs.uv
                pkgs.cargo-edit
              ];

              shellHook = ''
                prek install
              '';
            };
      }
    );
}
