# kconfq

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/kconfq.svg
[crates-url]: https://crates.io/crates/kconfq
[docs-badge]: https://img.shields.io/docsrs/kconfq
[docs-url]: https://docs.rs/kconfq
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/synalice/kconfq/blob/master/LICENSE
[actions-badge]: https://img.shields.io/github/actions/workflow/status/synalice/kconfq/ci.yml
[actions-url]: https://github.com/synalice/kconfq/actions/workflows/ci.yml

> [!CAUTION]
> The project is WIP and is not ready to be used yet! Please wait until the
> version `v1.0.0`.

A portable way to query kernel configuration on a live system (`/boot/config-*`
or `/proc/config.gz`).

It provides:

- A CLI (`kconfq`)
- A C-API library (`libkconfq.so`, `kconfq.h`, `kconfq.pc`)
- A Rust crate

## Usage (as a CLI)

Instaling via cargo:

```bash
cargo install kconfq
```

Instaling via Nix:

```bash
nix profile install github:synalice/kconfq
```

Running without installation:

```bash
nix run github:synalice/kconfq
```

## Usage as a C-API library

Even though the core of the library is written in Rust, it can be compiled as a
`cdynlib` with C ABI.

Alongside with `libkconfq.so` the project will also generate `kconfq.h` and
`kconfq.pc`.

All of this makes it possible to use library as any normal C dependency. Here is
an example of how this would look like in Meson

```meson
kconfq = dependency('kconfq')
```

### Building from source

To build and install library from source you have to use
[cargo-c](https://crates.io/crates/cargo-c).

```bash
cargo cbuild --release --destdir=${D} --prefix=/usr --libdir=/usr/lib64
cargo ctest
cargo cinstall --release --destdir=${D} --prefix=/usr --libdir=/usr/lib64
```

### Using inside the `flake.nix`

This is how you would add this library to your `flake.nix` and then reference it
inside your derivation's `buildInputs`

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    kconfq = {
      url = "github:synalice/kconfq";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, kconfq }:
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

          buildInputs = [ kconfq.packages.${system}.libkconfq ];
        };
      }
    );
}
```

## Cross-compilation

The project was created with easy cross-compilation in mind.

Here is how you can cross-compile `libkconfq` for `aarch64-unknown-linux-gnu`
using Nix

```bash
./scripts/cross-compile.sh aarch64-multiplatform ./nix/libkconfq.nix out
```

> [!TIP]
> To cross-compile for different host architectures, replace
> `aarch64-multiplatform` with `riscv64` or something else. [Read more
> here](https://nix.dev/tutorials/cross-compilation.html#choosing-the-host-platform-with-nix).

## License

This project is under the [MIT](https://opensource.org/license/mit) license.
