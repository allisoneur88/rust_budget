use std::sync::Arc;

use uuid::Uuid;

use crate::{
    SuperCategory,
    app::repositories::Repositories,
    util::error::{AppError, AppResult},
};

pub struct SuperCategoryController {
    repos: Arc<Repositories>,
}

impl SuperCategoryController {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub fn get_all(&self, budget_id: Uuid) -> AppResult<Vec<SuperCategory>> {
        self.repos.super_categories.list(budget_id)
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<SuperCategory> {
        self.repos
            .super_categories
            .get(id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Super Category",
                id: id,
            })
    }

    pub fn create<N: Into<String>>(&self, name: N, budget_id: Uuid) -> AppResult<()> {
        let super_category = SuperCategory::new(name, budget_id);
        self.repos.super_categories.save(&super_category)?;
        Ok(())
    }

    pub fn rename<N: Into<String>>(&self, id: Uuid, new_name: N) -> AppResult<()> {
        let mut super_category = self.get_by_id(id)?;
        super_category.rename(new_name);
        self.repos.super_categories.save(&super_category)?;
        Ok(())
    }

    pub fn delete(&self, id: Uuid) -> AppResult<()> {
        self.repos.super_categories.delete(id)
    }
}
