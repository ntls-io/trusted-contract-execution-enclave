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

use rocket_contrib::json::Json;
use rocket::State;
extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::fs;
use std::sync::{Arc, Mutex};

mod structs;

// Import the structs for easier reference
use structs::{
    IncomingAssetTransaction,
    IncomingPaymentTransaction,
    TransactionType,
    DVPTransaction,
    ReturnEnclaveTransactions,
    ErrorResponse,
    SuccessResponse,
    ResponseType
};


static ENCLAVE_FILE: &str = "enclave.signed.so";

// Ecalls to enclave
extern "C" {
    fn asset_transaction_check(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        data_bytes: *const u8,
        data_bytes: usize,
        result_out: *mut i32,
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


#[post("/check-asset-transaction", format = "json", data = "<data>")]
fn receive_asset(enclave: State<Arc<Mutex<Option<SgxEnclave>>>>,data: Json<IncomingAssetTransaction>) -> Result<Json<ResponseType>, Json<ErrorResponse>> {
    println!("Received: {:?}", data.0);
    let enclave_guard = enclave.lock().unwrap();
    
    let enclave_instance = if let Some(ref enclave) = *enclave_guard {
        enclave
    } else {
        // Handle the case when the enclave is None (shouldn't happen in normal cases)
        panic!("Enclave is unexpectedly None!");
    };

    let data_bytes = serde_json::to_vec(&data.0).expect("Failed to convert to bytes");

    let mut retval = sgx_status_t::SGX_SUCCESS;

    let mut result_out_median_int = 0i32;
    let mut result_out_median_float = 0f32;

    let result = unsafe {
        asset_transaction_check(
            enclave_instance.geteid(),
            &mut retval,
            data_bytes.as_ptr(),
            data_bytes.len(),
            &mut result_out_median_int,
        );
    };

    println!("Succesffuly ecalls: {:?}", result_out_median_float);

    let enclave_message = "successfullmatch";  // dummy value for now, but this should come from your actual logic

    if enclave_message == "successfullmatch" {
        let transaction_1 = DVPTransaction {
            transaction_type: TransactionType::TokenTransfer,
            sender: "escrowaddress1234".into(),
            recipient: "billaddress1234".into(),
            token_id: "1".into(),
            amount: 50,
        };

        let transaction_2 = DVPTransaction {
            transaction_type: TransactionType::Payment,
            sender: "escrowaddress1234".into(),
            recipient: "alexaddress1234".into(),
            token_id: "2".into(),
            amount: 1,
        };

        let response = ReturnEnclaveTransactions {
            transaction_1,
            transaction_2,
        };

        Ok(Json(ResponseType::Transactions(response)))
    } else if enclave_message == "successfullyadded" {
        let success_response = SuccessResponse {
            message: "Successfully added new DVp transaction.".into(),
        };

        Ok(Json(ResponseType::Message(success_response)))
    } else {
        let error_response = ErrorResponse {
            error: "Invalid message content.".into(),
        };

        Err(Json(error_response))
    }
}

#[post("/check-payment-transaction", format = "json", data = "<data>")]
fn receive_payment(enclave: State<Arc<Mutex<Option<SgxEnclave>>>>,data: Json<IncomingPaymentTransaction>) -> Result<Json<ResponseType>, Json<ErrorResponse>> {
    println!("Received: {:?}", data.0);
        let enclave_guard = enclave.lock().unwrap();
    
    let enclave_instance = if let Some(ref enclave) = *enclave_guard {
        enclave
    } else {
        // Handle the case when the enclave is None (shouldn't happen in normal cases)
        panic!("Enclave is unexpectedly None!");
    };
    let enclave_message = "successfullmatch";  // dummy value for now, but this should come from your actual logic

    if enclave_message == "successfullmatch" {
        let transaction_1 = DVPTransaction {
            transaction_type: TransactionType::TokenTransfer,
            sender: "escrowaddress1234".into(),
            recipient: "billaddress1234".into(),
            token_id: "1".into(),
            amount: 50,
        };

        let transaction_2 = DVPTransaction {
            transaction_type: TransactionType::Payment,
            sender: "escrowaddress1234".into(),
            recipient: "alexaddress1234".into(),
            token_id: "2".into(),
            amount: 1,
        };

        let response = ReturnEnclaveTransactions {
            transaction_1,
            transaction_2,
        };

        Ok(Json(ResponseType::Transactions(response)))
    } else if enclave_message == "successfullyadded" {
        let success_response = SuccessResponse {
            message: "Successfully added new DVp transaction.".into(),
        };

        Ok(Json(ResponseType::Message(success_response)))
    } else {
        let error_response = ErrorResponse {
            error: "Invalid message content.".into(),
        };

        Err(Json(error_response))
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
