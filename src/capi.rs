// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

//! C-API that will be compiled to `cdynlib` to be used from C.
//!
//! # Warning
//!
//! This module is basically a C code written in Rust. All of this is **extremely unsafe** and
//! should be approached very carefully.

#![allow(clippy::missing_safety_doc)]
#![allow(non_camel_case_types)]

use std::ffi::{CStr, CString, c_char};

/// Create compile-time null-terminated C string.
macro_rules! cstr {
    ($s:literal) => {{
        const BYTES: &[u8] = concat!($s, "\0").as_bytes();
        // Safety: `concat!($s, "\0")` ensures null termination and no interior nulls.
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(BYTES).as_ptr() }
    }};
}

/// Assert that the pointer is not NULL or return [`KconfqResult::KCONFQ_RESULT_NULL_PARAMETER`].
macro_rules! assert_not_null {
    ($ptr:expr) => {{
        let ptr = $ptr;
        if ptr.is_null() {
            return KconfqResult::KCONFQ_RESULT_NULL_PARAMETER;
        }
    }};
}

/// Run [`assert_not_null`] on pointer, then dereference it and set to NULL.
macro_rules! assert_not_null_and_set {
    ($ptr:expr) => {{
        let ptr = $ptr;
        assert_not_null!(ptr);
        unsafe {
            *ptr = std::ptr::null_mut();
        }
    }};
}

/// Result of the function's operation.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KconfqResult {
    /// Success.
    KCONFQ_RESULT_SUCCESS = 0,
    /// Not found.
    KCONFQ_RESULT_NOT_FOUND = 1,
    /// Error getting Linux kernel version.
    KCONFQ_RESULT_KERNEL_VERSION_ERROR = 2,
    /// Parameter is a NULL pointer.
    KCONFQ_RESULT_NULL_PARAMETER = 3,
    /// Value of the function's argument is malformed.
    KCONFQ_RESULT_MALFORMED_ARGUMENT = 4,
    /// Failed while trying to parse non-UTF-8 string.
    KCONFQ_RESULT_NON_UTF8_STRING = 5,
    /// Failed to get reader to the config's file.
    KCONFQ_RESULT_FAILED_TO_GET_READER = 6,
    /// Entry is missing from the config
    KCONFQ_RESULT_MISSING_ENTRY = 7,
    /// I/O error
    KCONFQ_RESULT_IO_ERROR = 8,
    /// Failed to parse the value of the config's entry.
    KCONFQ_RESULT_FAILED_TO_PARSE_ENTRY_VALUE = 9,
    /// Unknown internal error.
    KCONFQ_RESULT_UNKNOWN_ERROR = 255,
}

/// Returns a human-readable string, describing a [`KconfqResult`].
///
/// # Returns
///
/// A pointer to a null-terminated, static string describing the status. Must NOT be freed or
/// modified by the caller.
///
/// # Safety
///
/// This function assumes `result` is a valid member of the [`KconfqResult`] enum. Passing any other
/// arbitrary integer results in an undefined behavior.
#[unsafe(no_mangle)]
#[rustfmt::skip]
pub unsafe extern "C" fn kconfq_result_strerror(result: KconfqResult) -> *const c_char {
    match result {
        KconfqResult::KCONFQ_RESULT_SUCCESS => cstr!("success"),
        KconfqResult::KCONFQ_RESULT_NOT_FOUND => cstr!("not found"),
        KconfqResult::KCONFQ_RESULT_KERNEL_VERSION_ERROR => cstr!("error getting Linux kernel version"),
        KconfqResult::KCONFQ_RESULT_NULL_PARAMETER => cstr!("parameter is a NULL pointer"),
        KconfqResult::KCONFQ_RESULT_MALFORMED_ARGUMENT => cstr!("value of the function's argument is malformed"),
        KconfqResult::KCONFQ_RESULT_NON_UTF8_STRING => cstr!("failed while trying to parse non-UTF-8 string"),
        KconfqResult::KCONFQ_RESULT_UNKNOWN_ERROR => cstr!("unknown internal error"),
        KconfqResult::KCONFQ_RESULT_FAILED_TO_GET_READER => cstr!("failed to get reader to the config's file"),
        KconfqResult::KCONFQ_RESULT_MISSING_ENTRY => cstr!("entry is missing from the config"),
        KconfqResult::KCONFQ_RESULT_IO_ERROR => cstr!("I/O error"),
        KconfqResult::KCONFQ_RESULT_FAILED_TO_PARSE_ENTRY_VALUE => cstr!("failed to parse the value of the config's entry"),
    }
}

