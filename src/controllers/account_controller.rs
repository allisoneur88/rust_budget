use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Account, AccountType,
    app::repositories::Repositories,
    util::error::{AppError, AppResult},
};

pub struct AccountController {
    repos: Arc<Repositories>,
}

impl AccountController {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub fn get_all(&self, budget_id: Uuid) -> AppResult<Vec<Account>> {
        self.repos.accounts.list(budget_id)
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<Account> {
        self.repos
            .accounts
            .get(id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Account",
                id,
            })
    }

    pub fn create<N: Into<String>>(
        &self,
        name: N,
        is_off_budget: bool,
        acc_type: AccountType,
        currency_id: Uuid,
        budget_id: Uuid,
    ) -> AppResult<Account> {
        self.repos
            .budgets
            .get(budget_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Budget",
                id: budget_id,
            })?;
        self.repos
            .currencies
            .get(currency_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Currency",
                id: currency_id,
            })?;
        let account = Account::new(name, is_off_budget, acc_type, currency_id, budget_id);
        self.repos.accounts.save(&account)?;
        Ok(account)
    }

    pub fn rename<N: Into<String>>(&self, id: Uuid, new_name: N) -> AppResult<()> {
        let mut account = self.get_by_id(id)?;
        account.rename(new_name);
        self.repos.accounts.save(&account)
    }

    pub fn change_type(&self, id: Uuid, acc_type: AccountType) -> AppResult<()> {
        let mut account = self.get_by_id(id)?;
        account.change_account_type(acc_type);
        self.repos.accounts.save(&account)
    }

    pub fn set_off_budget(&self, id: Uuid, value: bool) -> AppResult<()> {
        let mut account = self.get_by_id(id)?;
        account.set_is_off_budget(value);
        self.repos.accounts.save(&account)
    }

    pub fn delete(&self, id: Uuid) -> AppResult<()> {
        self.repos.accounts.delete(id)
    }
}
