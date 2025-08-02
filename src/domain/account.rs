use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Currency;

use super::account_type::AccountType;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub is_off_budget: bool,
    pub acc_type: AccountType,
    pub currency: Currency,

    pub budget_id: Uuid,
}