/// Frees a string allocated by this library.
///
/// # Safety
///
/// `ptr` must be a pointer previously returned by this library, or NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_free_string(ptr: *const c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr as *mut c_char));
        }
    }
}

/// Frees a `KconfqConfig` allocated by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_free_config(ptr: *const crate::Config) {
    if !ptr.is_null() {
        unsafe {
            drop(Box::from_raw(ptr as *mut crate::Config));
        }
    }
}

/// Get a path to the config's underlying file.
///
/// # Parameters
///
/// - `config` - Pointer to `KconfqConfig` whose path we want to get.
/// - `out_path` - Pointer to a location that on success will receive the allocated constant
///   null-terminated string. Caller must free it using [`kconfq_free_string`]. Must NOT be NULL.
///
/// # Errors
///
/// - [`KconfqResult::KCONFQ_RESULT_NULL_PARAMETER`] - One of the arguments was NULL.
/// - [`KconfqResult::KCONFQ_RESULT_NON_UTF8_STRING`] - Path to config is not a valid UTF-8.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_config_path(
    config: *const crate::Config,
    out_path: *mut *const c_char,
) -> KconfqResult {
    assert_not_null!(config);
    assert_not_null_and_set!(out_path);

    let config = unsafe { &*config };

    match CString::new(config.path().to_string_lossy().as_bytes()) {
        Ok(c_string) => unsafe {
            *out_path = c_string.into_raw();
            KconfqResult::KCONFQ_RESULT_SUCCESS
        },
        Err(_) => KconfqResult::KCONFQ_RESULT_NON_UTF8_STRING,
    }
}

/// Locate the kernel config file and return path to it.
///
/// # Parameters
///
/// - `out_config` - Pointer to a location that on success will receive the allocated config. Caller
///   must free it using [`kconfq_free_config`]. Must NOT be NULL.
///
/// # Errors
///
/// - [`KconfqResult::KCONFQ_RESULT_NOT_FOUND`] - No configuration file was found at any possible
///   known location. `*out_path` was set to NULL.
/// - [`KconfqResult::KCONFQ_RESULT_KERNEL_VERSION_ERROR`] - Failed to determine the running kernel
///   version. `*out_path` was set to NULL.
/// - [`KconfqResult::KCONFQ_RESULT_NULL_PARAMETER`] - One of the arguments was NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_locate_config(
    out_config: *mut *const crate::Config,
) -> KconfqResult {
    assert_not_null_and_set!(out_config);

    match crate::locate_config() {
        Ok(Some(config)) => unsafe {
            *out_config = Box::into_raw(Box::new(config));
            KconfqResult::KCONFQ_RESULT_SUCCESS
        },
        Ok(None) => KconfqResult::KCONFQ_RESULT_NOT_FOUND,
        Err(crate::error::LocateConfigError::FailedToGetLinuxKernelVersion(_)) => {
            KconfqResult::KCONFQ_RESULT_KERNEL_VERSION_ERROR
        }
    }
}

/// Find line in the config that contains specified `entry_name`.
///
/// # Parameters
///
/// - `entry_name` - Pointer to a null-terminated C string specifying the name of the entry to
///   search for. Must NOT be NULL.
/// - `out_line` - Pointer to a location that on success will receive the allocated constant
///   null-terminated string. Caller must free it using [`kconfq_free_string`]. Must NOT be NULL.
///
/// # Example `out_line` values
///
/// - `CONFIG_FOO=y`
/// - `CONFIG_FOO=m`
/// - `CONFIG_FOO=12345`
/// - `CONFIG_FOO="something something"`
/// - `# CONFIG_FOO is not set`
///
/// # Errors
///
/// - [`KconfqResult::KCONFQ_RESULT_MISSING_ENTRY`] - Entry is missing from the config.
/// - [`KconfqResult::KCONFQ_RESULT_IO_ERROR`] - I/O error while trying to read the kernel config
///   file.
/// - [`KconfqResult::KCONFQ_RESULT_MALFORMED_ARGUMENT`] - `entry_name` is malformed.
/// - [`KconfqResult::KCONFQ_RESULT_FAILED_TO_GET_READER`] - Failed to get reader to the config's
///   file.
/// - [`KconfqResult::KCONFQ_RESULT_NON_UTF8_STRING`] - Line with desired entry was not a valid
///   UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_find_line(
    config: *const crate::Config,
    entry_name: *const c_char,
    out_line: *mut *const c_char,
) -> KconfqResult {
    assert_not_null!(config);
    assert_not_null!(entry_name);
    assert_not_null_and_set!(out_line);

    let entry_name = unsafe { CStr::from_ptr(entry_name) };
    let entry_name = match entry_name.to_str() {
        Ok(str) => str,
        Err(_) => return KconfqResult::KCONFQ_RESULT_MALFORMED_ARGUMENT,
    };

    let config = unsafe { &*config };
    let config_reader = match config.reader() {
        Ok(reader) => reader,
        Err(_) => return KconfqResult::KCONFQ_RESULT_FAILED_TO_GET_READER,
    };

    match crate::find_line(entry_name, config_reader) {
        Ok(line) => match CString::new(line.as_bytes()) {
            Ok(c_string) => unsafe {
                *out_line = c_string.into_raw();
                KconfqResult::KCONFQ_RESULT_SUCCESS
            },
            Err(_) => KconfqResult::KCONFQ_RESULT_NON_UTF8_STRING,
        },
        Err(err) => match err {
            crate::error::FindLineError::EntryIsMissing(_) => {
                KconfqResult::KCONFQ_RESULT_MISSING_ENTRY
            }
            crate::error::FindLineError::MalformedEntryName(_) => {
                KconfqResult::KCONFQ_RESULT_MALFORMED_ARGUMENT
            }
            crate::error::FindLineError::FailedToReadConfigLine(_) => {
                KconfqResult::KCONFQ_RESULT_IO_ERROR
            }
        },
    }
}

