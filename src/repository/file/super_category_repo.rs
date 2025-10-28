use std::path::PathBuf;

use crate::{
    Budget, SuperCategory,
    repository::{file::file_helper::FileHelper, traits::SuperCategoryRepository},
    util::error::AppResult,
};

pub struct FileSuperCategoryRepo {
    path: PathBuf,
    data: Vec<SuperCategory>,
}

impl FileSuperCategoryRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let data = FileHelper::load_from_file(&path)?;
        Ok(Self { path, data })
    }

    pub fn persist(&self) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, &self.data)
    }
}

impl SuperCategoryRepository for FileSuperCategoryRepo {
    fn list(&self, budget: &Budget) -> Vec<SuperCategory> {
        let mut data = self.data.clone();
        data.retain(|sc| sc.budget_id == budget.id);
        data
    }

    fn get(&self, budget: &Budget, id: uuid::Uuid) -> Option<SuperCategory> {
        let mut data = self.data.clone();
        data.retain(|sc| sc.budget_id == budget.id);
        data.iter().find(|&sc| sc.id == id).cloned()
    }

    fn save(&mut self, super_category: SuperCategory) -> AppResult<()> {
        if let Some(exisiting) = self.data.iter_mut().find(|sc| sc.id == super_category.id) {
            *exisiting = super_category;
        } else {
            self.data.push(super_category);
        }

        self.persist()
    }

    fn delete(&mut self, id: uuid::Uuid) -> AppResult<()> {
        self.data.retain(|sc| sc.id != id);
        self.persist()
    }
}
