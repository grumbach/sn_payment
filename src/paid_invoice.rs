use crate::error::{Error, Result};
use crate::invoice::Invoice;
use crate::payment::Payment;

// buyer pays it.
pub struct PaidInvoice {
    pub invoice: Invoice,
    pub payment: Payment,
}

impl PaidInvoice {
    pub fn verify(&self) -> Result<()> {
        self.invoice.verify()?;

        let payto_public_key = self.invoice.content.payto_public_key;

        let payment_sum = self.payment.commitment_sum_by_owner(&payto_public_key)?;

        let invoice_amount_commitment = self.invoice.content.amount_commitment;

        if payment_sum == invoice_amount_commitment {
            Ok(())
        } else {
            Err(Error::PaymentDoesNotMatchInvoiceAmount)
        }
    }
}
