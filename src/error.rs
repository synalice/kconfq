// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

//! All possible library errors.

use std::io;

use nix::errno::Errno;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetKernelVersionError {
    #[error("uname syscall returned errno {0}")]
    UnameError(Errno),
    #[error("release level of the OS is missing from uname")]
    ReleaseMissingFromUname,
}

#[derive(Error, Debug)]
pub enum LocateConfigError {
    #[error("failed to get linux kernel version")]
    FailedToGetLinuxKernelVersion(#[from] GetKernelVersionError),
}

#[derive(Error, Debug)]
pub enum RequireConfigError {
    #[error("failed to locate kernel config file")]
    FailedToLocate(#[from] LocateConfigError),
    #[error("kernel config file not found in any known location")]
    NotFound,
}

#[derive(Error, Debug)]
pub enum IsGzipError {
    #[error("failed to open kernel config file: {0}")]
    FailedToOpenFile(io::Error),
    #[error("failed to read magic of the kernel config file: {0}")]
    FailedToReadFileMagic(io::Error),
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
pub enum FindLineError {
    #[error("entry \"{0}\" is missing from the config")]
    EntryIsMissing(String),
    #[error("failed to get a reader to a kernel config file")]
    FailedToGetReader(#[from] GettingConfigReaderError),
    #[error(transparent)]
    FailedToFindConfig(#[from] RequireConfigError),
    #[error("entry name is malformed")]
    MalformedEntryName(#[from] regex::Error),
    #[error("failed to read kernel config file")]
    FailedToReadConfig(#[from] io::Error),
}

#[derive(Debug, Error)]
#[allow(unused)]
pub enum FindValueError {
    #[error("failed to find a line with a specified entry")]
    FailedToFindLine(#[from] FindLineError),
    #[error("failed to parse value from line {0}")]
    FailedToParseLine(String),
}
