// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use error::*;

pub mod error;

/// Search through all known config locations and return a path to it.
///
/// May not find a config an return `Ok(None)`
pub fn locate_config_file() -> Result<Option<PathBuf>, LocateConfigFileError> {
    let default_path = PathBuf::from(env!("DEFAULT_CONFIG_PATH"));

    if default_path.exists() {
        return Ok(Some(default_path));
    }

    let proc_path = PathBuf::from("/proc/config.gz");

    if proc_path.exists() {
        return Ok(Some(proc_path));
    }

    let uname_r = get_linux_kernel_version()?;
    let boot_path = PathBuf::from(&format!("/boot/config-{uname_r}"));

    if boot_path.exists() {
        return Ok(Some(boot_path));
    }

    Ok(None)
}

/// Same as [`locate_config_file`], but return and error if config was not
/// found.
pub fn require_config_file() -> Result<PathBuf, RequireConfigFileError> {
    locate_config_file()?.ok_or(RequireConfigFileError::NotFound)
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
