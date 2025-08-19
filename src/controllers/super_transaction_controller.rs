use std::{cell::RefCell, rc::Rc};

use crate::{
    app::app_state::AppState, services::super_transaction_service::SuperTransactionService,
};

pub struct SuperTransactionController {
    pub super_transaction_service: SuperTransactionService,
    pub app_state: Rc<RefCell<AppState>>,
}

impl SuperTransactionController {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            super_transaction_service: SuperTransactionService::new(),
            app_state,
        }
    }
}
