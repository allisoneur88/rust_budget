use crate::{
    Account, AccountType, Budget, User,
    app::app_config::AppConfig,
    repository::{
        file::{
            account_repo::FileAccountRepo, assignment_repo::FileAssignmentRepo,
            budget_repo::FileBudgetRepo, category_repo::FileCategoryRepo,
            currency_repo::FileCurrencyRepo, super_category_repo::FileSuperCategoryRepo,
            super_transaction_repo::FileSuperTransactionRepo,
            transaction_repo::FileTransactionRepo, user_repo::FileUserRepo,
        },
        traits::{
            AccountRepository, AssignmentRepository, BudgetRepository, CategoryRepository,
            CurrencyRepository, SuperCategoryRepository, SuperTransactionRepository,
            TransactionRepository, UserRepository,
        },
    },
};

pub struct AppState {
    pub version: u32,
    pub users: Box<dyn UserRepository>,
    pub budgets: Box<dyn BudgetRepository>,
    pub accounts: Box<dyn AccountRepository>,
    pub super_categories: Box<dyn SuperCategoryRepository>,
    pub super_transactions: Box<dyn SuperTransactionRepository>,
    pub categories: Box<dyn CategoryRepository>,
    pub transactions: Box<dyn TransactionRepository>,
    pub assignments: Box<dyn AssignmentRepository>,
    pub currencies: Box<dyn CurrencyRepository>,
    pub account_types: Vec<AccountType>,

    pub current_user: Option<User>,
    pub current_budget: Option<Budget>,
    pub current_account: Option<Account>,
}

impl AppState {
    pub fn new() -> Self {
        let config = AppConfig::load("config.toml").unwrap();

        Self {
            version: 1,
            users: Box::new(FileUserRepo::new(config.user_path)),
            budgets: Box::new(FileBudgetRepo::new(config.budget_path)),
            account: Box::new(FileAccountRepo::new(config.account_path)),
            super_categories: Box::new(FileSuperCategoryRepo::new(config.super_category_path)),
            super_transactions: Box::new(FileSuperTransactionRepo::new(
                config.super_transaction_path,
            )),
            categories: Box::new(FileCategoryRepo::new(config.category_path)),
            transactions: Box::new(FileTransactionRepo::new(config.transaction_path)),
            assignments: Box::new(FileAssignmentRepo::new(config.assignment_path)),
            currencies: Box::new(FileCurrencyRepo::new(config.currency_path)),
            account_types: vec![
                AccountType::Checking,
                AccountType::CreditCard,
                AccountType::Investment,
            ],

            current_user: None,
            current_budget: None,
            current_account: None,
        }
    }
}
