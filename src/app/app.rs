use std::{cell::RefCell, rc::Rc};

use crate::{
    app::app_state::AppState,
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

pub struct App {
    pub app_state: Rc<RefCell<AppState>>,
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

impl App {
    pub fn new() -> AppResult<Self> {
        let app_state = Rc::new(RefCell::new(AppState::new()?));

        let users = UserController::new(app_state.clone());
        let budgets = BudgetController::new(app_state.clone());
        let super_transactions = SuperTransactionController::new(app_state.clone());
        let super_categories = SuperCategoryController::new(app_state.clone());
        let accounts = AccountController::new(app_state.clone());
        let transactions = TransactionController::new(app_state.clone());
        let categories = CategoryController::new(app_state.clone());
        let payees = PayeeController::new(app_state.clone());
        let assignments = AssignmentController::new(app_state.clone());
        let currencies = CurrencyController::new(app_state.clone());

        Ok(Self {
            app_state,
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
