# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic
Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## [Unreleased]

## [0.1.5] - 2026-01-17

- Build [docs.rs](https://docs.rs/kconfq) documentation for all features.

## [0.1.4] - 2026-01-17

- Fix failing build when no DEFAULT_CONFIG_PATH env var is provided.

## [0.1.3] - 2026-01-17

### kconfq (CLI)

#### Added

- Sub-command `find` to print config's line by its entry name.
- Sub-command `find-value` to print value of the config's entry.

### kconfq (library)

#### Added

- Function `find_line` to find config's line by its entry name.
- Function `find_value` to find value of the config's entry.
- Error `FindLineError`.
- Error `FindValueError`.

#### Changed

- Rename `GetLinuxKernelVersionError` to `GetKernelVersionError`.
- Rename `GetLinuxKernelVersionError::MissingUnameRelease` to
  `GetKernelVersionError::ReleaseMissingFromUname`.
- Rename `LocateConfigFileError` to `LocateConfigError`.
- Rename `LocateConfigFileError::ErrorGettingLinuxKernelVersion` to
  `LocateConfigError::FailedToGetLinuxKernelVersion`.
- Rename `RequireConfigFileError` to `RequireConfigError`.
- Rename `RequireConfigFileError::Locate` to
  `RequireConfigError::FailedToLocate`.
- Rename `IsGzipError::FailedToReadMagic` to
  `IsGzipError::FailedToReadFileMagic`.

### libkconfq (C-API)

#### Added

- Opaque struct `KconfConfig`.
- Function `kconfq_config_path` to get a path to the config's underlying file.
- Function `kconfq_find_line` to find config's line by its entry name.
- Function `kconfq_find_value` to find value of the config's entry.

#### Changed

- `kconfq_locate_config` returns `KconfqConfig` instead of path to config file.

## [0.1.2] - 2026-01-14

### kconfq (CLI)

#### Added

- Sub-command `path` to print path to kernel config.
- Sub-command `config` to print the whole kernel config.

### kconfq (library)

#### Added

- Function `locate_config` and `require_config` to find the location of the
  kernel config.

### libkconfq (C-API)

#### Added

- Function `kconfq_locate_config` to find the location of the kernel config.

[Unreleased]: https://github.com/synalice/kconfq/compare/v0.1.2...HEAD
[0.1.5]: https://github.com/synalice/kconfq/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/synalice/kconfq/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/synalice/kconfq/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/synalice/kconfq/releases/tag/v0.1.2
