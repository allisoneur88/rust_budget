use std::{cell::RefCell, rc::Rc};

use crate::{app::app_state::AppState, services::account_service::AccountService};

pub struct AccountController {
    pub account_service: AccountService,
    pub app_state: Rc<RefCell<AppState>>,
}

impl AccountController {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            account_service: AccountService::new(),
            app_state,
        }
    }
}
