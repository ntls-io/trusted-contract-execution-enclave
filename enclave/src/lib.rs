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
#[macro_use]
extern crate heapless;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json_core;

use sgx_types::*;
use std::io::{self, Write};
use std::slice;
use heapless::String;
use heapless::consts::*;

mod structs;

// Import the structs for easier reference
use structs::{
    Transaction,
    IncomingAssetTransaction,
    IncomingPaymentTransaction,};


/// # Safety
/// The caller needs to ensure that `binary` is a valid pointer to a slice valid for `binary_len` items
/// and that `result_out` is a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn asset_transaction_check(
    data_bytes: *const u8,
    data_bytes_len: usize,
    result_out: *mut i32,
) -> sgx_status_t {
     // Safety: SGX generated code will check that the pointer is valid.
    if data_bytes.is_null() {
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER;
    }
   
    let data_bytes_slice = unsafe { slice::from_raw_parts(data_bytes, data_bytes_len) };
    let deserialized_data: IncomingAssetTransaction = serde_json_core::from_slice(data_bytes_slice).expect("Failed to deserialize");


    println!("deserialised data {:?}", deserialized_data);
    sgx_status_t::SGX_SUCCESS
}
