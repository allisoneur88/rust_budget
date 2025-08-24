use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::{Budget, Currency, app::app_state::AppState, services::budget_service::BudgetService};

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

    pub fn get_all(&self) -> Vec<Budget> {
        self.app_state
            .borrow()
            .budgets
            .list(self.app_state.borrow().current_user.as_ref().unwrap())
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<Budget> {
        self.app_state
            .borrow()
            .budgets
            .get(self.app_state.borrow().current_user.as_ref().unwrap(), id)
    }

    pub fn create(&self, name: &str, main_currency: Currency) {
        let budget = self.budget_service.make_budget(
            name,
            main_currency,
            self.app_state.borrow().current_user.as_ref().unwrap(),
        );

        self.app_state.borrow_mut().budgets.save(budget);
    }

    pub fn rename(&self, id: Uuid, new_name: &str) {
        let mut budget = self.get_by_id(id).unwrap();
        self.budget_service
            .update_budget_name(&mut budget, new_name);

        self.app_state.borrow_mut().budgets.save(budget);
    }

    pub fn change_currency(&self, id: Uuid, new_currency: Currency) {
        let mut budget = self.get_by_id(id).unwrap();
        self.budget_service
            .change_main_currency(&mut budget, new_currency);
        self.app_state.borrow_mut().budgets.save(budget);
    }

    pub fn delete(&self, id: Uuid) {
        self.app_state.borrow_mut().budgets.delete(id);
    }
}
