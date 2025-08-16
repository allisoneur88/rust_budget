use crate::{
    AccountType, Currency,
    repository::traits::{
        AccountRepository, AssignmentRepository, BudgetRepository, CategoryRepository,
        SuperCategoryRepository, SuperTransactionRepository, TransactionRepository, UserRepository,
    },
};

pub struct AppState {
    pub version: u32,
    pub users: Box<dyn UserRepository>,
    pub budgets: Box<dyn BudgetRepository>,
    pub account: Box<dyn AccountRepository>,
    pub super_categories: Box<dyn SuperCategoryRepository>,
    pub super_transactions: Box<dyn SuperTransactionRepository>,
    pub categories: Box<dyn CategoryRepository>,
    pub transactions: Box<dyn TransactionRepository>,
    pub assignments: Box<dyn AssignmentRepository>,
    pub account_types: Vec<AccountType>,
    pub currencies: Vec<Currency>,
}
