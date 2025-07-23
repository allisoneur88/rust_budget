use uuid::Uuid;

use super::account_type::AccountType;

#[derive(Debug)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub is_off_budget: bool,
    pub acc_type: AccountType,
}
