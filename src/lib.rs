// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use std::sync::LazyLock;

use flate2::read::GzDecoder;
use regex::Regex;

use error::*;

#[cfg(feature = "capi")]
pub mod capi;
pub mod error;

/// A kernel config struct.
pub struct Config {
    path: PathBuf,
}

/// An entry in the kernel config.
///
/// # Examples
///
/// `# CONFIG_EFI_PGT_DUMP is not set` will become
/// ```rust
/// # use kconfq::{ConfigEntry, ConfigValue};
/// ConfigEntry::new(
///     "CONFIG_EFI_PGT_DUMP",
///     ConfigValue::No,
/// );
/// ```
///
/// `CONFIG_CC_IS_GCC=y` will become
/// ```rust
/// # use kconfq::{ConfigEntry, ConfigValue};
/// ConfigEntry::new(
///     "CONFIG_CC_IS_GCC",
///     ConfigValue::Yes,
/// );
/// ```
///
/// `CONFIG_IKHEADERS=m` will become
/// ```rust
/// # use kconfq::{ConfigEntry, ConfigValue};
/// ConfigEntry::new(
///     "CONFIG_IKHEADERS",
///     ConfigValue::Module,
/// );
/// ```
///
/// `CONFIG_CC_VERSION_TEXT="gcc (GCC) 14.3.0"` will become
/// ```rust
/// # use kconfq::{ConfigEntry, ConfigValue};
/// ConfigEntry::new(
///     "CONFIG_CC_VERSION_TEXT",
///     ConfigValue::Value("gcc (GCC) 14.3.0".to_string()),
/// );
/// ```
///
/// `CONFIG_GCC_VERSION=140300` will become
/// ```rust
/// # use kconfq::{ConfigEntry, ConfigValue};
/// ConfigEntry::new(
///     "CONFIG_GCC_VERSION",
///     ConfigValue::Value("140300".to_string()),
/// );
/// ```
pub struct ConfigEntry {
    name: String,
    value: ConfigValue,
}

/// Possible value of the [`ConfigEntry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigValue {
    /// Example: `CONFIG_CC_IS_GCC=y`
    Yes,
    /// Example: `CONFIG_IKHEADERS=m`
    Module,
    /// Example: `# CONFIG_EFI_PGT_DUMP is not set`
    No,
    /// Example: `CONFIG_GCC_VERSION=140300`
    Value(String),
}

