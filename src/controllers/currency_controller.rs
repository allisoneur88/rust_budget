use std::{cell::RefCell, rc::Rc};

use crate::{app::app_state::AppState, services::currency_service::CurrencyService};

pub struct CurrencyController {
    pub currency_service: CurrencyService,
    pub app_state: Rc<RefCell<AppState>>,
}

impl CurrencyController {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            currency_service: CurrencyService::new(),
            app_state,
        }
    }
}
