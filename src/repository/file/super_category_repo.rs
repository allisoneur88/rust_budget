use std::path::PathBuf;

use crate::{
    Budget, SuperCategory,
    repository::{file::file_helper::FileHelper, traits::SuperCategoryRepository},
};

pub struct FileSuperCategoryRepo {
    path: PathBuf,
    data: Vec<SuperCategory>,
}

impl FileSuperCategoryRepo {
    pub fn new(path: PathBuf) -> Self {
        let data = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        Self { path, data }
    }

    pub fn persist(&self) {
        let _ = FileHelper::save_to_file(&self.path, &self.data);
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

    fn save(&mut self, super_category: SuperCategory) {
        if let Some(exisiting) = self.data.iter_mut().find(|sc| sc.id == super_category.id) {
            *exisiting = super_category;
        } else {
            self.data.push(super_category);
        }

        self.persist();
    }

    fn delete(&mut self, id: uuid::Uuid) {
        self.data.retain(|sc| sc.id != id);
        self.persist();
    }
}
