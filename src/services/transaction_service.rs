use crate::{Account, Category, IdGenerator, Payee, SuperTransaction, Transaction};

#[derive(Debug)]
pub struct TransactonService;

impl TransactonService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_transaction_wo_memo(
        &self,
        flow: f64,
        account: &Account,
        category: &Category,
        super_transaction: &SuperTransaction,
        payee: &Payee,
    ) -> Transaction {
        Transaction {
            id: IdGenerator::new_id(),
            flow,
            currency: account.currency.clone(),
            memo: "".to_string(),
            account_id: account.id,
            category_id: category.id,
            super_transaction_id: super_transaction.id,
            payee_id: payee.id,
        }
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
        domain::super_transaction,
        services::{
            account_service::AccountService, budget_service::BudgetService,
            category_service::CategoryService, payee_service::PayeeService,
            super_category_service::SuperCategoryService,
            super_transaction_service::SuperTransactionService, user_service::UserService,
        },
    };

    use super::*;
    #[test]
    pub fn create_everything_using_services() {
        let us = UserService::new();
        let mut user = us.make_user_wo_password("Sasha");

        let bs = BudgetService::new();
        let mut budget = bs.make_budget("Family", Currency::Roubles);

        let aserv = AccountService::new();
        let mut account =
            aserv.make_account("Sber", false, AccountType::Checking, Currency::Roubles);

        let scs = SuperCategoryService::new();
        let mut super_category = scs.make_super_category("Bills");

        let sts = SuperTransactionService::new();
        let mut super_transaction = sts.make_super_transaction(
            NaiveDate::from_ymd_opt(2025, 07, 24).unwrap(),
            "July Electricity",
        );

        let cs = CategoryService::new();
        let category = cs.make_category("Electricity");

        let ps = PayeeService::new();
        let payee = ps.make_payee("MosGorElek");

        let ts = TransactonService::new();
        let my_transaction =
            ts.make_transaction_wo_memo(-54.32, &account, &category, &super_transaction, &payee);

        sts.add_transaction(&mut super_transaction, my_transaction);
        scs.add_category(&mut super_category, category);
        bs.add_account(&mut budget, account);
        bs.add_super_category(&mut budget, super_category);
        bs.add_super_transaction(&mut budget, super_transaction);
        us.add_budget(&mut user, budget);

        assert_eq!(
            user.budgets.as_mut().unwrap()[0]
                .super_transactions
                .as_mut()
                .unwrap()[0]
                .transactions
                .as_mut()
                .unwrap()[0]
                .flow,
            -54.32
        );
    }
}
