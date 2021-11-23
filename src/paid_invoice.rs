use crate::error::{Error, Result};
use crate::invoice::Invoice;
use crate::payment::Payment;
use blsttc::SecretKey;
use sn_dbc::Amount;

// buyer pays it.
pub struct PaidInvoice {
    pub invoice: Invoice,
    pub payment: Payment,
}

impl PaidInvoice {
    pub fn verify_by_secret_key(&self, secret_key: &SecretKey) -> Result<()> {
        let payto_sum = self
            .payment
            .dbc_packets
            .iter()
            .filter(|d| d.dbc().owner() == self.invoice.payto)
            .map(
                |d| match d.dbc().content.amount_secret_by_secret_key(secret_key) {
                    Ok(s) => Ok(s.amount),
                    Err(e) => Err(Error::from(e)),
                },
            )
            .sum::<Result<Amount>>()?;

        if payto_sum >= self.invoice.amount {
            Ok(())
        } else {
            Err(Error::InsufficientPayment)
        }
    }
}
