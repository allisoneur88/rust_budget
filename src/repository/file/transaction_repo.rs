use std::path::PathBuf;

use crate::{
    Transaction,
    repository::{file::file_helper::FileHelper, traits::TransactionRepository},
    util::error::AppResult,
};

pub struct FileTransactionRepo {
    path: PathBuf,
    data: Vec<Transaction>,
}

impl FileTransactionRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let data = FileHelper::load_from_file(&path)?;
        Ok(Self { path, data })
    }

    pub fn persist(&self) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, &self.data)
    }
}

impl TransactionRepository for FileTransactionRepo {
    fn list(&self) -> Vec<Transaction> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<Transaction> {
        self.data.iter().find(|&t| t.id == id).cloned()
    }

    fn save(&mut self, transaction: Transaction) -> AppResult<()> {
        if let Some(existing) = self.data.iter_mut().find(|t| t.id == transaction.id) {
            *existing = transaction;
        } else {
            self.data.push(transaction);
        }

        self.persist()
    }

    fn delete(&mut self, id: uuid::Uuid) -> AppResult<()> {
        self.data.retain(|t| t.id != id);
        self.persist()
    }
}
