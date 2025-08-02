use std::clone;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::currency::Currency;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub flow: f64, // negative for outflow, positive for inflow
    pub currency: Currency,
    pub memo: String,

    //Foreign keys
    pub account_id: Uuid,
    pub category_id: Uuid,
    pub super_transaction_id: Uuid,
    pub payee_id: Uuid,
}
