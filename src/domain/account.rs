use super::account_type::AccountType;

#[derive(Debug)]
pub struct Account {
    pub id: u64,
    pub name: String,
    pub is_off_budget: bool,
    pub acc_type: AccountType,
}
