use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::{
    Budget,
    app::app_state::AppState,
    services::budget_service::BudgetService,
    util::{
        error::{AppError, AppResult},
        validators::require_user,
    },
};

pub struct BudgetController {
    pub budget_service: BudgetService,
    pub app_state: Rc<RefCell<AppState>>,
}

impl BudgetController {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            budget_service: BudgetService::new(),
            app_state,
        }
    }

    pub fn get_all(&self) -> AppResult<Vec<Budget>> {
        let state = self.app_state.borrow();
        let user = require_user(&state)?;
        Ok(state.budgets.list(user))
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<Budget> {
        let state = self.app_state.borrow();
        let user = require_user(&state)?;
        let budget = state
            .budgets
            .get(user, id)
            .ok_or_else(|| AppError::NotFound {
                entity: "Budget",
                id: id,
            })?;
        drop(state);
        Ok(budget)
    }

    pub fn create(&self, name: &str, main_currency_id: Uuid) -> AppResult<()> {
        let state = self.app_state.borrow();
        let user = require_user(&state)?;
        let main_currency = state
            .currencies
            .get(main_currency_id)
            .ok_or(AppError::NotFound {
                entity: "Currency",
                id: main_currency_id,
            })?;
        let budget = self.budget_service.make_budget(name, &main_currency, user);
        drop(state);
        self.app_state.borrow_mut().budgets.save(budget)
    }

    pub fn rename(&self, id: Uuid, new_name: &str) -> AppResult<()> {
        let state = self.app_state.borrow();
        let user = require_user(&state)?;
        let mut budget = state.budgets.get(user, id).ok_or(AppError::NotFound {
            entity: "Budget",
            id,
        })?;
        drop(state);
        self.budget_service
            .update_budget_name(&mut budget, new_name);
        self.app_state.borrow_mut().budgets.save(budget)
    }

    pub fn change_currency(&self, id: Uuid, new_currency_id: Uuid) -> AppResult<()> {
        let state = self.app_state.borrow();
        let user = require_user(&state)?;
        let mut budget = state.budgets.get(user, id).ok_or(AppError::NotFound {
            entity: "Budget",
            id,
        })?;
        let new_currency = state
            .currencies
            .get(new_currency_id)
            .ok_or(AppError::NotFound {
                entity: "Currency",
                id: new_currency_id,
            })?;
        drop(state);
        self.budget_service
            .change_main_currency(&mut budget, &new_currency);
        self.app_state.borrow_mut().budgets.save(budget)
    }

    pub fn delete(&self, id: Uuid) -> AppResult<()> {
        let state = self.app_state.borrow();
        let user = require_user(&state)?;
        let budget = state.budgets.get(user, id).ok_or(AppError::NotFound {
            entity: "Budget",
            id,
        })?;
        drop(state);
        self.app_state.borrow_mut().budgets.delete(budget.id)
    }
}
