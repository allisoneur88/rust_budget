use std::{cell::RefCell, rc::Rc};

use crate::{app::app_state::AppState, services::transaction_service::TransactonService};

pub struct TransactionController {
    pub transaction_service: TransactonService,
    pub app_state: Rc<RefCell<AppState>>,
}

impl TransactionController {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            transaction_service: TransactonService::new(),
            app_state,
        }
    }
}
