use std::{cell::RefCell, rc::Rc};

use crate::{app::app_state::AppState, services::budget_service::BudgetService};

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
}
