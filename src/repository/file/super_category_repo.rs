use std::path::PathBuf;

use uuid::Uuid;

use crate::{
    SuperCategory,
    repository::{file::file_helper::FileHelper, traits::SuperCategoryRepository},
    util::error::AppResult,
};

pub struct FileSuperCategoryRepo {
    path: PathBuf,
}

impl FileSuperCategoryRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        if !path.exists() {
            FileHelper::save_to_file(&path, &Vec::<SuperCategory>::new())?;
        }
        Ok(Self { path })
    }

    fn read_all(&self) -> AppResult<Vec<SuperCategory>> {
        FileHelper::load_from_file(&self.path)
    }

    fn write_all(&self, super_categories: &Vec<SuperCategory>) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, super_categories)
    }
}

impl SuperCategoryRepository for FileSuperCategoryRepo {
    fn list(&self, budget_id: Uuid) -> AppResult<Vec<SuperCategory>> {
        let all = self.read_all()?;
        Ok(all
            .into_iter()
            .filter(|sc| sc.budget_id == budget_id)
            .collect())
    }

    fn get(&self, id: Uuid) -> AppResult<Option<SuperCategory>> {
        let all = self.read_all()?;
        Ok(all.into_iter().find(|sc| sc.id == id))
    }

    fn save(&self, super_category: &SuperCategory) -> AppResult<()> {
        let mut all = self.read_all()?;
        if let Some(exisiting) = all.iter_mut().find(|sc| sc.id == super_category.id) {
            *exisiting = super_category.clone();
        } else {
            all.push(super_category.clone());
        }
        self.write_all(&all)
    }

    fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut all = self.read_all()?;
        all.retain(|sc| sc.id != id);
        self.write_all(&all)
    }
}
