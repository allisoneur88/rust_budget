use std::path::PathBuf;

use crate::{
    Account, Budget,
    repository::{file::file_helper::FileHelper, traits::AccountRepository},
    util::error::AppResult,
};

pub struct FileAccountRepo {
    path: PathBuf,
    data: Vec<Account>,
}

impl FileAccountRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let data = FileHelper::load_from_file(&path)?;
        Ok(Self { path, data })
    }

    pub fn persist(&self) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, &self.data)
    }
}

impl AccountRepository for FileAccountRepo {
    fn list(&self, budget: &Budget) -> Vec<Account> {
        let mut data = self.data.clone();
        data.retain(|a| a.budget_id == budget.id);
        data
    }

    fn get(&self, budget: &Budget, id: uuid::Uuid) -> Option<Account> {
        let mut data = self.data.clone();
        data.retain(|a| a.budget_id == budget.id);
        data.iter().find(|&a| a.id == id).cloned()
    }

    fn save(&mut self, account: Account) -> AppResult<()> {
        if let Some(existing) = self.data.iter_mut().find(|a| a.id == account.id) {
            *existing = account;
        } else {
            self.data.push(account);
        }

        self.persist()
    }

    fn delete(&mut self, id: uuid::Uuid) -> AppResult<()> {
        self.data.retain(|a| a.id != id);
        self.persist()
    }
}
