// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

#![crate_name = "sample"]
#![crate_type = "staticlib"]
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]
#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate wasmi;
extern crate wasmi_impl;

use sgx_types::*;
use std::io::{self, Write};
use std::slice;

/// # Safety
/// The caller needs to ensure that `binary` is a valid pointer to a slice valid for `binary_len` items
/// and that `result_out` is a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn exec_wasm_median_int(
    data_in: *const u8,
    data_len: usize,
    binary: *const u8,
    binary_len: usize,
    result_out: *mut i32,
) -> sgx_status_t {
    if binary.is_null() {
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER;
    }
    // Safety: SGX generated code will check that the pointer is valid.
    let binary_slice = unsafe { slice::from_raw_parts(binary, binary_len) };
    let data = unsafe { slice::from_raw_parts(data_in, data_len) };

    unsafe {
        *result_out = match wasmi_impl::exec_wasm_with_data(binary_slice, data) {
            Ok(Some(wasmi::RuntimeValue::I32(ret))) => ret,
            Ok(_) | Err(_) => return sgx_status_t::SGX_ERROR_UNEXPECTED,
        }
    };
    sgx_status_t::SGX_SUCCESS
}

/// # Safety
/// The caller needs to ensure that `binary` is a valid pointer to a slice valid for `binary_len` items
/// and that `result_out` is a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn exec_wasm_median_float(
    data_in: *const u8,
    data_len: usize,
    binary: *const u8,
    binary_len: usize,
    result_out: *mut f32,
) -> sgx_status_t {
    if binary.is_null() {
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER;
    }
    // Safety: SGX generated code will check that the pointer is valid.
    let binary_slice = unsafe { slice::from_raw_parts(binary, binary_len) };
    let data = unsafe { slice::from_raw_parts(data_in, data_len) };

    unsafe {
        *result_out = match wasmi_impl::exec_wasm_with_data(binary_slice, data) {
            Ok(Some(wasmi::RuntimeValue::F32(ret))) => ret.to_float(),
            Ok(_) | Err(_) => return sgx_status_t::SGX_ERROR_UNEXPECTED,
        }
    };
    sgx_status_t::SGX_SUCCESS
}

/// # Safety
/// The caller needs to ensure that `binary` is a valid pointer to a slice valid for `binary_len` items
/// and that `result_out` is a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn exec_wasm_mean_int(
    data_in: *const u8,
    data_len: usize,
    binary: *const u8,
    binary_len: usize,
    result_out: *mut i32,
) -> sgx_status_t {
    if binary.is_null() {
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER;
    }
    // Safety: SGX generated code will check that the pointer is valid.
    let binary_slice = unsafe { slice::from_raw_parts(binary, binary_len) };
    let data = unsafe { slice::from_raw_parts(data_in, data_len) };

    unsafe {
        *result_out = match wasmi_impl::exec_wasm_with_data(binary_slice, data) {
            Ok(Some(wasmi::RuntimeValue::I32(ret))) => ret,
            Ok(_) | Err(_) => return sgx_status_t::SGX_ERROR_UNEXPECTED,
        }
    };
    sgx_status_t::SGX_SUCCESS
}

/// # Safety
/// The caller needs to ensure that `binary` is a valid pointer to a slice valid for `binary_len` items
/// and that `result_out` is a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn exec_wasm_mean_float(
    data_in: *const u8,
    data_len: usize,
    binary: *const u8,
    binary_len: usize,
    result_out: *mut f32,
) -> sgx_status_t {
    if binary.is_null() {
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER;
    }
    // Safety: SGX generated code will check that the pointer is valid.
    let binary_slice = unsafe { slice::from_raw_parts(binary, binary_len) };
    let data = unsafe { slice::from_raw_parts(data_in, data_len) };

    unsafe {
        *result_out = match wasmi_impl::exec_wasm_with_data(binary_slice, data) {
            Ok(Some(wasmi::RuntimeValue::F32(ret))) => ret.to_float(),
            Ok(_) | Err(_) => return sgx_status_t::SGX_ERROR_UNEXPECTED,
        }
    };
    sgx_status_t::SGX_SUCCESS
}

/// # Safety
/// The caller needs to ensure that `binary` is a valid pointer to a slice valid for `binary_len` items
/// and that `result_out` is a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn exec_wasm_sd_int(
    binary: *const u8,
    binary_len: usize,
    result_out: *mut f32,
) -> sgx_status_t {
    if binary.is_null() {
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER;
    }
    // Safety: SGX generated code will check that the pointer is valid.
    let binary_slice = unsafe { slice::from_raw_parts(binary, binary_len) };
    let data = b"[1,2,3,4,5]";
    unsafe {
        *result_out = match wasmi_impl::exec_wasm_with_data(binary_slice, data) {
            Ok(Some(wasmi::RuntimeValue::F32(ret))) => ret.to_float(),
            Ok(_) | Err(_) => return sgx_status_t::SGX_ERROR_UNEXPECTED,
        }
    };
    sgx_status_t::SGX_SUCCESS
}

/// # Safety
/// The caller needs to ensure that `binary` is a valid pointer to a slice valid for `binary_len` items
/// and that `result_out` is a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn exec_wasm_sd_float(
    binary: *const u8,
    binary_len: usize,
    result_out: *mut f32,
) -> sgx_status_t {
    if binary.is_null() {
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER;
    }
    // Safety: SGX generated code will check that the pointer is valid.
    let binary_slice = unsafe { slice::from_raw_parts(binary, binary_len) };
    let data = b"[1,2,3,4,5]";
    unsafe {
        *result_out = match wasmi_impl::exec_wasm_with_data(binary_slice, data) {
            Ok(Some(wasmi::RuntimeValue::F32(ret))) => ret.to_float(),
            Ok(_) | Err(_) => return sgx_status_t::SGX_ERROR_UNEXPECTED,
        }
    };
    sgx_status_t::SGX_SUCCESS
}
