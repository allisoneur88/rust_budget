use std::sync::Arc;

use crate::{
    app::app_config::AppConfig,
    repository::{
        file::{
            account_repo::FileAccountRepo, assignment_repo::FileAssignmentRepo,
            budget_repo::FileBudgetRepo, category_repo::FileCategoryRepo,
            currency_repo::FileCurrencyRepo, payee_repo::FilePayeeRepo,
            super_category_repo::FileSuperCategoryRepo,
            super_transaction_repo::FileSuperTransactionRepo,
            transaction_repo::FileTransactionRepo, user_repo::FileUserRepo,
        },
        traits::{
            AccountRepository, AssignmentRepository, BudgetRepository, CategoryRepository,
            CurrencyRepository, PayeeRepository, SuperCategoryRepository,
            SuperTransactionRepository, TransactionRepository, UserRepository,
        },
    },
    util::error::AppResult,
};

pub struct Repositories {
    pub users: Arc<dyn UserRepository>,
    pub budgets: Arc<dyn BudgetRepository>,
    pub accounts: Arc<dyn AccountRepository>,
    pub currencies: Arc<dyn CurrencyRepository>,
    pub categories: Arc<dyn CategoryRepository>,
    pub super_categories: Arc<dyn SuperCategoryRepository>,
    pub transactions: Arc<dyn TransactionRepository>,
    pub super_transactions: Arc<dyn SuperTransactionRepository>,
    pub payees: Arc<dyn PayeeRepository>,
    pub assigments: Arc<dyn AssignmentRepository>,
}

impl Repositories {
    pub fn new(config: &AppConfig) -> AppResult<Self> {
        Ok(Self {
            users: Arc::new(FileUserRepo::new(config.user_path.clone())?),
            budgets: Arc::new(FileBudgetRepo::new(config.budget_path.clone())?),
            accounts: Arc::new(FileAccountRepo::new(config.account_path.clone())?),
            currencies: Arc::new(FileCurrencyRepo::new(config.currency_path.clone())?),
            categories: Arc::new(FileCategoryRepo::new(config.category_path.clone())?),
            super_categories: Arc::new(FileSuperCategoryRepo::new(
                config.super_category_path.clone(),
            )?),
            transactions: Arc::new(FileTransactionRepo::new(config.transaction_path.clone())?),
            super_transactions: Arc::new(FileSuperTransactionRepo::new(
                config.super_transaction_path.clone(),
            )?),
            payees: Arc::new(FilePayeeRepo::new(config.payee_path.clone())?),
            assigments: Arc::new(FileAssignmentRepo::new(config.assignment_path.clone())?),
        })
    }
}
