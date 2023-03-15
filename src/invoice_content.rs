use serde::{Deserialize, Serialize};
use sn_dbc::{rng::thread_rng, Amount, AmountSecrets, Ciphertext, Commitment, Hash, PublicKey};
use tiny_keccak::{Hasher, Sha3};

/// represents data fields of an Invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceContent {
    pub amount_commitment: Commitment,
    pub amount_secrets_cipher: Ciphertext,
    pub payto_public_key: PublicKey, // Owner's well-known key.  must match key Dbc.owner_base().public_key
}

impl From<(Amount, PublicKey)> for InvoiceContent {
    /// create InvoiceContent from Amount and PublicKey
    fn from(params: (Amount, PublicKey)) -> Self {
        let (amount, payto_public_key) = params;
        let mut rng = thread_rng();

        let amount_secrets = AmountSecrets::from_amount(amount, &mut rng);
        let amount_commitment = amount_secrets.commitment();
        let amount_secrets_cipher = amount_secrets.encrypt(&payto_public_key);

        Self {
            amount_commitment,
            amount_secrets_cipher,
            payto_public_key,
        }
    }
}

impl InvoiceContent {
    /// represent as byte array
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Default::default();
        v.extend(&self.amount_commitment.compress().to_bytes());
        v.extend(&self.amount_secrets_cipher.to_bytes());
        v.extend(&self.payto_public_key.to_bytes());
        v
    }

    /// generate hash
    pub fn hash(&self) -> Hash {
        let mut sha3 = Sha3::v256();

        sha3.update(&self.to_bytes());

        let mut hash = [0; 32];
        sha3.finalize(&mut hash);
        Hash::from(hash)
    }

    /// Checks if the provided AmountSecrets matches the amount commitment.
    /// note that both the amount and blinding_factor must be correct.
    pub fn verify_provided_amount_matches_commitment(&self, amount: &AmountSecrets) -> bool {
        self.amount_commitment == amount.commitment()
    }
}
