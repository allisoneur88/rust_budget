use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Account, AccountType,
    app::repositories::Repositories,
    util::error::{AppError, AppResult},
};

pub struct AccountController {
    pub repos: Arc<Repositories>,
}

impl AccountController {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub fn get_all(&self, budget_id: Uuid) -> AppResult<Vec<Account>> {
        self.repos.accounts.list(budget_id)
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<Option<Account>> {
        self.repos.accounts.get(id)
    }

    pub fn create<N: Into<String>>(
        &self,
        name: N,
        is_off_budget: bool,
        acc_type: AccountType,
        currency_id: Uuid,
        budget_id: Uuid,
    ) -> AppResult<()> {
        let account = Account::new(name, is_off_budget, acc_type, currency_id, budget_id);
        self.repos.accounts.save(&account)
    }

    pub fn rename<N: Into<String>>(&self, id: Uuid, new_name: N) -> AppResult<()> {
        match self.repos.accounts.get(id)? {
            Some(mut account) => {
                account.rename(new_name);
                self.repos.accounts.save(&account);
                Ok(())
            }
            None => Err(AppError::NotFound {
                entity: "Account",
                id: id,
            }),
        }
    }

    pub fn change_type(&self, id: Uuid, acc_type: AccountType) -> AppResult<()> {
        match self.repos.accounts.get(id)? {
            Some(mut account) => {
                account.change_account_type(acc_type);
                self.repos.accounts.save(&account);
                Ok(())
            }
            None => Err(AppError::NotFound {
                entity: "Account",
                id: id,
            }),
        }
    }

    pub fn set_off_budget(&self, id: Uuid, value: bool) -> AppResult<()> {
        match self.repos.accounts.get(id)? {
            Some(mut account) => {
                account.set_is_off_budget(value);
                self.repos.accounts.save(&account);
                Ok(())
            }
            None => Err(AppError::NotFound {
                entity: "Account",
                id: id,
            }),
        }
    }

    pub fn delete(&self, id: Uuid) -> AppResult<()> {
        match self.repos.accounts.get(id)? {
            Some(account) => {
                self.repos.accounts.delete(account.id);
                Ok(())
            }
            None => Err(AppError::NotFound {
                entity: "Account",
                id: id,
            }),
        }
    }
}
