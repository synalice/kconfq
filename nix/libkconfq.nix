# SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
#
# SPDX-License-Identifier: MIT

{
  lib,
  rustPlatform,
  cargo-c,
  rust,
  stdenv,
}:

rustPlatform.buildRustPackage rec {
  pname = "libkconfq";
  version = "0.1.1";
  src = ../.;

  outputs = [
    "out"
    "dev"
  ];

  doCheck = true;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [ cargo-c ];

  env = {
    RUSTFLAGS = "-Dwarnings";
  };

  buildPhase = ''
    runHook preBuild
    ${rust.envVars.setEnv} cargo cbuild -j $NIX_BUILD_CORES --release --frozen --prefix=${placeholder "out"} --target ${stdenv.hostPlatform.rust.rustcTarget}
    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall
    ${rust.envVars.setEnv} cargo cinstall -j $NIX_BUILD_CORES --release --frozen --prefix=${placeholder "out"} --target ${stdenv.hostPlatform.rust.rustcTarget}
    runHook postInstall
  '';

  checkPhase = ''
    runHook preCheck
    ${rust.envVars.setEnv} cargo ctest -j $NIX_BUILD_CORES --release --frozen --prefix=${placeholder "out"} --target ${stdenv.hostPlatform.rust.rustcTarget}
    runHook postCheck
  '';

  meta = {
    description = "A portable way to query kernel configuration on a live system";
    homepage = "https://github.com/synalice/kconfq";
    license = lib.licenses.mit;
  };
}
