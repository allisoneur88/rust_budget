use uuid::Uuid;

use super::currency::Currency;

#[derive(Debug)]
pub struct Transaction {
    pub id: Uuid,
    pub flow: Option<f64>,
    pub currency: Currency,

    //Foreign keys
    pub account_id: u64,
    pub category_id: u64,
    pub super_transaction_id: u64,
    pub payee_id: u64,
}
