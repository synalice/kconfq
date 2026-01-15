// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

//! C-API that will be compiled to `cdynlib` to be used from C.
//!
//! # Important
//!
//! This module is basically a C code written in Rust. All of this is extremely
//! unsafe and should be written and modified very carefully.

#![allow(clippy::missing_safety_doc)]
#![allow(non_camel_case_types)]

use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_char;
use std::ptr;

use crate::capi_helpers::make_kconfq_error;
use crate::error::GetKernelVersionError;

/// Create compile-time null-terminated C string.
#[macro_export]
macro_rules! cstr {
    ($s:literal) => {{
        const BYTES: &[u8] = concat!($s, "\0").as_bytes();
        // Safety: `concat!($s, "\0")` ensures null termination and no interior nulls.
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(BYTES).as_ptr() }
    }};
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KconfqErrorKind {
    KCONFQ_ERROR_KIND_UNKNOWN = 0,
    KCONFQ_ERROR_KIND_UNAME = 1,
    KCONFQ_ERROR_KIND_MISSING_UNAME_RELEASE = 2,
    KCONFQ_ERROR_KIND_NUL_BYTE = 3,
    KCONFQ_ERROR_KIND_NOT_FOUND = 4,
    KCONFQ_ERROR_KIND_INVALID_UTF8 = 5,
}

/// A struct that represents kconfq error.
#[repr(C)]
pub struct KconfqError {
    /// What specific kind of error has happened.
    pub kind: KconfqErrorKind,
    /// Owned, C string. May be null if no message.
    pub message: *mut c_char,
    /// Owned pointer to cause (or NULL).
    pub cause: *mut KconfqError,
}

trait IntoKconfqError {
    fn into_kconfq_error(self) -> *mut KconfqError;
}

impl IntoKconfqError for GetKernelVersionError {
    fn into_kconfq_error(self) -> *mut KconfqError {
        match self {
            GetKernelVersionError::UnameError(_) => make_kconfq_error(
                KconfqErrorKind::KCONFQ_ERROR_KIND_UNAME,
                self.to_string(),
                None,
            ),
            GetKernelVersionError::MissingUnameRelease => make_kconfq_error(
                KconfqErrorKind::KCONFQ_ERROR_KIND_MISSING_UNAME_RELEASE,
                self.to_string(),
                None,
            ),
        }
    }
}

impl IntoKconfqError for NulError {
    fn into_kconfq_error(self) -> *mut KconfqError {
        make_kconfq_error(
            KconfqErrorKind::KCONFQ_ERROR_KIND_NUL_BYTE,
            self.to_string(),
            None,
        )
    }
}

/// Frees a string allocated by this library.
///
/// # Safety
///
/// `ptr` must be a pointer previously returned by this library, or `NULL`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr));
        }
    }
}

/// Frees an error allocated by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_free_error(err: *mut KconfqError) {
    if err.is_null() {
        return;
    }

    unsafe {
        // Reclaim Box
        let boxed: Box<KconfqError> = Box::from_raw(err);

        // Free message
        if !boxed.message.is_null() {
            drop(CString::from_raw(boxed.message));
        }

        // Free cause (recursively)
        if !boxed.cause.is_null() {
            kconfq_free_error(boxed.cause);
        }

        drop(boxed);
    }
}

/// Get error kind.
///
/// Returns [`KconfqErrorKind::KCONFQ_ERROR_KIND_UNKNOWN`] if `err` is NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_error_kind(err: *const KconfqError) -> KconfqErrorKind {
    if err.is_null() {
        return KconfqErrorKind::KCONFQ_ERROR_KIND_UNKNOWN;
    }
    unsafe { (*err).kind }
}

/// Get error message.
///
/// Returns `NULL` if `err` is NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_error_message(err: *const KconfqError) -> *const c_char {
    if err.is_null() {
        return ptr::null();
    }
    unsafe { (*err).message as *const c_char }
}

/// Get the cause of an error.
///
/// Returns `NULL` if `err` is NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_error_cause(err: *const KconfqError) -> *mut KconfqError {
    if err.is_null() {
        return ptr::null_mut();
    }
    unsafe { (*err).cause }
}

/// Locate the kernel config file and return its path.
///
/// On success, this function allocates a null-terminated C string containing
/// the path to the configuration file and stores a pointer to immutable
/// characters in `*out_path`.
///
/// # Ownership
///
/// - `out_path` must be freed by the caller by using [`kconfq_free_string`].
/// - `out_err` must be freed by the caller by using [`kconfq_free_error`].
///
/// # Parameters
///
/// - `out_path` - Pointer to a location that will receive the allocated string
///   on success. Must not be `NULL`.
/// - `out_err` - Pointer to an error that happened during execution. Must not
///   be `NULL`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_locate_config(
    out_path: *mut *mut c_char,
    out_err: *mut *mut KconfqError,
) {
    assert!(!out_path.is_null());
    assert!(!out_err.is_null());

    match crate::locate_config() {
        Ok(Some(config)) => match CString::new(config.path.to_string_lossy().as_bytes()) {
            Ok(c_string) => unsafe {
                *out_path = c_string.into_raw();
                *out_err = ptr::null_mut();
            },
            Err(err) => unsafe {
                *out_path = ptr::null_mut();
                *out_err = make_kconfq_error(
                    KconfqErrorKind::KCONFQ_ERROR_KIND_NUL_BYTE,
                    "config path contains an internal NUL byte".into(),
                    Some(err.into_kconfq_error()),
                )
            },
        },

        Ok(None) => unsafe {
            *out_path = ptr::null_mut();
            *out_err = make_kconfq_error(
                KconfqErrorKind::KCONFQ_ERROR_KIND_NOT_FOUND,
                "failed to find kernel config location".into(),
                None,
            )
        },

        Err(crate::error::LocateConfigFileError::ErrorGettingLinuxKernelVersion(err)) => unsafe {
            *out_path = ptr::null_mut();
            *out_err = make_kconfq_error(
                KconfqErrorKind::KCONFQ_ERROR_KIND_NOT_FOUND,
                "failed to find kernel config location".into(),
                Some(err.into_kconfq_error()),
            )
        },
    }
}

