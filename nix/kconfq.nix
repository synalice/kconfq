# SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
#
# SPDX-License-Identifier: MIT

{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  pname = "kconfq";
  version = "0.1.0";

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  meta = {
    description = "A portable way to query kernel configuration on a live system";
    homepage = "https://github.com/synalice/kconfq";
    license = lib.licenses.mit;
    mainProgram = "kconfq";
  };
}
