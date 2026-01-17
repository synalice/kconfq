# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic
Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## [1.0.0] - 2026-01-17

Release of the first stable version! 🎉

### kconfq (CLI)

#### Added

- Sub-command `is-gzip` to print whenever the config's file is gzip-compressed
  or not.
- Global arg `--path` to specify location of the config file.

### kconfq (library)

#### Removed

- Unused `ConfigEntry` struct.
- Unused `ConfigValue` enum.

#### Changed

- Functions `locate_config` and `require_config` now require an optional
  `default_path` argument.
- Update error strings.

### libkconfq (C-API)

#### Added

- Function `kconfq_config_is_gzip` to check whenever the config's file is
  gzip-compressed or not.

#### Changed

- Function `kconfq_locate_config` now takes an optional argument `default_path`.

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

[1.0.0]: https://github.com/synalice/kconfq/releases/tag/v1.0.0
[0.1.3]: https://github.com/synalice/kconfq/releases/tag/v0.1.3
[0.1.2]: https://github.com/synalice/kconfq/releases/tag/v0.1.2
