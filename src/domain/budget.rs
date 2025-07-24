use uuid::Uuid;

use super::{
    account::Account, currency::Currency, super_category::SuperCategory,
    super_transaction::SuperTransaction,
};

#[derive(Debug)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub accounts: Option<Vec<Account>>,
    pub super_categories: Option<Vec<SuperCategory>>,
    pub super_transactions: Option<Vec<SuperTransaction>>,
    pub main_currency: Currency,
}
