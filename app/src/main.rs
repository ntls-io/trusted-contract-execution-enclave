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

extern crate sgx_types;
extern crate sgx_urts;
extern crate wabt;
use serde_json::Error;
use serde_json::Error;
use sgx_types::*;
use sgx_urts::SgxEnclave;

extern crate serde_json;
use std::path::Path;
use std::{
    fs::{self, File},
    io::Read,
};

static WASM_FILE_MEDIAN_INT: &str = "get_median_int.wasm";
static WASM_FILE_MEDIAN_FLOAT: &str = "get_median_float.wasm";

static WASM_FILE_MEAN_INT: &str = "get_mean_int.wasm";
static WASM_FILE_MEAN_FLOAT: &str = "get_mean_float.wasm";

static WASM_FILE_SD_INT: &str = "get_sd_int.wasm";
static WASM_FILE_SD_FLOAT: &str = "get_sd_float.wasm";

static ENCLAVE_FILE: &str = "enclave.signed.so";

extern "C" {

    fn exec_wasm_median_int(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        data_in: *const u8,
        data_len: usize,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut i32,
    ) -> sgx_status_t;

    fn exec_wasm_median_float(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        data_in: *const u8,
        data_len: usize,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut f32,
    ) -> sgx_status_t;

    fn exec_wasm_mean_int(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        data_in: *const u8,
        data_len: usize,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut i32,
    ) -> sgx_status_t;

    fn exec_wasm_mean_float(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        data_in: *const u8,
        data_len: usize,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut f32,
    ) -> sgx_status_t;

    fn exec_wasm_sd_int(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut f32,
    ) -> sgx_status_t;

    fn exec_wasm_sd_float(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        binary: *const u8,
        binary_len: usize,
        result_out: *mut f32,
    ) -> sgx_status_t;

}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };
    SgxEnclave::create(
        ENCLAVE_FILE,
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    )
}

/// Read data from the JSON file and parse it into a vector of ints.
fn read_data_from_json_float(file_path: &str, array_name: &str) -> Result<Vec<f32>, Error> {
    let path = Path::new(file_path);
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json_data: serde_json::Value = serde_json::from_str(&data)?;

    let array_data = json_data[array_name].as_array().unwrap();

    let data_vec: Vec<f32> = array_data
        .iter()
        .map(|v| v.as_f64().unwrap() as f32)
        .collect();

    Ok(data_vec)
}

/// Read data from the JSON file and parse it into a vector of ints.
fn read_data_from_json_int(file_path: &str, array_name: &str) -> Result<Vec<i32>, Error> {
    let path = Path::new(file_path);
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json_data: serde_json::Value = serde_json::from_str(&data)?;

    let array_data = json_data[array_name].as_array().unwrap();

    let data_vec: Vec<i32> = array_data
        .iter()
        .map(|v| v.as_i64().unwrap() as i32)
        .collect();

    Ok(data_vec)
}

fn main() {
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        }
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        }
    };

    let mut retval = sgx_status_t::SGX_SUCCESS;

    let binary_median_int = fs::read(WASM_FILE_MEDIAN_INT).unwrap();
    let binary_median_float = fs::read(WASM_FILE_MEDIAN_FLOAT).unwrap();

    //// Mean test
    // Float
    let data_mean_float = read_data_from_json_float(
        "/root/workspace/wasm-exec-sgx/get-mean-float-wasm/test_data.json",
        "mean_float_works",
    )
    .unwrap();
    let serialized_data_mean_float: Vec<u8> = serde_json::to_vec(&data_mean_float).unwrap(); // Create a new byte array that holds the serialized JSON data
    let binary_mean_float = fs::read(WASM_FILE_MEAN_FLOAT).unwrap();
    let mut result_out_mean_float = 0f32;

    // Int
    let data_mean_int = read_data_from_json_int(
        "/root/workspace/wasm-exec-sgx/get-mean-int-wasm/test_data.json",
        "mean_int_works",
    )
    .unwrap();
    let serialized_data_mean_int: Vec<u8> = serde_json::to_vec(&data_mean_int).unwrap(); // Create a new byte array that holds the serialized JSON data
    let binary_mean_int = fs::read(WASM_FILE_MEAN_INT).unwrap();
    let mut result_out_mean_int = 0i32;

    // Sd test
    let binary_sd_int = fs::read(WASM_FILE_SD_INT).unwrap();
    let binary_sd_float = fs::read(WASM_FILE_SD_FLOAT).unwrap();
    let mut result_out_sd_int = 0f32;
    let mut result_out_sd_float = 0f32;

    let result = unsafe {
        exec_wasm_median_int(
            enclave.geteid(),
            &mut retval,
            serialized_data_median_int.as_ptr(),
            serialized_data_median_int.len(),
            binary_median_int.as_ptr(),
            binary_median_int.len(),
            &mut result_out_median_int,
        );

        exec_wasm_median_float(
            enclave.geteid(),
            &mut retval,
            serialized_data_median_float.as_ptr(),
            serialized_data_median_float.len(),
            binary_median_float.as_ptr(),
            binary_median_float.len(),
            &mut result_out_median_float,
        );

        exec_wasm_mean_int(
            enclave.geteid(),
            &mut retval,
            serialized_data_mean_int.as_ptr(),
            serialized_data_mean_int.len(),
            binary_mean_int.as_ptr(),
            binary_mean_int.len(),
            &mut result_out_mean_int,
        );

        exec_wasm_mean_float(
            enclave.geteid(),
            &mut retval,
            serialized_data_mean_float.as_ptr(),
            serialized_data_mean_float.len(),
            binary_mean_float.as_ptr(),
            binary_mean_float.len(),
            &mut result_out_mean_float,
        );

        exec_wasm_sd_int(
            enclave.geteid(),
            &mut retval,
            binary_sd_int.as_ptr(),
            binary_sd_int.len(),
            &mut result_out_sd_int,
        );

        exec_wasm_sd_float(
            enclave.geteid(),
            &mut retval,
            binary_sd_float.as_ptr(),
            binary_sd_float.len(),
            &mut result_out_sd_float,
        )
    };

    match result {
        sgx_status_t::SGX_SUCCESS => {}
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    println!(
        "[+] ecall_test success, Median Int result -  {:?}",
        result_out_median_int
    );
    println!(
        "[+] ecall_test success, Median Float result -  {:?}",
        result_out_median_float
    );
    println!();
    println!(
        "[+] ecall_test success, Mean Int result -  {:?}",
        result_out_mean_int
    );
    println!(
        "[+] ecall_test success, Mean Float result -  {:?}",
        result_out_mean_float
    );
    println!();
    println!(
        "[+] ecall_test success, SD Int result -  {:?}",
        result_out_sd_int
    );
    println!(
        "[+] ecall_test success, SD Float result -  {:?}",
        result_out_sd_float
    );

    enclave.destroy();
}
