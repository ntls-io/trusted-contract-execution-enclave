
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub transaction_id: String,
    pub sender: String,
    pub recipient: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingAssetTransaction {
    pub transaction: Transaction,
    pub token_amount: i32,
    pub trade_price: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingPaymentTransaction {
    pub transaction: Transaction,
    pub payment_amount: i32,
    pub trade_price: i32,
}


#[derive(Debug, Deserialize, Serialize)]
pub enum TransactionType {
    Payment,
    TokenTransfer,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DVPTransaction {
    pub transaction_type: TransactionType,
    pub sender: String,
    pub recipient: String,
    pub token_id: String, // XRP or token
    pub amount: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReturnEnclaveTransactions {
    pub transaction_1: DVPTransaction,
    pub transaction_2: DVPTransaction,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
   pub  message: String,
}

#[derive(Debug, Serialize)]
pub enum ResponseType {
   Transactions(ReturnEnclaveTransactions),
   Message(SuccessResponse),
}