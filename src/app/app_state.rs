use std::env;

use uuid::Uuid;

use crate::util::error::{AppError, AppResult};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct AppState {
    pub version: &'static str,

    pub current_user_id: Option<Uuid>,
    pub current_budget_id: Option<Uuid>,
    pub current_account_id: Option<Uuid>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            version: VERSION,
            current_user_id: None,
            current_budget_id: None,
            current_account_id: None,
        }
    }

    pub fn login(&mut self, user_id: Uuid) {
        self.current_user_id = Some(user_id);

        self.current_budget_id = None;
        self.current_account_id = None;
    }

    pub fn logout(&mut self) {
        *self = Self::new();
    }

    pub fn select_budget(&mut self, budget_id: Uuid) {
        self.current_budget_id = Some(budget_id);
        self.current_account_id = None;
    }

    pub fn select_account(&mut self, account_id: Uuid) {
        self.current_account_id = Some(account_id);
    }

    pub fn require_user(&self) -> AppResult<Uuid> {
        self.current_user_id.ok_or(AppError::NotLoggedIn)
    }

    pub fn require_budget(&self) -> AppResult<Uuid> {
        self.current_budget_id.ok_or(AppError::NoBudgetSelected)
    }

    pub fn require_account(&self) -> AppResult<Uuid> {
        self.current_account_id.ok_or(AppError::NoAccountSelected)
    }
}
