use heapless::String;
use heapless::consts::U256;
use serde::{Deserialize, Serialize};

pub fn deserialize_heapless_string<'de, D>(deserializer: D) -> Result<String<U256>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    Ok(String::from(s))
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(deserialize_with = "deserialize_heapless_string")]
    pub transaction_id: String<U256>,
    #[serde(deserialize_with = "deserialize_heapless_string")]
    pub sender: String<U256>,
    #[serde(deserialize_with = "deserialize_heapless_string")]
    pub recipient: String<U256>,
}

#[derive(Debug, Deserialize)] // Use the correct derive
pub struct IncomingAssetTransaction {
    pub transaction: Transaction,
    pub token_amount: i32,
    pub trade_price: i32,
}

//#[derive(Debug, Deserialize, Serialize)] // Use the correct derive
pub struct IncomingPaymentTransaction {
    pub transaction: Transaction,
    pub payment_amount: i32,
    pub trade_price: i32,
}
