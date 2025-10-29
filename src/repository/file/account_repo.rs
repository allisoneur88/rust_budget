use std::path::PathBuf;

use uuid::Uuid;

use crate::{
    Account, Budget,
    repository::{file::file_helper::FileHelper, traits::AccountRepository},
    util::error::AppResult,
};

pub struct FileAccountRepo {
    path: PathBuf,
}

impl FileAccountRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        if !path.exists() {
            FileHelper::save_to_file(&path, &Vec::<Account>::new())?;
        }
        Ok(Self { path })
    }

    fn read_all(&self) -> AppResult<Vec<Account>> {
        FileHelper::load_from_file(&self.path)
    }

    fn write_all(&self, accounts: &Vec<Account>) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, accounts)
    }
}

impl AccountRepository for FileAccountRepo {
    fn list(&self, budget_id: Uuid) -> AppResult<Vec<Account>> {
        let all = self.read_all()?;
        Ok(all
            .into_iter()
            .filter(|a| a.budget_id == budget_id)
            .collect())
    }

    fn get(&self, id: Uuid) -> AppResult<Option<Account>> {
        let all = self.read_all()?;
        Ok(all.into_iter().find(|a| a.id == id))
    }

    fn save(&self, account: &Account) -> AppResult<()> {
        let mut all = self.read_all()?;
        if let Some(existing) = all.iter_mut().find(|a| a.id == account.id) {
            *existing = account.clone();
        } else {
            all.push(account.clone());
        }

        self.write_all(&all)
    }

    fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut all = self.read_all()?;
        all.retain(|a| a.id != id);
        self.write_all(&all)
    }
}
