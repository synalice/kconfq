// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

//! C-API that will be compiled to `cdynlib` to be used from C.
//!
//! # Important
//!
//! This module is basically a C code written in Rust. All of this is extremely
//! unsafe and should be audited very carefully.

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

/// Status of the [`kconfq_locate_config`] function.
#[repr(C)]
pub enum KconfqResult {
    /// Success.
    KCONFQ_RESULT_SUCCESS = 0,
    /// Not found.
    KCONFQ_RESULT_NOT_FOUND = 1,
    /// Error getting linux kernel version.
    KCONFQ_RESULT_KERNEL_VERSION_ERROR = 2,
    /// NULL pointer passed as parameter.
    KCONFQ_RESULT_NULL_PARAMETER = 3,
    /// Input to a fucntion is malformed.
    KCONFQ_RESULT_MALFORMED_INPUT = 4,
    /// Unknown internal error.
    KCONFQ_RESULT_UNKNOWN_ERROR = 255,
}

/// Returns a human-readable string, describing a [`KconfqResult`].
///
/// # Parameters
///
/// * `result` - A valid [`KconfqResult`] value returned by
///   [`kconfq_locate_config`].
///
/// # Returns
///
/// A pointer to a null-terminated, static string describing the status.
///
/// The returned pointer:
///
/// - Has static lifetime
/// - Must NOT be freed or modified by the caller
/// - Is valid for the duration of the program
///
/// # Safety
///
/// This function assumes `result` is a valid member of the
/// [`KconfqResult`] enum. Passing any other arbitrary integer results in
/// an undefined behavior.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_result_strerror(result: KconfqResult) -> *const c_char {
    // SAFETY: all strings are static and null-terminated
    match result {
        KconfqResult::KCONFQ_RESULT_SUCCESS => cstr!("success"),
        KconfqResult::KCONFQ_RESULT_NOT_FOUND => cstr!("not found"),
        KconfqResult::KCONFQ_RESULT_KERNEL_VERSION_ERROR => {
            cstr!("error getting Linux kernel version")
        }
        KconfqResult::KCONFQ_RESULT_NULL_PARAMETER => cstr!("parameter is a NULL pointer"),
        KconfqResult::KCONFQ_RESULT_UNKNOWN_ERROR => cstr!("unknown internal error"),
        KconfqResult::KCONFQ_RESULT_MALFORMED_INPUT => cstr!("input to a fucntion is malformed"),
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

/// Locate the kernel config file and return its path.
///
/// On success, this function allocates a null-terminated C string containing
/// the path to the configuration file and stores a pointer to immutable
/// characters in `*out_path`.
///
/// # Ownership
///
/// Ownership of the returned string is transferred to the caller, who must free
/// it using [`kconfq_free_string`].
///
/// # Parameters
///
/// - `out_path` - Pointer to a location that will receive the allocated string
///   on success.
///
/// # Return value
///
/// Returns a [`KconfqResult`] indicating the result of the operation:
///
/// - [`KconfqResult::KCONFQ_RESULT_SUCCESS`] - The configuration file was found
///   and `*out_path` is set to a newly allocated string.
/// - [`KconfqResult::KCONFQ_RESULT_NOT_FOUND`] - No configuration file
///   was found. `*out_path` is set to `NULL`.
/// - [`KconfqResult::KCONFQ_RESULT_KERNEL_VERSION_ERROR`] - Failed to determine
///   the running kernel version. `*out_path` is set to `NULL`.
/// - [`KconfqResult::KCONFQ_RESULT_NULL_PARAMETER`] - `out_path` itself was
///   `NULL`.
/// - [`KconfqResult::KCONFQ_RESULT_UNKNOWN_ERROR`] - An unexpected internal
///   error occurred.
///
/// # Safety
///
/// The caller must not assume `*out_path` is initialized unless the return
/// value is `Success`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_locate_config(out_path: *mut *const c_char) -> KconfqResult {
    if out_path.is_null() {
        return KconfqResult::KCONFQ_RESULT_NULL_PARAMETER;
    }

    match crate::locate_config() {
        Ok(Some(config)) => match CString::new(config.path.to_string_lossy().as_bytes()) {
            Ok(c_string) => unsafe {
                *out_path = c_string.into_raw();
                KconfqResult::KCONFQ_RESULT_SUCCESS
            },
            Err(_) => unsafe {
                *out_path = ptr::null_mut();
                KconfqResult::KCONFQ_RESULT_UNKNOWN_ERROR
            },
        },

        Ok(None) => unsafe {
            *out_path = ptr::null_mut();
            KconfqResult::KCONFQ_RESULT_NOT_FOUND
        },

        Err(crate::error::LocateConfigFileError::GettingLinuxKernelVersion(_)) => unsafe {
            *out_path = ptr::null_mut();
            KconfqResult::KCONFQ_RESULT_KERNEL_VERSION_ERROR
        },
    }
}

/// Find line in the config that contains specified `entry_name`.
///
/// # Ownership
///
/// Ownership of the returned string is transferred to the caller, who must free
/// it using [`kconfq_free_string`].
///
/// # Parameters
///
/// - `entry_name` - Pointer to a null-terminated C string specifying the name
///   of the entry to search for. The pointer must not be `NULL`.
/// - `out_line` - Pointer to a location that will receive the allocated string
///   on success.
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
pub unsafe extern "C" fn kconfq_find_line(
    entry_name: *const c_char,
    out_line: *mut *const c_char,
) -> KconfqResult {
    if entry_name.is_null() {
        return KconfqResult::KCONFQ_RESULT_NULL_PARAMETER;
    }

    let entry_name = unsafe { CStr::from_ptr(entry_name) };
    let _entry_name = match entry_name.to_str() {
        Ok(str) => str,
        Err(_) => {
            unsafe {
                *out_line = ptr::null_mut();
                return KconfqResult::KCONFQ_RESULT_MALFORMED_INPUT;
            };
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
