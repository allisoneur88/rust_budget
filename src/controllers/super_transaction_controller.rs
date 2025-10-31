use std::sync::Arc;

use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    SuperTransaction,
    app::repositories::Repositories,
    util::error::{AppError, AppResult},
};

pub struct SuperTransactionController {
    repos: Arc<Repositories>,
}

impl SuperTransactionController {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub fn get_all(&self, budget_id: Uuid) -> AppResult<Vec<SuperTransaction>> {
        self.repos.super_transactions.list(budget_id)
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<SuperTransaction> {
        self.repos
            .super_transactions
            .get(id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Super Transaction",
                id,
            })
    }

    pub fn create(
        &self,
        date: NaiveDate,
        memo: &str,
        budget_id: Uuid,
    ) -> AppResult<SuperTransaction> {
        self.repos
            .budgets
            .get(budget_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Budget",
                id: budget_id,
            })?;
        let super_transaction = SuperTransaction::new(date, memo, budget_id);
        self.repos.super_transactions.save(&super_transaction)?;
        Ok(super_transaction)
    }

    pub fn update_memo(&self, id: Uuid, new_memo: &str) -> AppResult<()> {
        let mut super_transaction = self.get_by_id(id)?;
        super_transaction.update_memo(new_memo);
        self.repos.super_transactions.save(&super_transaction)
    }

    pub fn update_date(&self, id: Uuid, new_date: NaiveDate) -> AppResult<()> {
        let mut super_transaction = self.get_by_id(id)?;
        super_transaction.update_date(new_date);
        self.repos.super_transactions.save(&super_transaction)
    }

    pub fn delete(&self, id: Uuid) -> AppResult<()> {
        self.repos.super_transactions.delete(id)
    }
}
