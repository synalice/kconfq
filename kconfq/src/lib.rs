// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use nix::errno::Errno;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetLinuxKernelVersionError {
    #[error("uname syscall returned with an errno {0}")]
    UnameError(Errno),
    #[error("release level of the OS is missing from uname")]
    MissingUnameRelease,
}

#[derive(Error, Debug)]
pub enum LocateConfigFileError {
    #[error("error getting linux kernel version")]
    ErrorGettignLinuxKernelVersion(#[from] GetLinuxKernelVersionError),
}

/// Searches through all possible config locations and returns path to it if
/// found.
pub fn locate_config_file() -> Result<Option<PathBuf>, LocateConfigFileError> {
    let uname_r = get_linux_kernel_version()?;

    let possible_paths = [
        PathBuf::from("/proc/config.gz"),
        PathBuf::from(&format!("/boot/config-{uname_r}")),
    ];

    for p in possible_paths {
        if p.exists() {
            return Ok(Some(p));
        }
    }

    Ok(None)
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