impl ConfigEntry {
    pub fn new(name: impl Into<String>, value: ConfigValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &ConfigValue {
        &self.value
    }
}

impl Config {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    /// Get a reader to a kernel config file.
    pub fn reader(&self) -> Result<Box<dyn Read>, GettingConfigReaderError> {
        let config_file =
            File::open(self.path()).map_err(GettingConfigReaderError::FailedToOpenFile)?;

        if self.is_gzip()? {
            Ok(Box::new(GzDecoder::new(config_file)))
        } else {
            Ok(Box::new(BufReader::new(config_file)))
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Detect whenever the config file is gzip-compressed or not.
    pub fn is_gzip(&self) -> Result<bool, IsGzipError> {
        let file = File::open(self.path()).map_err(IsGzipError::FailedToOpenFile)?;
        let mut reader = BufReader::new(file);

        const GZIP_MAGIC: [u8; 2] = [0x1F, 0x8B];

        let mut magic = [0u8; GZIP_MAGIC.len()];
        let n = reader
            .read(&mut magic)
            .map_err(IsGzipError::FailedToReadMagic)?;

        Ok(n == GZIP_MAGIC.len() && magic == GZIP_MAGIC)
    }
}

/// Search through all known config locations and return a path to it.
///
/// May not find a config an return `Ok(None)`
pub fn locate_config() -> Result<Option<Config>, LocateConfigFileError> {
    let default_path = PathBuf::from(env!("DEFAULT_CONFIG_PATH"));

    if default_path.exists() {
        return Ok(Some(Config { path: default_path }));
    }

    let proc_path = PathBuf::from("/proc/config.gz");

    if proc_path.exists() {
        return Ok(Some(Config { path: proc_path }));
    }

    let uname_r = get_linux_kernel_version()?;
    let boot_path = PathBuf::from(&format!("/boot/config-{uname_r}"));

    if boot_path.exists() {
        return Ok(Some(Config { path: boot_path }));
    }

    Ok(None)
}

/// Same as [`locate_config`], but return and error if config was not
/// found.
pub fn require_config() -> Result<Config, RequireConfigFileError> {
    locate_config()?.ok_or(RequireConfigFileError::NotFound)
}

/// Find line in the config that contains specified `entry_name`.
///
/// # Examples
///
/// `entry_name == "CONFIG_CC_VERSION_TEXT"` may return\
///  `CONFIG_CC_VERSION_TEXT="gcc (GCC) 14.3.0"`
///
/// `entry_name == "CONFIG_CC_IS_GCC"` may return\
///  `CONFIG_CC_IS_GCC=y`
///
/// `entry_name == "CONFIG_COMPILE_TEST"` may return\
///  `# CONFIG_COMPILE_TEST is not set`
pub fn find_line(entry_name: &str) -> Result<String, error::FindLineError> {
    let config_reader = require_config()?.reader()?;
    let config_reader = BufReader::new(config_reader);

    if is_config_entry_name_valid(entry_name) {
        let name = entry_name.trim();

        let regex_is_not_set = Regex::new(&format!(r"^# {} is not set", regex::escape(name)))?;
        let regex_is_set = Regex::new(&format!(r"^{}=.*$", regex::escape(name)))?;

        for line in config_reader.lines() {
            let line = line?;
            let line = line.trim();

            if regex_is_not_set.find(line).is_some() || regex_is_set.find(line).is_some() {
                return Ok(line.to_string());
            }
        }
    }

    Err(FindLineError::EntryIsMissing(entry_name.to_string()))
}

/// Same as [`find_line`], but returns only the value of the entry.
///
/// # Examples
///
/// `entry_name == "CONFIG_CC_VERSION_TEXT"` may return\
///  `gcc (GCC) 14.3.0`
///
/// `entry_name == "CONFIG_CC_IS_GCC"` may return\
///  `y`
///
/// `entry_name == "CONFIG_COMPILE_TEST"` may return\
///  `# CONFIG_COMPILE_TEST is not set`
pub fn find_value(entry_name: &str) -> Result<String, error::FindValueError> {
    let line = find_line(entry_name)?;

    if line.starts_with("#") {
        return Ok(line);
    }

    let split_line = line.split("=").collect::<Vec<&str>>();

    if split_line.len() != 2 {
        return Err(FindValueError::FailedToParseLine(line));
    }

    Ok(split_line
        .get(1)
        .expect("vec should have exactly 2 items")
        .to_string())
}

/// Check that `name` is a valid config name (`CONFIG_FOO_BAR` instead of
/// `abracadabra` or something else).
fn is_config_entry_name_valid(name: &str) -> bool {
    static VALID_CONFIG_ENTRY_NAME_REGEX: LazyLock<Regex> = std::sync::LazyLock::new(|| {
        Regex::new(r"^CONFIG_[A-Z0-9_]+$").expect("hardcoded regex should be valid")
    });

    VALID_CONFIG_ENTRY_NAME_REGEX.is_match(name)
}

/// Get the version specified by `uname -r`.
///
/// This treats everything after the `major.minor.patch` triple as build metadata.
fn get_linux_kernel_version() -> Result<String, GetKernelVersionError> {
    let uname = nix::sys::utsname::uname().map_err(GetKernelVersionError::UnameError)?;

    Ok(uname
        .release()
        .to_str()
        .ok_or(GetKernelVersionError::MissingUnameRelease)?
        .to_owned())
}
