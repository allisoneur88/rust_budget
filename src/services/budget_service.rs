use crate::Account;
use crate::Budget;
use crate::Currency;
use crate::IdGenerator;
use crate::SuperCategory;
use crate::SuperTransaction;
use crate::domain::super_transaction;

#[derive(Debug)]
pub struct BudgetService;

impl BudgetService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_budget(&self, name: &str, main_currency: Currency) -> Budget {
        Budget {
            id: IdGenerator::new_id(),
            name: name.to_string(),
            accounts: None,
            super_categories: None,
            super_transactions: None,
            main_currency: main_currency,
        }
    }

    pub fn update_budget_name(&self, budget: &mut Budget, new_name: &str) {
        budget.name = new_name.to_string();
    }

    pub fn change_main_currency(&self, budget: &mut Budget, new_currency: Currency) {
        budget.main_currency = new_currency;
    }

    pub fn add_account(&self, budget: &mut Budget, account: Account) {
        match &mut budget.accounts {
            None => budget.accounts = Some(vec![account]),
            Some(accounts) => accounts.push(account),
        }
    }

    pub fn add_super_category(&self, budget: &mut Budget, super_category: SuperCategory) {
        match &mut budget.super_categories {
            None => budget.super_categories = Some(vec![super_category]),
            Some(super_categories) => super_categories.push(super_category),
        }
    }

    pub fn add_super_transaction(&self, budget: &mut Budget, super_transaction: SuperTransaction) {
        match &mut budget.super_transactions {
            None => budget.super_transactions = Some(vec![super_transaction]),
            Some(super_transactions) => super_transactions.push(super_transaction),
        }
    }
}
