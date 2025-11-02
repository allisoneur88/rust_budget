use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Currency,
    app::repositories::Repositories,
    util::error::{AppError, AppResult},
};

pub struct CurrencyController {
    repos: Arc<Repositories>,
}

impl CurrencyController {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub fn get_all(&self, user_id: Uuid) -> AppResult<Vec<Currency>> {
        self.repos.currencies.list(user_id)
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<Currency> {
        self.repos
            .currencies
            .get(id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Currency",
                id,
            })
    }

    pub fn create<C, S, N>(&self, user_id: Uuid, code: C, symbol: S, name: N) -> AppResult<Currency>
    where
        C: Into<String>,
        S: Into<String>,
        N: Into<String>,
    {
        let currency = Currency::new(user_id, code, symbol, name);
        self.repos.currencies.save(&currency)?;
        Ok(currency)
    }

    pub fn update_currency<C, S, N>(
        &self,
        id: Uuid,
        new_code: C,
        new_symbol: S,
        new_name: N,
    ) -> AppResult<()>
    where
        C: Into<String>,
        S: Into<String>,
        N: Into<String>,
    {
        let mut currency = self.get_by_id(id)?;
        currency.update_currency(new_code, new_symbol, new_name);
        self.repos.currencies.save(&currency)
    }
}
