// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetLinuxKernelVersionError {
    #[error("uname syscall returned an error: {0}")]
    UnameError(&'static str),
    #[error("release level of the OS is missing from uname")]
    MissingUnameRelease,
}

/// Get the version specified by `uname -r`.
///
/// This treats everything after the `major.minor.patch` triple as build metadata.
fn get_linux_kernel_version() -> Result<String, GetLinuxKernelVersionError> {
    let uname = nix::sys::utsname::uname()
        .map_err(|err| GetLinuxKernelVersionError::UnameError(err.desc()))?;

    Ok(uname
        .release()
        .to_str()
        .ok_or(GetLinuxKernelVersionError::MissingUnameRelease)?
        .to_owned())
}
