use std::path::PathBuf;

use crate::{
    Budget, User,
    repository::{file::file_helper::FileHelper, traits::BudgetRepository},
    util::error::AppResult,
};

pub struct FileBudgetRepo {
    path: PathBuf,
    data: Vec<Budget>,
}

impl FileBudgetRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let data = FileHelper::load_from_file(&path)?;
        Ok(Self { path, data })
    }

    pub fn persist(&self) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, &self.data)
    }
}

impl BudgetRepository for FileBudgetRepo {
    fn list(&self, user: &User) -> Vec<Budget> {
        let mut data = self.data.clone();
        data.retain(|b| b.user_id == user.id);
        data
    }

    fn get(&self, user: &User, id: uuid::Uuid) -> Option<Budget> {
        let mut data = self.data.clone();
        data.retain(|b| b.user_id == user.id);
        data.iter().find(|&b| b.id == id).cloned()
    }

    fn save(&mut self, budget: Budget) -> AppResult<()> {
        if let Some(existing) = self.data.iter_mut().find(|b| b.id == budget.id) {
            *existing = budget;
        } else {
            self.data.push(budget);
        }

        self.persist()
    }

    fn delete(&mut self, id: uuid::Uuid) -> AppResult<()> {
        self.data.retain(|b| b.id != id);
        self.persist()
    }
}
