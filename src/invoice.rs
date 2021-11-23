use sn_dbc::{Amount, PublicKey, Signature};

// seller commits to a price.
pub struct Invoice {
    // pub invoice_id: [u8; 32],  <---- should we have an ID?
    // pub datetime: DateTime     <---- what about a date/time?  system cannot verify
    pub amount: Amount,
    pub payto: PublicKey,
    pub seller_signature: Signature,
    // pub content: InvoiceContent. <--- should we transport arbitrary invoice content?
    // pub content_hash: [u8; 32],  <--- and/or just a hash of an invoice?
}

// pub enum InvoiceContent {
//     Pdf(data),
//     Html(data),
//     Markdown(data),
//     Text(data),
// }
