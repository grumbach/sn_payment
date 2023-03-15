use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use sn_dbc::{Commitment, Dbc, Hash, PublicKey};
use tiny_keccak::{Hasher, Sha3};

/// a payment is a list of Dbc
#[derive(Clone, Deserialize, Serialize)]
pub struct Payment {
    pub dbcs: Vec<Dbc>,
}

impl Payment {
    /// payment hash
    pub fn hash(&self) -> sn_dbc::Hash {
        let mut sha3 = Sha3::v256();
        for dp in self.dbcs.iter() {
            sha3.update(dp.hash().as_ref());
        }
        let mut hash = [0u8; 32];
        sha3.finalize(&mut hash);
        Hash::from(hash)
    }

    /// retrieve sum of commitments for Dbcs derived from payto_public_key
    pub fn commitment_sum_by_owner(&self, payto_public_key: &PublicKey) -> Result<Commitment> {
        self.dbcs
            .iter()
            .filter(|d| &d.content.owner_base.public_key() == payto_public_key)
            .map(|d| d.commitment().map_err(|_| Error::AmountCommitmentInvalid))
            .sum::<Result<Commitment, _>>()
    }
}

/*
const PAYMENT_MEMO_MAX_BYTES: usize = 128;

#[derive(Clone, Deserialize, Serialize)]
struct UserPayment {
    #[serde(deserialize_with = "memo_deserialize")]
    memo: Vec<u8>, // utf-8, max_bytes = PAYMENT_MEMO_MAX_BYTES
}

impl TryFrom<(&str)> for UserPayment {
    type Error = Error;

    fn try_from(params: (&str)) -> Result<Self> {
        let (memo_str) = params;

        let memo_bytes = memo_str.as_bytes();
        let memo = if memo_bytes.len() <= PAYMENT_MEMO_MAX_BYTES {
            memo_bytes.to_vec()
        } else {
            return Err(Error::MemoTooLong);
        };

        Ok(Self { memo })
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
*/
