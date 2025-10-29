use std::path::PathBuf;

use uuid::Uuid;

use crate::{
    Budget,
    repository::{file::file_helper::FileHelper, traits::BudgetRepository},
    util::error::AppResult,
};

pub struct FileBudgetRepo {
    path: PathBuf,
}

impl FileBudgetRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        if !path.exists() {
            FileHelper::save_to_file(&path, &Vec::<Budget>::new())?;
        }
        Ok(Self { path })
    }

    fn read_all(&self) -> AppResult<Vec<Budget>> {
        FileHelper::load_from_file(&self.path)
    }

    fn write_all(&self, budgets: &Vec<Budget>) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, budgets)
    }
}

impl BudgetRepository for FileBudgetRepo {
    fn list(&self, user_id: Uuid) -> AppResult<Vec<Budget>> {
        let all = self.read_all()?;
        Ok(all.into_iter().filter(|b| b.user_id == user_id).collect())
    }

    fn get(&self, id: Uuid) -> AppResult<Option<Budget>> {
        let all = self.read_all()?;
        Ok(all.into_iter().find(|b| b.id == id))
    }

    fn save(&self, budget: &Budget) -> AppResult<()> {
        let mut all = self.read_all()?;
        if let Some(existing) = all.iter_mut().find(|b| b.id == budget.id) {
            *existing = budget.clone();
        } else {
            all.push(budget.clone());
        }
        self.write_all(&all)
    }

    fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut all = self.read_all()?;
        all.retain(|b| b.id != id);
        self.write_all(&all)
    }
}
