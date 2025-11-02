use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Assignment,
    app::repositories::Repositories,
    domain::assignment::YearMonth,
    util::error::{AppError, AppResult},
};

pub struct AssignmentController {
    repos: Arc<Repositories>,
}

impl AssignmentController {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub fn get_all(&self, category_id: Uuid) -> AppResult<Vec<Assignment>> {
        self.repos.assigments.list(category_id)
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<Assignment> {
        self.repos
            .assigments
            .get(id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Assignment",
                id,
            })
    }

    pub fn create(
        &self,
        amount: f64,
        year_month: YearMonth,
        category_id: Uuid,
    ) -> AppResult<Assignment> {
        let assignment = Assignment::new(amount, year_month, category_id);
        self.repos.assigments.save(&assignment)?;
        Ok(assignment)
    }

    pub fn update_ammount(&self, id: Uuid, new_amount: f64) -> AppResult<()> {
        let mut assignment = self.get_by_id(id)?;
        assignment.update_amount(new_amount);
        self.repos.assigments.save(&assignment)
    }
}
