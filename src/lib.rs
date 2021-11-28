/// Flow:
///  1. buyer informs seller s/he wishes to send payment, and attaches a public key.
///  2. seller creates an invoice and returns to buyer.  The invoice contains:
///       a. an amount commitment.
///       b. AmountSecret (amount, blindfactor) ciphertext encrypted to buyer's pubkey
///       c. payto:  owner's well-known pubkey (should be one-time-use)
///       d. seller_signature.  signature over the other Invoice fields.
///  3. buyer verifies seller's signature on Invoice
///  4. buyer decrypts the AmountSecret to obtain the invoice amount.
///  5. buyer reissues necessary DBC(s) using payto as the recipient's well-known key
///     in the exact amount of the invoice.
///  6. buyer constructs a Payment which consists of one or more DBCs paying to seller.
///  7. buyer constructs a PaidInvoice which consists of the Invoice and Payment.
///  8. buyer sends the PaidInvoice to the seller.  transaction is complete.
///  9. seller verifies that:
///       a. the invoice is valid, with seller's own signature
///       b. the sum of payment commitments is equal to the invoice amount commitment.
///  10. seller is satisfied, delivers goods to buyer.
///
///  11. if buyer ever needs to prove payment, buyer can show any third party the PaidInvoice
///      and the 3rd party can verify that payment amount matches invoice amount, but
///      cannot actually see the invoice or payment amount
///      (without obtaining buyer's secret key).
///
///      note that payment is not actually proven unless/until buyer can prove that
///      seller has access to the PaidInvoice. This can be done by publishing it for
///      all to see.
mod error;
mod invoice;
mod invoice_content;
mod paid_invoice;
mod payment;

pub use crate::{
    error::{Error, Result},
    invoice::Invoice,
    invoice_content::InvoiceContent,
    paid_invoice::PaidInvoice,
    payment::Payment,
};
