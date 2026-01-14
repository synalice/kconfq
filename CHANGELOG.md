# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic
Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## [Unreleased]

## [0.1.2] - 2026-01-14

### kconfq (Rust CLI)

#### Added

- Sub-command `path` to print path to kernel config.
- Sub-command `config` to print the whole kernel config.

### kconfq (Rust library)

#### Added

- Function `locate_config` and `require_config` to find the location of the
  kernel config.

### libkconfq (C-API)

#### Added

- Function `kconfq_locate_config` to find the location of the kernel config.

[Unreleased]: https://github.com/synalice/kconfq/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/synalice/kconfq/releases/tag/v0.1.2
