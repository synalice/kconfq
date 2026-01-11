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

## Using as a CLI

### Nix

To run the CLI without installing it

```bash
nix run github:synalice/kconfq
```

To install the CLI

```bash
nix profile install github:synalice/kconfq
```

### Cargo

To install the CLI from [crates.io](https://crates.io/crates/kconfq)

```bash
cargo install kconfq
```

## Using as a C-API library

In all of the examples bellow `kconfq.pc` file would be installed so that the
library can be found by `pkg-config`.

In Meson you would then find the library like this

```meson
kconfq = dependency('kconfq')
```

### Building from source

```bash
meson setup builddir/
meson compile -C builddir/
meson install -C builddir/
```

This will build and then install `libkconfq.so`, `kconfq.h` and `kconfq.pc`.

### Using with `flake.nix`

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
