// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

//! All possible library errors.

use std::io;

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

#[derive(Error, Debug)]
pub enum IsGzipError {
    #[error("failed to open kernel config file: {0}")]
    FailedToOpenFile(io::Error),
    #[error("failed to read magic of the kernel config file: {0}")]
    FailedToReadMagic(io::Error),
}

#[derive(Error, Debug)]
pub enum GettingConfigReaderError {
    #[error("failed to open kernel config file: {0}")]
    FailedToOpenFile(io::Error),
    #[error("failed to check whenever the file is gzip-compressed or not")]
    GzipError(#[from] IsGzipError),
}

#[derive(Debug, Error)]
#[allow(unused)]
pub enum GetEntryError {
    #[error("entry \"{0}\" was not found")]
    EntryNotFound(String),
    #[error("failed to get a reader to a kernel config file")]
    ConfigReaderError(#[from] GettingConfigReaderError),
    #[error(transparent)]
    FailedToFindConfig(#[from] RequireConfigFileError),
    #[error("entry name is malformed")]
    MalformedEntryName(#[from] regex::Error),
    #[error("failed to read kernel config file")]
    FailedToReadConfig(#[from] io::Error),
}
