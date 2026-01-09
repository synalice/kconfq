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

        packages.kconfq = pkgs.callPackage ./nix/kconfq.nix { };

        packages.libkconfq = pkgs.callPackage ./nix/libkconfq.nix { stdenv = pkgs.clangStdenv; };

        devShells.default =
          pkgs.mkShell.override
            {
              stdenv = pkgs.clangStdenv;
            }
            {
              inputsFrom = [ self.packages.${system}.libkconfq ];

              buildInputs = [
                pkgs.bash
                pkgs.mesonlsp
                pkgs.rust-analyzer
                pkgs.rustfmt
                pkgs.clippy
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
