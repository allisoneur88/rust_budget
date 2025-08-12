use serde::{Deserialize, Serialize};

use crate::{
    Account, AccountType, Assignment, Budget, Category, Currency, SuperCategory, SuperTransaction,
    Transaction, User,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    pub version: u32,
    pub users: Vec<User>,
    pub budgets: Vec<Budget>,
    pub account: Vec<Account>,
    pub super_categories: Vec<SuperCategory>,
    pub super_transactions: Vec<SuperTransaction>,
    pub categories: Vec<Category>,
    pub transactions: Vec<Transaction>,
    pub assignments: Vec<Assignment>,
    pub account_types: Vec<AccountType>,
    pub currencies: Vec<Currency>,
}
