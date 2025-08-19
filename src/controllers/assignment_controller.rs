use std::{cell::RefCell, rc::Rc};

use crate::{app::app_state::AppState, services::assignment_service::AssignmentService};

pub struct AssignmentController {
    pub assignment_service: AssignmentService,
    pub app_state: Rc<RefCell<AppState>>,
}

impl AssignmentController {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            assignment_service: AssignmentService::new(),
            app_state,
        }
    }
}
