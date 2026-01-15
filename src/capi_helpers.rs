// SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::ffi::{CString, c_char};
use std::ptr;

use crate::capi::{KconfqError, KconfqErrorKind};

pub fn boxed_cstring(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

pub fn make_kconfq_error(
    kind: KconfqErrorKind,
    msg: String,
    cause: Option<*mut KconfqError>,
) -> *mut KconfqError {
    Box::into_raw(Box::new(KconfqError {
        kind,
        message: boxed_cstring(msg),
        cause: cause.unwrap_or(ptr::null_mut()),
    }))
}
