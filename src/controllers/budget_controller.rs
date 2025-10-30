use std::{cell::RefCell, rc::Rc, sync::Arc};

use uuid::Uuid;

use crate::{
    Budget,
    app::repositories::Repositories,
    util::error::{AppError, AppResult},
};

pub struct BudgetController {
    pub repos: Arc<Repositories>,
}

impl BudgetController {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub fn get_all(&self, user_id: Uuid) -> AppResult<Vec<Budget>> {
        self.repos.budgets.list(user_id)
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<Option<Budget>> {
        self.repos.budgets.get(id)
    }

    pub fn create<N: Into<String>>(
        &self,
        name: N,
        main_currency_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<()> {
        let budget = Budget::new(name, main_currency_id, user_id);
        self.repos.budgets.save(&budget)
    }

    pub fn rename<N: Into<String>>(&self, id: Uuid, new_name: N) -> AppResult<()> {
        match self.repos.budgets.get(id)? {
            Some(mut budget) => {
                budget.rename(new_name);
                self.repos.budgets.save(&budget)?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                entity: "Budget",
                id: id,
            }),
        }
    }

    pub fn change_currency(&self, id: Uuid, new_currency_id: Uuid) -> AppResult<()> {
        match self.repos.budgets.get(id)? {
            Some(mut budget) => {
                budget.change_currency(new_currency_id);
                self.repos.budgets.save(&budget)?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                entity: "Budget",
                id: id,
            }),
        }
    }

    pub fn delete(&self, id: Uuid) -> AppResult<()> {
        match self.repos.budgets.get(id)? {
            Some(budget) => {
                self.repos.budgets.delete(budget.id)?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                entity: "Budget",
                id: id,
            }),
        }
    }
}
