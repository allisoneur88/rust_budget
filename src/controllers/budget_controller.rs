use std::sync::Arc;

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

    pub fn get_by_id(&self, id: Uuid) -> AppResult<Budget> {
        self.repos
            .budgets
            .get(id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Budget",
                id,
            })
    }

    pub fn create<N: Into<String>>(
        &self,
        name: N,
        main_currency_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Budget> {
        self.repos
            .currencies
            .get(main_currency_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Currency",
                id: main_currency_id,
            })?;

        let budget = Budget::new(name, main_currency_id, user_id);
        self.repos.budgets.save(&budget)?;
        Ok(budget)
    }

    pub fn rename<N: Into<String>>(&self, id: Uuid, new_name: N) -> AppResult<()> {
        let mut budget = self.get_by_id(id)?;
        budget.rename(new_name);
        self.repos.budgets.save(&budget)
    }

    pub fn change_currency(&self, id: Uuid, new_currency_id: Uuid) -> AppResult<()> {
        let mut budget = self.get_by_id(id)?;
        self.repos
            .currencies
            .get(new_currency_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Currency",
                id: new_currency_id,
            })?;
        budget.change_currency(new_currency_id);
        self.repos.budgets.save(&budget)
    }

    pub fn delete(&self, id: Uuid) -> AppResult<()> {
        self.repos.budgets.delete(id)
    }
}
