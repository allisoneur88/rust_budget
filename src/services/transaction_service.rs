use crate::{Account, Category, IdGenerator, Payee, SuperTransaction, Transaction};

#[derive(Debug)]
pub struct TransactonService;

impl TransactonService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_transaction(
        &self,
        flow: f64,
        memo: &str,
        account: &Account,
        category: &Category,
        super_transaction: &SuperTransaction,
        payee: &Payee,
    ) -> Transaction {
        Transaction {
            id: IdGenerator::new_id(),
            flow,
            memo: memo.to_string(),
            currency: account.currency.clone(),
            account_id: account.id,
            category_id: category.id,
            super_transaction_id: super_transaction.id,
            payee_id: payee.id,
        }
    }

    pub fn update_flow(&self, transaction: &mut Transaction, new_flow: f64) {
        transaction.flow = new_flow;
    }

    pub fn update_memo(&self, transaction: &mut Transaction, new_memo: &str) {
        transaction.memo = new_memo.to_string();
    }

    pub fn update_account(&self, transaction: &mut Transaction, new_account: &Account) {
        transaction.account_id = new_account.id;
        transaction.currency = new_account.currency.clone();
    }

    pub fn update_category(&self, transaction: &mut Transaction, new_category: &Category) {
        transaction.category_id = new_category.id;
    }

    pub fn update_super_transaction(
        &self,
        transaction: &mut Transaction,
        new_super_transaction: &SuperTransaction,
    ) {
        transaction.super_transaction_id = new_super_transaction.id;
    }

    pub fn update_payee(&self, transaction: &mut Transaction, new_payee: &Payee) {
        transaction.payee_id = new_payee.id;
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::{
        AccountType, Currency,
        domain::{category, super_category, super_transaction},
        services::{
            account_service::AccountService, budget_service::BudgetService,
            category_service::CategoryService, currency_service::CurrencyService,
            payee_service::PayeeService, super_category_service::SuperCategoryService,
            super_transaction_service::SuperTransactionService, user_service::UserService,
        },
    };

    use super::*;
    #[test]
    pub fn create_everything_using_services() {
        let us = UserService::new();
        let user = us.make_user_wo_password("sasha");

        let bs = BudgetService::new();
        let cs = CurrencyService::new();
        let currency = cs.make_currency("RUB", "R", "Roubles");
        let budget = bs.make_budget("budget", currency.clone(), &user);

        let accs = AccountService::new();
        let account = accs.make_account(
            "sber",
            false,
            AccountType::Checking,
            currency.clone(),
            &budget,
        );

        let sts = SuperTransactionService::new();
        let super_transaction =
            sts.make_super_transaction(NaiveDate::from_ymd_opt(2025, 8, 25).unwrap(), "", &budget);

        let scs = SuperCategoryService::new();
        let super_category = scs.make_super_category("bills", &budget);

        let cs = CategoryService::new();
        let category = cs.make_category("groceries", &super_category);

        let ps = PayeeService::new();
        let payee = ps.make_payee("Spar", "");

        let ts = TransactonService::new();
        let transaction = ts.make_transaction(
            -320.2,
            "spar",
            &account,
            &category,
            &super_transaction,
            &payee,
        );
    }
}
