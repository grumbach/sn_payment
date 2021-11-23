use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use sn_dbc::DbcPacket;
use std::convert::TryFrom;

const PAYMENT_MEMO_MAX_BYTES: usize = 128;

// A payment looks like
#[derive(Clone, Deserialize, Serialize)]
pub struct Payment {
    // payment id?
    // (unverified) date/time?
    pub dbc_packets: Vec<DbcPacket>,

    #[serde(deserialize_with = "memo_deserialize")]
    memo: Vec<u8>, // utf-8, max_bytes = PAYMENT_MEMO_MAX_BYTES
}

impl TryFrom<(Vec<DbcPacket>, &str)> for Payment {
    type Error = Error;

    fn try_from(params: (Vec<DbcPacket>, &str)) -> Result<Self> {
        let (dbc_packets, memo_str) = params;

        let memo_bytes = memo_str.as_bytes();
        let memo = if memo_bytes.len() <= PAYMENT_MEMO_MAX_BYTES {
            memo_bytes.to_vec()
        } else {
            return Err(Error::MemoTooLong);
        };

        let payment = Self { dbc_packets, memo };
        Ok(payment)
    }
}

fn memo_deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let v = Vec::<u8>::deserialize(deserializer)?;
    if v.len() <= PAYMENT_MEMO_MAX_BYTES {
        Ok(v)
    } else {
        Err(serde::de::Error::custom("memo too long"))
    }
}
