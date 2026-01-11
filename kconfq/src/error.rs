// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

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
    ErrorGettingLinuxKernelVersion(#[from] GetLinuxKernelVersionError),
}

#[derive(Error, Debug)]
pub enum RequireConfigFileError {
    #[error("failed to locate kernel config file")]
    Locate(#[from] LocateConfigFileError),
    #[error("kernel config file not found in any known location")]
    NotFound,
}
