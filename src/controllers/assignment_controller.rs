use std::sync::Arc;

use crate::app::repositories::Repositories;

pub struct AssignmentController {
    repos: Arc<Repositories>,
}

impl AssignmentController {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }
}
