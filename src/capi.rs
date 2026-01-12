// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

//! C-API that will be compiled to `cdynlib` to be used from C.

#![allow(clippy::missing_safety_doc)]

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

/// Status of the [`kconfq_locate_config`] function.
#[repr(C)]
pub enum LocateConfigStatus {
    Success = 0,
    NotFound = 1,
    ErrorGettingKernelVersion = 2,
    UnknownError = 255,
}

/// Search through all known config locations and return a path to it. May not
/// find a config.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kconfq_locate_config(out_path: *mut *mut c_char) -> LocateConfigStatus {
    if out_path.is_null() {
        return LocateConfigStatus::UnknownError;
    }

    match crate::locate_config() {
        Ok(Some(config)) => match CString::new(config.path.to_string_lossy().as_bytes()) {
            Ok(c_string) => {
                unsafe {
                    *out_path = c_string.into_raw();
                }
                LocateConfigStatus::Success
            }
            Err(_) => LocateConfigStatus::UnknownError,
        },

        Ok(None) => {
            unsafe {
                *out_path = ptr::null_mut();
            }
            LocateConfigStatus::NotFound
        }

        Err(crate::error::LocateConfigFileError::ErrorGettingLinuxKernelVersion(_)) => {
            unsafe {
                *out_path = ptr::null_mut();
            }
            LocateConfigStatus::ErrorGettingKernelVersion
        }
    }
}
