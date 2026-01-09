#!/usr/bin/env bash
#
# SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
#
# SPDX-License-Identifier: MIT

set -o errexit
set -o pipefail
set -o nounset

usage() {
  cat >&2 <<EOF
Usage: $0 <arch> <out|dev|doc> <package>

  arch     cross architecture (aarch64-multiplatform, riscv64, etc.)
  package  nix package path (./nix/libkconfq.nix)
  output   desired output (out, dev, doc)
EOF
  exit 1
}

(( $# == 3 )) || usage

nix build -I nixpkgs=flake:nixpkgs --impure --expr "
let
    pkgs = import <nixpkgs> {};
in
    (pkgs.pkgsCross.$1.callPackage $2 {}).$3
"