/// Find line in the config that contains specified `entry_name`.
///
/// # Ownership
///
/// - `out_line` must be freed by the caller by using [`kconfq_free_string`].
/// - `out_err` must be freed by the caller by using [`kconfq_free_error`].
///
/// # Parameters
///
/// - `entry_name` - Pointer to a null-terminated C string specifying the name
///   of the entry to search for. Must not be `NULL`.
/// - `out_line` - Pointer to a location that will receive the allocated string
///   on success. Must not be `NULL`.
/// - `out_err` - Pointer to an error that happened during execution. Must not
///   be `NULL`.
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_line(
    entry_name: *const c_char,
    out_line: *mut *mut c_char,
    out_err: *mut *mut KconfqError,
) {
    assert!(!entry_name.is_null());
    assert!(!out_line.is_null());
    assert!(!out_err.is_null());

    let entry_name = unsafe { CStr::from_ptr(entry_name) };
    let _entry_name = match entry_name.to_str() {
        Ok(str) => str,
        Err(_) => {
            unsafe {
                *out_line = ptr::null_mut();
                *out_err = make_kconfq_error(
                    KconfqErrorKind::KCONFQ_ERROR_KIND_INVALID_UTF8,
                    "entry_name is not a valid UTF-8 string".into(),
                    None,
                );
                return;
            };
        }
    };

    // let a = crate::find_line(entry_name)
}

//   ⠀⠀⠀⠀⢠⡶⠚⢷⣤⡀⠀⠀⠀⠀⠀⣲⡶⠛⠻⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//   ⠀⠀⠀⢠⡿⠁⠀⠀⠙⣷⣄⠀⢀⣴⡟⠁⠀⠀⢷⢹⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀
//   ⠀⠀⠀⣾⠃⠀⠠⠶⠚⠛⠛⠛⠛⠋⠀⠀⣀⡀⢸⠈⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀
//   ⠀⠀⢸⣏⡔⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠚⠉⠉⣿⠀⢹⠀⠀⠀⠀⠀⠀⠀⠀⠀
//   ⠀⠀⢾⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀
//   ⠀⢠⣿⢠⣶⡆⠀⠀⠀⠀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀
//   ⢒⡾⠁⠘⠟⠁⠀⠀⠀⠀⣿⣿⡆⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀
//   ⠉⣧⠀⠀⠀⠀⠃⠀⠀⠀⠈⠉⠠⣍⠀⠀⠀⠀⠀⠀⣸⡇⢀⣤⠶⠛⠛⠻⢦⣄
//   ⠀⠸⣧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⡟⣴⠟⠁⠀⠀⠀⠀⠀⢻
//   ⠀⠀⠀⠛⣷⡦⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣤⡴⠞⠋⢠⡟⠀⠀⠀⠀⠀⠀⢀⡾
//   ⠀⠀⠀⢰⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠳⣤⡀⢸⠃⠀⠀⠀⠀⢠⡶⠟⠁
//   ⠀⠀⠀⣸⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢷⣹⡄⠀⠀⠀⠀⣼⠀⠀⠀
//   ⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢿⣇⠀⠀⠀⠀⢹⡄⠀⠀
//   ⠀⠀⠀⢸⡀⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⡄⠀⠀⠀⠈⣧⠀⠀
//   ⠀⠀⠀⢸⡇⠘⡇⠀⠀⠀⠀⠀⠀⠀⣀⠀⠀⠀⠀⠀⠀⢸⣿⠀⠀⠀⠀⢹⡇⠀
//   ⠀⠀⠀⢸⡇⠀⠙⠀⠀⠀⠀⠀⢠⠞⠁⠀⠀⠀⠀⠀⠀⠀⣿⠇⠀⠀⠀⢸⡇⠀
//   ⠀⠀⠀⢸⡇⠀⢸⡆⠀⠀⠀⠀⣟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠀⠀⠀⠀⣸⠇⠀
//   ⠀⠀⠀⢸⣿⠀⠀⡇⠀⠀⠀⠀⣿⡀⠀⠀⠀⠀⠀⠀⠀⢀⡇⠀⠀⢀⣴⡟⠁⠀
//   ⠀⠀⠀⠘⠿⠶⢶⢧⣦⣦⡴⢾⣥⣽⣤⣤⣤⣤⣤⣤⡴⣯⡤⠴⠶⠛⠋⠀⠀⠀
