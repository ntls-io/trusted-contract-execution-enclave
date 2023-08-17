use serde::{Deserialize, Serialize};

// ----- Transactions -----

/// Represents an asset transaction
#[derive(Debug, Deserialize, Serialize)]
pub struct AssetTransaction {
    pub transaction_id: String,
    pub sender: String,
    pub recipient: String,
    pub token_amount: i32,
}

/// Represents a payment transaction
#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentTransaction {
    pub transaction_id: String,
    pub sender: String,
    pub recipient: String,
    pub payment_amount: i32,
}

/// Represents an incoming asset transaction with additional agreed upon details
#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingAssetTransaction {
    pub transaction: AssetTransaction,
    pub agreed_token_amount: i32,
    pub agreed_trade_price: i32,
}

/// Represents an incoming payment transaction with additional agreed upon details
#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingPaymentTransaction {
    pub transaction: PaymentTransaction,
    pub agreed_token_amount: i32,
    pub agreed_trade_price: i32,
}

// ----- Responses -----

/// Represents an error response with an error message
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ReturnTransaction {
    pub transaction_type: String,
    pub sender: String,
    pub recipient: String,
    pub token_id: String,
    pub amount: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ReturnDVPTransactions {
    pub message: String,
    pub transaction_1: Option<ReturnTransaction>,
    pub transaction_2: Option<ReturnTransaction>,
}