/// Same as [`kconfq_find_line`], but return only the value of the entry.
///
/// # Parameters
///
/// - `entry_name` - Pointer to a null-terminated C string specifying the name of the entry to
///   search for. Must NOT be NULL.
/// - `out_value` - Pointer to a location that on success will receive the allocated constant
///   null-terminated string. Caller must free it using [`kconfq_free_string`]. Must NOT be NULL.
///
/// # Example `out_value` values
///
/// - `y`
/// - `m`
/// - `12345`
/// - `something something`
/// - `# CONFIG_FOO is not set`
///
/// # Errors
///
/// - [`KconfqResult::KCONFQ_RESULT_MISSING_ENTRY`] - Entry is missing from the config.
/// - [`KconfqResult::KCONFQ_RESULT_IO_ERROR`] - I/O error while trying to read the kernel config
///   file.
/// - [`KconfqResult::KCONFQ_RESULT_MALFORMED_ARGUMENT`] - `entry_name` is malformed.
/// - [`KconfqResult::KCONFQ_RESULT_FAILED_TO_GET_READER`] - Failed to get reader to the config's
///   file.
/// - [`KconfqResult::KCONFQ_RESULT_NON_UTF8_STRING`] - Line with desired entry was not a valid
///   UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_find_value(
    config: *const crate::Config,
    entry_name: *const c_char,
    out_value: *mut *const c_char,
) -> KconfqResult {
    assert_not_null!(config);
    assert_not_null!(entry_name);
    assert_not_null_and_set!(out_value);

    let entry_name = unsafe { CStr::from_ptr(entry_name) };
    let entry_name = match entry_name.to_str() {
        Ok(str) => str,
        Err(_) => return KconfqResult::KCONFQ_RESULT_MALFORMED_ARGUMENT,
    };

    let config = unsafe { &*config };
    let config_reader = match config.reader() {
        Ok(reader) => reader,
        Err(_) => return KconfqResult::KCONFQ_RESULT_FAILED_TO_GET_READER,
    };

    match crate::find_value(entry_name, config_reader) {
        Ok(line) => match CString::new(line.as_bytes()) {
            Ok(c_string) => unsafe {
                *out_value = c_string.into_raw();
                KconfqResult::KCONFQ_RESULT_SUCCESS
            },
            Err(_) => KconfqResult::KCONFQ_RESULT_NON_UTF8_STRING,
        },
        Err(err) => match err {
            crate::error::FindValueError::FailedToFindLine(err) => match err {
                crate::error::FindLineError::EntryIsMissing(_) => {
                    KconfqResult::KCONFQ_RESULT_MISSING_ENTRY
                }
                crate::error::FindLineError::MalformedEntryName(_) => {
                    KconfqResult::KCONFQ_RESULT_MALFORMED_ARGUMENT
                }
                crate::error::FindLineError::FailedToReadConfigLine(_) => {
                    KconfqResult::KCONFQ_RESULT_IO_ERROR
                }
            },
            crate::error::FindValueError::FailedToParseLine(_) => {
                KconfqResult::KCONFQ_RESULT_FAILED_TO_PARSE_ENTRY_VALUE
            }
        },
    }
}
