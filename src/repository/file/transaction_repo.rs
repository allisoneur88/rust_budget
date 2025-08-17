use std::path::PathBuf;

use crate::{
    Transaction,
    repository::{file::file_helper::FileHelper, traits::TransactionRepository},
};

pub struct FileTransactionRepo {
    path: PathBuf,
    data: Vec<Transaction>,
}

impl FileTransactionRepo {
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

impl TransactionRepository for FileTransactionRepo {
    fn list(&self) -> Vec<Transaction> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<Transaction> {
        self.data.iter().cloned().find(|t| t.id == id)
    }

    fn save(&mut self, transaction: Transaction) {
        if let Some(existing) = self.data.iter_mut().find(|t| t.id == transaction.id) {
            *existing = transaction;
        } else {
            self.data.push(transaction);
        }

        self.persist();
    }

    fn delete(&mut self, id: uuid::Uuid) {
        self.data.retain(|t| t.id != id);
        self.persist();
    }
}
