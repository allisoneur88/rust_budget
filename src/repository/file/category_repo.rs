use std::path::PathBuf;

use crate::{
    Category,
    repository::{file::file_helper::FileHelper, traits::CategoryRepository},
    util::error::AppResult,
};

pub struct FileCategoryRepo {
    path: PathBuf,
    data: Vec<Category>,
}

impl FileCategoryRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let data = FileHelper::load_from_file(&path)?;
        Ok(Self { path, data })
    }

    pub fn persist(&self) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, &self.data)
    }
}

impl CategoryRepository for FileCategoryRepo {
    fn list(&self) -> Vec<Category> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<Category> {
        self.data.iter().find(|&c| c.id == id).cloned()
    }

    fn save(&mut self, category: Category) -> AppResult<()> {
        if let Some(existing) = self.data.iter_mut().find(|c| c.id == category.id) {
            *existing = category;
        } else {
            self.data.push(category);
        }

        self.persist()
    }

    fn delete(&mut self, id: uuid::Uuid) -> AppResult<()> {
        self.data.retain(|c| c.id != id);
        self.persist()
    }
}
