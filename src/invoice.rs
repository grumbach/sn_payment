use crate::error::{Error, Result};
use crate::InvoiceContent;
use serde::{Deserialize, Serialize};
use sn_dbc::{Hash, Signature};
use tiny_keccak::{Hasher, Sha3};

/// seller issues an Invoice thereby commiting to a price/amount.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub content: InvoiceContent,
    pub payto_signature: Signature,
}

impl Invoice {
    /// verifies that payto_signature is correct.
    pub fn verify(&self) -> Result<()> {
        let valid = self
            .content
            .payto_public_key
            .verify(&self.payto_signature, self.content.to_bytes());

        match valid {
            true => Ok(()),
            false => Err(Error::InvoiceSignatureInvalid),
        }
    }

    /// represent Invoice as bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Default::default();
        v.extend(&self.content.to_bytes());
        v.extend(&self.payto_signature.to_bytes());
        v
    }

    /// generate hash of Invoice
    pub fn hash(&self) -> Hash {
        let mut sha3 = Sha3::v256();

        sha3.update(&self.to_bytes());

        let mut hash = [0; 32];
        sha3.finalize(&mut hash);
        Hash::from(hash)
    }
}

// pub struct UserInvoice {
//     Date/Time,
//     Invoice,
//     invoice_content: SafeUrl?  Url?
//     InvoiceContent,
// }

// pub enum InvoiceContent {
//     Pdf(data),
//     Html(data),
//     Markdown(data),
//     Text(data),
// }
