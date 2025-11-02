use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Category,
    app::repositories::Repositories,
    util::error::{AppError, AppResult},
};

pub struct CategoryController {
    repos: Arc<Repositories>,
}

impl CategoryController {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub fn get_all(&self, super_category_id: Uuid) -> AppResult<Vec<Category>> {
        self.repos.categories.list(super_category_id)
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<Category> {
        self.repos
            .categories
            .get(id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Category",
                id,
            })
    }

    pub fn create<N: Into<String>>(&self, name: N, super_category_id: Uuid) -> AppResult<Category> {
        self.repos
            .super_categories
            .get(super_category_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Super Category",
                id: super_category_id,
            })?;
        let category = Category::new(name, super_category_id);
        self.repos.categories.save(&category)?;
        Ok(category)
    }

    pub fn rename<N: Into<String>>(&self, id: Uuid, new_name: N) -> AppResult<()> {
        let mut category = self.get_by_id(id)?;
        category.rename(new_name);
        self.repos.categories.save(&category)
    }

    pub fn move_to_another_super_category(
        &self,
        id: Uuid,
        new_super_category_id: Uuid,
    ) -> AppResult<()> {
        let mut category = self.get_by_id(id)?;
        self.repos
            .super_categories
            .get(new_super_category_id)?
            .ok_or_else(|| AppError::NotFound {
                entity: "Super Category",
                id: new_super_category_id,
            })?;
        category.move_to_another_super_category(new_super_category_id);
        self.repos.categories.save(&category)
    }
}
