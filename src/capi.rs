// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

//! C-API that will be compiled to `cdynlib` to be used from C.
//!
//! # Warning
//!
//! This module is basically a C code written in Rust. All of this is **extremely
//! unsafe** and should be approached very carefully.

#![allow(clippy::missing_safety_doc)]
#![allow(non_camel_case_types)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// Create compile-time null-terminated C string.
macro_rules! cstr {
    ($s:literal) => {{
        const BYTES: &[u8] = concat!($s, "\0").as_bytes();
        // Safety: `concat!($s, "\0")` ensures null termination and no interior nulls.
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(BYTES).as_ptr() }
    }};
}

/// Generate enum + FFI-safe strerror function + doc comments
macro_rules! kconfq_results {
    (
        $(
            $(#[$meta:meta])*
            $name:ident = $val:expr => $msg:literal
        ),+ $(,)?
    ) => {
        /// Result of the function's operation.
        #[repr(C)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum KconfqResult {
            $(
                $(#[$meta])*
                $name = $val,
            )+
        }

        /// Returns a human-readable string, describing a [`KconfqResult`].
        ///
        /// # Returns
        ///
        /// A pointer to a null-terminated, static string describing the status.
        /// Must NOT be freed or modified by the caller.
        ///
        /// # Safety
        ///
        /// This function assumes `result` is a valid member of the
        /// [`KconfqResult`] enum. Passing any other arbitrary integer results in
        /// an undefined behavior.
        #[unsafe(no_mangle)]
        pub extern "C" fn kconfq_result_strerror(result: KconfqResult) -> *const std::os::raw::c_char {
            match result {
                $(KconfqResult::$name => cstr!($msg),)+
            }
        }
    };
}

kconfq_results! {
    /// Success.
    KCONFQ_RESULT_SUCCESS = 0 => "success",
    /// Not found.
    KCONFQ_RESULT_NOT_FOUND = 1 => "not found",
    /// Error getting Linux kernel version.
    KCONFQ_RESULT_KERNEL_VERSION_ERROR = 2 => "error getting Linux kernel version",
    /// Parameter is a NULL pointer.
    KCONFQ_RESULT_NULL_PARAMETER = 3 => "parameter is a NULL pointer",
    /// Value of the function's argument is malformed.
    KCONFQ_RESULT_MALFORMED_ARGUMENT_VALUE = 4 => "value of the function's argument is malformed",
    /// Path to config is not a valid UTF-8.
    KCONFQ_RESULT_NON_UTF8_PATH_TO_CONFIG = 5 => "path to config is not a valid UTF-8",
    /// Unknown internal error.
    KCONFQ_RESULT_UNKNOWN_ERROR = 255 => "unknown internal error",
}

/// Frees a string allocated by this library.
///
/// # Safety
///
/// `ptr` must be a pointer previously returned by this library, or `NULL`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_free_string(ptr: *const c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr as *mut i8));
        }
    }
}

/// Locate the kernel config file and return path to it.
///
/// # Parameters
///
/// - `out_path` - Pointer to a location that will receive the allocated
///   constant null-terminated string on success. Caller must free it using
///   [`kconfq_free_string`]. Must NOT be `NULL`.
///
/// # Return values
///
/// - [`KconfqResult::KCONFQ_RESULT_SUCCESS`] - Configuration file was found and
///   `*out_path` was set to a newly allocated string.
/// - [`KconfqResult::KCONFQ_RESULT_NOT_FOUND`] - No configuration file was
///   found at any possible known location. `*out_path` was set to `NULL`.
/// - [`KconfqResult::KCONFQ_RESULT_KERNEL_VERSION_ERROR`] - Failed to determine
///   the running kernel version. `*out_path` was set to `NULL`.
/// - [`KconfqResult::KCONFQ_RESULT_NULL_PARAMETER`] - `out_path` itself was
///   `NULL`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_locate_config(out_path: *mut *const c_char) -> KconfqResult {
    if out_path.is_null() {
        return KconfqResult::KCONFQ_RESULT_NULL_PARAMETER;
    }

    unsafe {
        *out_path = ptr::null_mut();
    }

    match crate::locate_config() {
        Ok(Some(config)) => match CString::new(config.path.to_string_lossy().as_bytes()) {
            Ok(c_string) => unsafe {
                *out_path = c_string.into_raw();
                KconfqResult::KCONFQ_RESULT_SUCCESS
            },
            Err(_) => KconfqResult::KCONFQ_RESULT_NON_UTF8_PATH_TO_CONFIG,
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
/// - `entry_name` - Pointer to a null-terminated C string specifying the name
///   of the entry to search for. The pointer must not be `NULL`.
/// - `out_line` - Pointer to a location that will receive the allocated
///   constant null-terminated string on success. Caller must free it using
///   [`kconfq_free_string`]. Must NOT be `NULL`.
///
/// # Return value examples
///
/// - `CONFIG_FOO=y`
/// - `CONFIG_FOO=m`
/// - `CONFIG_FOO=12345`
/// - `CONFIG_FOO="something something"`
/// - `# CONFIG_FOO is not set`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_find_line(
    entry_name: *const c_char,
    out_line: *mut *const c_char,
) -> KconfqResult {
    if entry_name.is_null() {
        return KconfqResult::KCONFQ_RESULT_NULL_PARAMETER;
    }

    unsafe {
        *out_line = ptr::null_mut();
    }

    let entry_name = unsafe { CStr::from_ptr(entry_name) };
    let _entry_name = match entry_name.to_str() {
        Ok(str) => str,
        Err(_) => {
            return KconfqResult::KCONFQ_RESULT_MALFORMED_ARGUMENT_VALUE;
        }
    };

    // let a = crate::find_line(entry_name)

    KconfqResult::KCONFQ_RESULT_SUCCESS
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
