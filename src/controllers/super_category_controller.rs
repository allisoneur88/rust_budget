use std::{cell::RefCell, rc::Rc};

use crate::{app::app_state::AppState, services::super_category_service::SuperCategoryService};

pub struct SuperCategoryController {
    pub super_category_service: SuperCategoryService,
    pub app_state: Rc<RefCell<AppState>>,
}

impl SuperCategoryController {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            super_category_service: SuperCategoryService::new(),
            app_state,
        }
    }
}
