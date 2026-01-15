# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic
Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## [Unreleased]

## [0.1.3] - 2026-01-15

### kconfq (Rust CLI)

#### Added

- Sub-command `find` to print config's line by its entry name.
- Sub-command `find-value` to print value of the config's entry.

### kconfq (Rust library)

#### Added

- Function `find_line` to find config's line by its entry name.
- Function `find_value` to find value of the config's entry.

### libkconfq (C-API)

#### Added

- Function `kconfq_find_line` to find config's line by its entry name.
- Function `kconfq_free_error` to free returned error.
- Function `kconfq_error_kind` to get errors's kind.
- Function `kconfq_error_message` to get error's message.
- Function `kconfq_error_cause` to get errors's cause.

#### Changed

- Replace error handling by return values with error handling by out parameters.

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
[0.1.3]: https://github.com/synalice/kconfq/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/synalice/kconfq/releases/tag/v0.1.2
