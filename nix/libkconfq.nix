# SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
#
# SPDX-License-Identifier: MIT

{
  lib,
  stdenv,
  meson,
  cargo,
  rustc,
  ninja,
  pkg-config,
  rust-cbindgen,
}:
stdenv.mkDerivation {
  pname = "libkconfq";
  version = "0.1.1";
  src = ../.;

  doCheck = true;

  outputs = [
    "out"
    "dev"
  ];

  nativeBuildInputs = [
    cargo
    rustc
    meson
    ninja
    pkg-config
    rust-cbindgen
  ];

  meta = {
    description = "A portable way to query kernel configuration on a live system";
    homepage = "https://github.com/synalice/kconfq";
    license = lib.licenses.mit;
  };
}
