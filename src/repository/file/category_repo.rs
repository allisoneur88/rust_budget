use std::path::PathBuf;

use uuid::Uuid;

use crate::{
    Category,
    repository::{file::file_helper::FileHelper, traits::CategoryRepository},
    util::error::AppResult,
};

pub struct FileCategoryRepo {
    path: PathBuf,
}

impl FileCategoryRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        if !path.exists() {
            FileHelper::save_to_file(&path, &Vec::<Category>::new())?;
        }
        Ok(Self { path })
    }

    fn read_all(&self) -> AppResult<Vec<Category>> {
        FileHelper::load_from_file(&self.path)
    }

    fn write_all(&self, categories: &Vec<Category>) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, categories)
    }
}

impl CategoryRepository for FileCategoryRepo {
    fn list(&self, super_category_id: Uuid) -> AppResult<Vec<Category>> {
        let all = self.read_all()?;
        Ok(all
            .into_iter()
            .filter(|c| c.super_category_id == super_category_id)
            .collect())
    }

    fn get(&self, id: Uuid) -> AppResult<Option<Category>> {
        let all = self.read_all()?;
        Ok(all.into_iter().find(|c| c.id == id))
    }

    fn save(&self, category: &Category) -> AppResult<()> {
        let mut all = self.read_all()?;
        if let Some(existing) = all.iter_mut().find(|c| c.id == category.id) {
            *existing = category.clone();
        } else {
            all.push(category.clone());
        }
        self.write_all(&all)
    }

    fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut all = self.read_all()?;
        all.retain(|c| c.id != id);
        self.write_all(&all)
    }
}
