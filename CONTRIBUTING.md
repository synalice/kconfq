# Contributing to kconfq

I'm really glad you're reading this, because contributions are always very
welcomed!

## Setting up developer environment

The project uses `devShells` from [flake.nix](flake.nix) to setup a developer
environment. There is also a [.envrc](.envrc) file that automatically enables
`devShell` when you `cd` into project's directory.

To use all of that you want to [install Nix](https://nixos.org/download/) and
[install direnv](https://direnv.net/). After then Nix will automatically
download and install all dependencies required for developing the project. That
may take some time.

You can also avoid using direnv and manually invoke `nix develop` each time you
navigate to the project's directory.

Alternatively, you can skim the [flake.nix](flake.nix) file and install
everything manually from your own package manager (not recommended).

## Workflow

The easiest way to check that your changes compile is to run

```bash
nix build .#kconfq
nix build .#libkconfq
```

## Pre-commit hooks

The project has a lot different pre-commit hooks that ensure the code is
properly formatted and licensed.

These hooks will be installed automatically if you've used `nix develop` or
direnv. Alternatively, you can install them manually

```bash
prek install
```
