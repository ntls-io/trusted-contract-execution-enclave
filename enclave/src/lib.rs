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
extern crate heapless;
extern crate serde;
extern crate serde_json_core;

use sgx_types::*;
use std::io::{ Write};
use std::slice;
use heapless::String;
use std::sgxfs::SgxFile;
use serde::{Deserialize};
use std::io::Read;

mod structs;

// Import the structs for easier reference
use structs::{
    IncomingAssetTransaction,
    IncomingPaymentTransaction,
    ReturnDVPTransactions,
    ReturnTransaction,
};


fn read_and_deserialize_file<T: for<'de> Deserialize<'de> + core::fmt::Debug>(name: &str) -> Result<T, sgx_status_t> {
    let mut file = SgxFile::open(name).map_err(|e| {
        println!("Failed to open file: {:?}", e);
        sgx_status_t::SGX_ERROR_UNEXPECTED
    })?;
    
    let mut contents = vec![];
    file.read_to_end(&mut contents).map_err(|_| {
        println!("Failed to read file contents.");
        sgx_status_t::SGX_ERROR_UNEXPECTED
    })?;
    println!("Read file contents: {:?}",&contents);

    let (deserialized, _): (T, _) = serde_json_core::from_slice(&contents).map_err(|_| {
        println!("Failed to deserialize file contents.");
        sgx_status_t::SGX_ERROR_UNEXPECTED
    })?;
    println!("Deserialized data from file: {:?}", deserialized);


    Ok(deserialized)
}

/// # Safety
/// The caller needs to ensure that `binary` is a valid pointer to a slice valid for `binary_len` items
/// and that `result_out` is a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn asset_transaction_check(
    data_bytes: *const u8,
    data_bytes_len: usize,
    result_out: *mut u8,
) -> sgx_status_t {
     // Safety: SGX generated code will check that the pointer is valid.
    if data_bytes.is_null() {
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER;
    }
   
    let data_bytes_slice = unsafe { slice::from_raw_parts(data_bytes, data_bytes_len) };
    let (deserialized_data, _bytes_read): (IncomingAssetTransaction, usize) = serde_json_core::from_slice(data_bytes_slice).expect("Failed to deserialize");
    println!("Size of deserialized data: {}", core::mem::size_of_val(&deserialized_data));

    println!("deserialised data {:?}", deserialized_data);

    let serialized = serde_json_core::to_string::<IncomingAssetTransaction, 1024>(&deserialized_data).unwrap();
    let serialized_bytes = serialized.as_bytes();

    // Extract sender's address from the deserialized data to use as the file name
    let incoming_txn_recipient = &deserialized_data.transaction.recipient;
    let incoming_txn_sender = &deserialized_data.transaction.sender;

    let incoming_txn_recipient_str = incoming_txn_recipient.as_str();
    let incoming_txn_sender_str = incoming_txn_sender.as_str();

    // Check if recipient file exists
    match handle_sgx_file(&incoming_txn_recipient_str, serialized_bytes, false) {
        // Recipient file exists, handle its logic
        Ok(_) => {
            println!("MATCHED with corresponding payment transaction: Recipient file {} exists.", incoming_txn_recipient_str);
            println!("Attempting to read file: {}", incoming_txn_recipient_str);

            // 1. Read the recipient file and deserialize it into IncomingPaymentTransaction
            let payment_txn = match read_and_deserialize_file::<IncomingPaymentTransaction>(&incoming_txn_recipient_str) {
                Ok(value) => value,
                Err(e) => return e
            };

            println!("Existing Payment Transaction recipient: {}", payment_txn.transaction.recipient);
            println!("Incoming Asset Transaction sender: {}", incoming_txn_sender);
        println!("Existing ayment Transaction payment_amount: {}", payment_txn.transaction.payment_amount);
        println!("Incoming Asset Transaction agreed_trade_price: {}", deserialized_data.agreed_trade_price);
        println!("Incoming Asset Transaction token_amount: {}", deserialized_data.transaction.token_amount);
        println!("Existing Payment Transaction agreed_token_amount: {}", payment_txn.agreed_token_amount);

            // 2. Compare fields
        if payment_txn.transaction.recipient == *incoming_txn_sender && 
            payment_txn.transaction.payment_amount == deserialized_data.agreed_trade_price && 
            deserialized_data.transaction.token_amount == payment_txn.agreed_token_amount {
            println!("Successful Match - return enclave transactions");
            // Create the two transaction structs
            let txn1 = ReturnTransaction {
            transaction_type: "Payment".into(),
            sender: "enclave wallet address".into(),
            recipient: deserialized_data.transaction.sender.into(),
            token_id: "XRP".into(), // or whatever value you need
            amount: deserialized_data.agreed_trade_price.into(),
            };
            let txn2 = ReturnTransaction {
            transaction_type: "TokenTransfer".into(),
            sender: "enclave wallet address".into(),
            recipient: payment_txn.transaction.sender.into(),
            token_id: "Foo".into(), // or whatever value you need
            amount: payment_txn.agreed_token_amount.into(),
            };
         // Create the ReturnTransaction struct
            let return_txn = ReturnDVPTransactions {
            message: "SuccessMatch".into(),
            transaction_1: Some(txn1),
            transaction_2: Some(txn2),
            };
        // Serialize return_txn and write to result_out
            let serialized = serde_json_core::to_string::<ReturnDVPTransactions, 2048>(&return_txn).unwrap();
            let serialized_bytes = serialized.as_bytes();

            for (i, &byte) in serialized_bytes.iter().enumerate() {
                    unsafe { *result_out.add(i) = byte; }
            }

            return sgx_status_t::SGX_SUCCESS

            }}
        // Recipient file doesn't exist
        Err(_) => {
            // Check if sender file exists
            match handle_sgx_file(&incoming_txn_sender_str, serialized_bytes, false) {
                // Sender file exists, hence it's a duplicate transaction
                Ok(_) => {
                    let return_txn = ReturnDVPTransactions {
                        message: String::from("DuplicateTransactionExists"),
                        transaction_1: None,
                        transaction_2: None,
                    };
                    let serialized = serde_json_core::to_string::<ReturnDVPTransactions, 1024>(&return_txn).unwrap();
                    let serialized_bytes = serialized.as_bytes();
                    for (i, &byte) in serialized_bytes.iter().enumerate() {
                        unsafe { *result_out.add(i) = byte; }
                    }
                    return sgx_status_t::SGX_SUCCESS;
                },
                // Sender file doesn't exist, we add a new transaction
                Err(_) => {
                        match handle_sgx_file(&incoming_txn_sender_str, serialized_bytes, true) {
                    Ok(_) => { // Successfully created the sender's file
                        let return_txn = ReturnDVPTransactions {
                            message: String::from("SuccessfullyAddedNewTransaction"),
                            transaction_1: None,
                            transaction_2: None,
                        };
                        let serialized = serde_json_core::to_string::<ReturnDVPTransactions, 1024>(&return_txn).unwrap();
                        let serialized_bytes = serialized.as_bytes();
                        for (i, &byte) in serialized_bytes.iter().enumerate() {
                            unsafe { *result_out.add(i) = byte; }
                        }
                        return sgx_status_t::SGX_SUCCESS;
                    },
                    Err(e) => { // Error while trying to create the sender's file
                        println!("Error when trying to create the sender's file: {:?}", e);
                        return sgx_status_t::SGX_ERROR_UNEXPECTED;
                    }
                }
                }
            }
        }
    }
 sgx_status_t::SGX_SUCCESS


   
}

