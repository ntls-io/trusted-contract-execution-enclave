use heapless::String;
use heapless::consts::*;
use serde::{Deserialize, Serialize};

pub fn deserialize_heapless_string<'de, D>(deserializer: D) -> Result<String<U256>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    Ok(String::from(s))
}

pub fn serialize_heapless_string<S>(string: &String<U256>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(string.as_str())
}


#[derive(Debug, Deserialize, Serialize)]
pub struct AssetTransaction {
    #[serde(deserialize_with = "deserialize_heapless_string", serialize_with = "serialize_heapless_string")]
    pub transaction_id: String<U256>,
    #[serde(deserialize_with = "deserialize_heapless_string", serialize_with = "serialize_heapless_string")]
    pub sender: String<U256>,
    #[serde(deserialize_with = "deserialize_heapless_string", serialize_with = "serialize_heapless_string")]
    pub recipient: String<U256>,
    pub token_amount: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentTransaction {
    #[serde(deserialize_with = "deserialize_heapless_string", serialize_with = "serialize_heapless_string")]
    pub transaction_id: String<U256>,
    #[serde(deserialize_with = "deserialize_heapless_string", serialize_with = "serialize_heapless_string")]
    pub sender: String<U256>,
    #[serde(deserialize_with = "deserialize_heapless_string", serialize_with = "serialize_heapless_string")]
    pub recipient: String<U256>,
    pub payment_amount: i32,
}

#[derive(Debug, Deserialize, Serialize)] // Use the correct derive
pub struct IncomingAssetTransaction {
    pub transaction: AssetTransaction,
    pub agreed_token_amount: i32,
    pub agreed_trade_price: i32,
}

#[derive(Debug, Deserialize, Serialize)] // Use the correct derive
pub struct IncomingPaymentTransaction {
    pub transaction: PaymentTransaction,
    pub agreed_token_amount: i32,
    pub agreed_trade_price: i32,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ReturnTransaction {
    #[serde(deserialize_with = "deserialize_heapless_string", serialize_with = "serialize_heapless_string")]
    pub transaction_type: String<U256>,
    #[serde(deserialize_with = "deserialize_heapless_string", serialize_with = "serialize_heapless_string")]
    pub sender: String<U256>,
    #[serde(deserialize_with = "deserialize_heapless_string", serialize_with = "serialize_heapless_string")]
    pub recipient: String<U256>,
    #[serde(deserialize_with = "deserialize_heapless_string", serialize_with = "serialize_heapless_string")]
    pub token_id: String<U256>,
    #[serde(deserialize_with = "deserialize_heapless_string", serialize_with = "serialize_heapless_string")]
    pub amount: String<U256>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ReturnDVPTransactions {
    #[serde(deserialize_with = "deserialize_heapless_string", serialize_with = "serialize_heapless_string")]
    pub message: String<U256>,
    pub transaction_1: Option<ReturnTransaction>,
    pub transaction_2: Option<ReturnTransaction>,
}