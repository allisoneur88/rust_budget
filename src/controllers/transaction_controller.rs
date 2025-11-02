use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Transaction,
    app::repositories::Repositories,
    util::error::{AppError, AppResult},
};

pub struct TransactionController {
    repos: Arc<Repositories>,
}

impl TransactionController {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub fn get_all(&self, super_transaction_id: Uuid) -> AppResult<Vec<Transaction>> {
        self.repos.transactions.list(super_transaction_id)
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<Transaction> {
        self.repos
            .transactions
            .get(id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Transaction",
                id,
            })
    }

    pub fn create<M: Into<String>>(
        &self,
        flow: f64,
        memo: M,
        account_id: Uuid,
        category_id: Uuid,
        super_transaction_id: Uuid,
        payee_id: Uuid,
    ) -> AppResult<Transaction> {
        let account = self
            .repos
            .accounts
            .get(account_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Account",
                id: account_id,
            })?;
        self.repos
            .categories
            .get(category_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Category",
                id: category_id,
            })?;
        self.repos
            .super_transactions
            .get(super_transaction_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Super Transaction",
                id: super_transaction_id,
            })?;
        self.repos
            .payees
            .get(payee_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Payee",
                id: payee_id,
            })?;
        let transaction = Transaction::new(
            flow,
            memo,
            account_id,
            category_id,
            super_transaction_id,
            account.currency_id,
            payee_id,
        );
        self.repos.transactions.save(&transaction)?;
        Ok(transaction)
    }

    pub fn update_flow(&self, id: Uuid, new_flow: f64) -> AppResult<()> {
        let mut transaction = self.get_by_id(id)?;
        transaction.update_flow(new_flow);
        self.repos.transactions.save(&transaction)
    }

    pub fn update_memo<M: Into<String>>(&self, id: Uuid, new_memo: M) -> AppResult<()> {
        let mut transaction = self.get_by_id(id)?;
        transaction.update_memo(new_memo);
        self.repos.transactions.save(&transaction)
    }

    pub fn update_account(&self, id: Uuid, new_account_id: Uuid) -> AppResult<()> {
        let mut transaction = self.get_by_id(id)?;
        let new_account =
            self.repos
                .accounts
                .get(new_account_id)?
                .ok_or_else(|| AppError::NotFound {
                    entity: "Account",
                    id: new_account_id,
                })?;
        transaction.update_account(new_account.id, new_account.currency_id);
        self.repos.transactions.save(&transaction)
    }
    pub fn update_category(&self, id: Uuid, new_category_id: Uuid) -> AppResult<()> {
        let mut transaction = self.get_by_id(id)?;
        self.repos
            .categories
            .get(new_category_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Category",
                id: new_category_id,
            })?;
        transaction.update_category(new_category_id);
        self.repos.transactions.save(&transaction)
    }
    pub fn update_super_transaction(
        &self,
        id: Uuid,
        new_super_transaction_id: Uuid,
    ) -> AppResult<()> {
        let mut transaction = self.get_by_id(id)?;
        self.repos
            .super_transactions
            .get(new_super_transaction_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Super Transaction",
                id: new_super_transaction_id,
            })?;
        transaction.update_super_transaction(new_super_transaction_id);
        self.repos.transactions.save(&transaction)
    }

    pub fn update_payee(&self, id: Uuid, new_payee_id: Uuid) -> AppResult<()> {
        let mut transaction = self.get_by_id(id)?;
        self.repos
            .payees
            .get(new_payee_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Payee",
                id: new_payee_id,
            })?;
        transaction.update_payee(new_payee_id);
        self.repos.transactions.save(&transaction)
    }
}
