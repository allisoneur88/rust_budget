use std::path::PathBuf;

use crate::{
    Account,
    repository::{file::file_helper::FileHelper, traits::AccountRepository},
};

pub struct FileAccountRepo {
    path: PathBuf,
    data: Vec<Account>,
}

impl FileAccountRepo {
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

impl AccountRepository for FileAccountRepo {
    fn list(&self) -> Vec<Account> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<Account> {
        self.data.iter().cloned().find(|a| a.id == id)
    }

    fn save(&mut self, account: Account) {
        if let Some(existing) = self.data.iter_mut().find(|a| a.id == account.id) {
            *existing = account;
        } else {
            self.data.push(account);
        }

        self.persist();
    }

    fn delete(&mut self, id: uuid::Uuid) {
        self.data.retain(|a| a.id != id);
        self.persist();
    }
}
