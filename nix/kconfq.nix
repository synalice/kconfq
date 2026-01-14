# SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
#
# SPDX-License-Identifier: MIT

{
  version ? "unknown",
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  inherit version;

  pname = "kconfq";

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  env = {
    RUSTFLAGS = "-Dwarnings";
  };

  meta = {
    description = "A portable way to query kernel configuration on a live system";
    homepage = "https://github.com/synalice/kconfq";
    license = lib.licenses.mit;
    mainProgram = "kconfq";
  };
}
