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
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;  // Include this line
extern crate serde;
extern crate serde_json;

use rocket_contrib::json::Json;
use rocket::State;
extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::sync::{Arc, Mutex};
use std::sync::MutexGuard;
use serde_json::Error;

mod structs;

// Import the structs for easier reference
use structs::{
    IncomingAssetTransaction,
    IncomingPaymentTransaction,
    ReturnDVPTransactions,
    ErrorResponse
};

static ENCLAVE_FILE: &str = "enclave.signed.so";

// Ecalls to enclave
extern "C" {
    fn asset_transaction_check(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        data_bytes: *const u8,
        data_bytes_len: usize,
        result_out: *mut u8
    ) -> sgx_status_t;
    
    fn payment_transaction_check(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        data_bytes: *const u8,
        data_bytes_len: usize,
        result_out: *mut u8
    ) -> sgx_status_t;
}

// initatialise enclave
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

fn get_enclave_instance<'a>(enclave: &'a State<Arc<Mutex<Option<SgxEnclave>>>>) -> MutexGuard<'a, Option<SgxEnclave>> {
    let enclave_guard = enclave.lock().unwrap();
    if enclave_guard.is_none() {
        panic!("Enclave is unexpectedly None!");
    }
    enclave_guard
}

fn execute_asset_transaction(enclave_instance: &SgxEnclave, data: &Json<IncomingAssetTransaction>) -> [u8; 1024] {
    let data_bytes = serde_json::to_vec(&data.0).expect("Failed to convert to bytes");
    let mut retval = sgx_status_t::SGX_SUCCESS;
    let mut result_out: [u8; 1024] = [0; 1024];
    
    unsafe {
        asset_transaction_check(
            enclave_instance.geteid(),
            &mut retval,
            data_bytes.as_ptr(),
            data_bytes.len(),
            result_out.as_mut_ptr(),
        );
    }
    
    result_out
}

fn execute_payment_transaction(enclave_instance: &SgxEnclave, data: &Json<IncomingPaymentTransaction>) -> [u8; 1024] {
    let data_bytes = serde_json::to_vec(&data.0).expect("Failed to convert to bytes");
    let mut retval = sgx_status_t::SGX_SUCCESS;
    let mut result_out: [u8; 1024] = [0; 1024];
    
    unsafe {
        payment_transaction_check(
            enclave_instance.geteid(),
            &mut retval,
            data_bytes.as_ptr(),
            data_bytes.len(),
            result_out.as_mut_ptr(),
        );
    }
    
    result_out
}


#[post("/check-asset-transaction", format = "json", data = "<data>")]
fn receive_asset(enclave: State<Arc<Mutex<Option<SgxEnclave>>>>, data: Json<IncomingAssetTransaction>) -> Result<Json<ReturnDVPTransactions>, Json<ErrorResponse>> {
    
    let enclave_guard = get_enclave_instance(&enclave);
    let result_out = execute_asset_transaction(enclave_guard.as_ref().unwrap(), &data);

    let end_of_data = result_out.iter().position(|&x| x == 0).unwrap_or(result_out.len());
    let meaningful_data = &result_out[0..end_of_data];

    let deserialized: Result<ReturnDVPTransactions, Error> = serde_json::from_slice(meaningful_data);
    println!("Deserialized data: {:?}", deserialized);

    match deserialized {
        Ok(value) => Ok(Json(value)),
        Err(_) => {
            let error_response = ErrorResponse {
                error: "Error in deserializing the returned data from the enclave.".into(),
            };
            Err(Json(error_response))
        }
    }
}

#[post("/check-payment-transaction", format = "json", data = "<data>")]
fn receive_payment(enclave: State<Arc<Mutex<Option<SgxEnclave>>>>,data: Json<IncomingPaymentTransaction>) -> Result<Json<ReturnDVPTransactions>, Json<ErrorResponse>> {
    
    let enclave_guard = get_enclave_instance(&enclave);
    let result_out = execute_payment_transaction(enclave_guard.as_ref().unwrap(), &data);

    let end_of_data = result_out.iter().position(|&x| x == 0).unwrap_or(result_out.len());
    let meaningful_data = &result_out[0..end_of_data];

    let deserialized: Result<ReturnDVPTransactions, Error> = serde_json::from_slice(meaningful_data);
    println!("Deserialized data: {:?}", deserialized);

    match deserialized {
        Ok(value) => Ok(Json(value)),
        Err(_) => {
            let error_response = ErrorResponse {
                error: "Error in deserializing the returned data from the enclave.".into(),
            };
            Err(Json(error_response))
        }
    }
}

// main function
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

    let enclave_arc = Arc::new(Mutex::new(Some(enclave)));

   rocket::ignite()
        .manage(enclave_arc.clone()) // Add the enclave instance as a managed state in Rocket
        .mount("/", routes![receive_asset, receive_payment])
        .launch();
    

    if let Some(enclave) = enclave_arc.lock().unwrap().take() {
    enclave.destroy();
};

}
