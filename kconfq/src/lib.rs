// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use error::*;

pub mod error;

pub struct Config {
    path: PathBuf,
}

impl Config {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Detect whenever the config file is gzip-compressed or not.
    pub fn is_gzip(self) -> Result<bool, IsGzipError> {
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

/// Same as [`locate_config_file`], but return and error if config was not
/// found.
pub fn require_config() -> Result<Config, RequireConfigFileError> {
    locate_config()?.ok_or(RequireConfigFileError::NotFound)
}

/// Get the version specified by `uname -r`.
///
/// This treats everything after the `major.minor.patch` triple as build metadata.
fn get_linux_kernel_version() -> Result<String, GetLinuxKernelVersionError> {
    let uname = nix::sys::utsname::uname().map_err(GetLinuxKernelVersionError::UnameError)?;

    Ok(uname
        .release()
        .to_str()
        .ok_or(GetLinuxKernelVersionError::MissingUnameRelease)?
        .to_owned())
}