/// # Safety
/// The caller needs to ensure that `binary` is a valid pointer to a slice valid for `binary_len` items
/// and that `result_out` is a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn payment_transaction_check(
    data_bytes: *const u8,
    data_bytes_len: usize,
    result_out: *mut u8,
) -> sgx_status_t{
     // Safety: SGX generated code will check that the pointer is valid.
    if data_bytes.is_null() {
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER
    }
   
    let data_bytes_slice = unsafe { slice::from_raw_parts(data_bytes, data_bytes_len) };
    let (deserialized_data, _bytes_read): (IncomingPaymentTransaction, usize) = serde_json_core::from_slice(data_bytes_slice).expect("Failed to deserialize");
    println!("Size of deserialized data: {}", core::mem::size_of_val(&deserialized_data));

    println!("deserialised data {:?}", deserialized_data);

    let serialized = serde_json_core::to_string::<IncomingPaymentTransaction, 1024>(&deserialized_data).unwrap();
    let serialized_bytes = serialized.as_bytes();

    // Extract sender's address from the deserialized data to use as the file name
    let incoming_txn_recipient = &deserialized_data.transaction.recipient;
    let incoming_txn_sender = &deserialized_data.transaction.sender;

    let incoming_txn_recipient_str = incoming_txn_recipient.as_str();
    let incoming_txn_sender_str = incoming_txn_sender.as_str();

    // Check if recipient file exists
    match handle_sgx_file(&incoming_txn_recipient_str, serialized_bytes, false) {
        // Recipient file exists, handle its logic
        Ok(_) => {
            println!("MATCHED with corresponding asset transaction: Recipient file {} exists.", incoming_txn_recipient_str);
            println!("Attempting to read file: {}", incoming_txn_recipient_str);

            // 1. Read the recipient file and deserialize it into IncomingPaymentTransaction
            let asset_txn = match read_and_deserialize_file::<IncomingAssetTransaction>(&incoming_txn_recipient_str) {
                Ok(value) => value,
                Err(e) => return e
            };

            println!("Existing Asset Transaction recipient: {}", asset_txn.transaction.recipient);
            println!("Incoming Payment Transaction sender: {}", incoming_txn_sender);
        println!("Existing Asset Transaction token_amount: {}", asset_txn.transaction.token_amount);
        println!("Incoming Payment Transaction agreed_trade_price: {}", deserialized_data.agreed_trade_price);
        println!("Incoming Payment Transaction payment_amount: {}", deserialized_data.transaction.payment_amount);
        println!("Existing Asset Transaction agreed_token_amount: {}", asset_txn.agreed_token_amount);

            // 2. Compare fields
        if asset_txn.transaction.recipient == *incoming_txn_sender && 
            asset_txn.transaction.token_amount == deserialized_data.agreed_token_amount && 
            deserialized_data.transaction.payment_amount == asset_txn.agreed_trade_price {
            println!("Successful Match - return enclave transactions");
            // Create the two transaction structs
            let txn1 = ReturnTransaction {
            transaction_type: "Payment".into(),
            sender: "enclave wallet address".into(),
            recipient: asset_txn.transaction.sender.into(),
            token_id: "XRP".into(), // or whatever value you need
            amount: deserialized_data.agreed_trade_price.into(),
            };
            let txn2 = ReturnTransaction {
            transaction_type: "TokenTransfer".into(),
            sender: "enclave wallet address".into(),
            recipient: deserialized_data.transaction.sender.into(),
            token_id: "Foo".into(), // or whatever value you need
            amount: asset_txn.agreed_token_amount.into(),
            };
         // Create the ReturnTransaction struct
            let return_txn = ReturnDVPTransactions {
            message: "SuccessMatch".into(),
            transaction_1: Some(txn1),
            transaction_2: Some(txn2),
            };
        // Serialize return_txn and write to result_out
            let serialized = serde_json_core::to_string::<ReturnDVPTransactions, 2048>(&return_txn).unwrap();
            let serialized_bytes = serialized.as_bytes();


            for (i, &byte) in serialized_bytes.iter().enumerate() {
                    unsafe { *result_out.add(i) = byte; }
            }

            return sgx_status_t::SGX_SUCCESS

            }}
        
        // Recipient file doesn't exist
        Err(_) => {
            // Check if sender file exists
            match handle_sgx_file(&incoming_txn_sender_str, serialized_bytes, false) {
                // Sender file exists, hence it's a duplicate transaction
                Ok(_) => {
                    let return_txn = ReturnDVPTransactions {
                        message: String::from("DuplicateTransactionExists"),
                        transaction_1: None,
                        transaction_2: None,
                    };
                    let serialized = serde_json_core::to_string::<ReturnDVPTransactions, 1024>(&return_txn).unwrap();
                    let serialized_bytes = serialized.as_bytes();
                    for (i, &byte) in serialized_bytes.iter().enumerate() {
                        unsafe { *result_out.add(i) = byte; }
                    }
                    return sgx_status_t::SGX_SUCCESS;
                },

        // Sender file doesn't exist, we add a new transaction
                Err(_) => {
                        match handle_sgx_file(&incoming_txn_sender_str, serialized_bytes, true) {
                    Ok(_) => { // Successfully created the sender's file
                        let return_txn = ReturnDVPTransactions {
                            message: String::from("SuccessfullyAddedNewTransaction"),
                            transaction_1: None,
                            transaction_2: None,
                        };
                        let serialized = serde_json_core::to_string::<ReturnDVPTransactions, 1024>(&return_txn).unwrap();
                        let serialized_bytes = serialized.as_bytes();
                        for (i, &byte) in serialized_bytes.iter().enumerate() {
                            unsafe { *result_out.add(i) = byte; }
                        }
                        return sgx_status_t::SGX_SUCCESS;
                    },
                    Err(e) => { // Error while trying to create the sender's file
                        println!("Error when trying to create the sender's file: {:?}", e);
                        return sgx_status_t::SGX_ERROR_UNEXPECTED;
                    }
                }
                }
            }
        }
    }
 sgx_status_t::SGX_SUCCESS
}


// Helper function to check for the existence of a file or create (if `create` flag is true) 
// and write to it if not present
fn handle_sgx_file(name: &str, serialized_bytes: &[u8], create: bool) -> Result<(), sgx_status_t> {
    match SgxFile::open(name) {
        // If the file exists, return Ok
        Ok(_) => {
            Ok(())
        }
        Err(_) if create => {
            // If file doesn't exist and we want to create it, then create it and write the serialized data into it
            let mut file = SgxFile::create(name).map_err(|e| {
                println!("SgxFile::create failed with error: {:?}", e);
                sgx_status_t::SGX_ERROR_UNEXPECTED
            })?;

            file.write(serialized_bytes).map_err(|_| {
                println!("SgxFile::write failed.");
                sgx_status_t::SGX_ERROR_UNEXPECTED
            })?;

            file.flush().map_err(|_| {
                println!("SgxFile::flush failed.");
                sgx_status_t::SGX_ERROR_UNEXPECTED
            })?;

            Ok(())
        }
        Err(_) => {
            // If file doesn't exist and we don't want to create it, return Err
            Err(sgx_status_t::SGX_ERROR_UNEXPECTED)
        }
    }
}
