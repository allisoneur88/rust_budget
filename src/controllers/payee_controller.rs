use std::{cell::RefCell, rc::Rc};

use crate::{app::app_state::AppState, services::payee_service::PayeeService};

pub struct PayeeController {
    pub payee_service: PayeeService,
    pub app_state: Rc<RefCell<AppState>>,
}

impl PayeeController {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            payee_service: PayeeService::new(),
            app_state,
        }
    }
}
