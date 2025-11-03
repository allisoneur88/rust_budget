use std::sync::Arc;

use crate::{
    app::{app_config::AppConfig, app_state::AppState, repositories::Repositories},
    controllers::{
        account_controller::AccountController, assignment_controller::AssignmentController,
        budget_controller::BudgetController, category_controller::CategoryController,
        currency_controller::CurrencyController, payee_controller::PayeeController,
        super_category_controller::SuperCategoryController,
        super_transaction_controller::SuperTransactionController,
        transaction_controller::TransactionController, user_controller::UserController,
    },
    util::error::AppResult,
};

#[allow(dead_code)]
pub struct Application {
    pub app_state: AppState,
    repos: Arc<Repositories>,

    pub users: UserController,
    pub budgets: BudgetController,
    pub super_transactions: SuperTransactionController,
    pub super_categories: SuperCategoryController,
    pub accounts: AccountController,
    pub transactions: TransactionController,
    pub categories: CategoryController,
    pub payees: PayeeController,
    pub assignments: AssignmentController,
    pub currencies: CurrencyController,
}

impl Application {
    pub fn new() -> AppResult<Self> {
        let config = AppConfig::load("config.toml")?;
        let repos = Arc::new(Repositories::new(&config)?);
        let app_state = AppState::new();

        let users = UserController::new(repos.clone());
        let budgets = BudgetController::new(repos.clone());
        let super_transactions = SuperTransactionController::new(repos.clone());
        let super_categories = SuperCategoryController::new(repos.clone());
        let accounts = AccountController::new(repos.clone());
        let transactions = TransactionController::new(repos.clone());
        let categories = CategoryController::new(repos.clone());
        let payees = PayeeController::new(repos.clone());
        let assignments = AssignmentController::new(repos.clone());
        let currencies = CurrencyController::new(repos.clone());

        Ok(Self {
            app_state,
            repos,
            users,
            budgets,
            super_transactions,
            super_categories,
            accounts,
            transactions,
            categories,
            payees,
            assignments,
            currencies,
        })
    }
}
