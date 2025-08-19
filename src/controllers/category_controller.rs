use std::{cell::RefCell, rc::Rc};

use crate::{app::app_state::AppState, services::category_service::CategoryService};

pub struct CategoryController {
    pub category_service: CategoryService,
    pub app_state: Rc<RefCell<AppState>>,
}

impl CategoryController {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            category_service: CategoryService::new(),
            app_state,
        }
    }
}
