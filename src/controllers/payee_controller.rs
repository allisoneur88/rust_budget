use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Payee,
    app::repositories::Repositories,
    util::error::{AppError, AppResult},
};

pub struct PayeeController {
    repos: Arc<Repositories>,
}

impl PayeeController {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub fn get_all(&self, budget_id: Uuid) -> AppResult<Vec<Payee>> {
        self.repos.payees.list(budget_id)
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<Payee> {
        self.repos
            .payees
            .get(id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Payee",
                id,
            })
    }

    pub fn create<N: Into<String>, M: Into<String>>(
        &self,
        name: N,
        memo: M,
        budget_id: Uuid,
    ) -> AppResult<Payee> {
        let payee = Payee::new(name, memo, budget_id);
        self.repos.payees.save(&payee)?;
        Ok(payee)
    }

    pub fn rename<N: Into<String>>(&self, id: Uuid, new_name: N) -> AppResult<()> {
        let mut payee = self.get_by_id(id)?;
        payee.rename(new_name);
        self.repos.payees.save(&payee)
    }

    pub fn update_memo<M: Into<String>>(&self, id: Uuid, new_memo: M) -> AppResult<()> {
        let mut payee = self.get_by_id(id)?;
        payee.update_memo(new_memo);
        self.repos.payees.save(&payee)
    }
}
