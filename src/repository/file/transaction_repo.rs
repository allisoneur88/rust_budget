use std::path::PathBuf;

use uuid::Uuid;

use crate::{
    Transaction,
    repository::{file::file_helper::FileHelper, traits::TransactionRepository},
    util::error::AppResult,
};

pub struct FileTransactionRepo {
    path: PathBuf,
}

impl FileTransactionRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        if !path.exists() {
            FileHelper::save_to_file(&path, &Vec::<Transaction>::new())?;
        }
        Ok(Self { path })
    }

    fn read_all(&self) -> AppResult<Vec<Transaction>> {
        FileHelper::load_from_file(&self.path)
    }

    fn write_all(&self, transactions: &Vec<Transaction>) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, transactions)
    }
}

impl TransactionRepository for FileTransactionRepo {
    fn list(&self, super_transaction_id: Uuid) -> AppResult<Vec<Transaction>> {
        let all = self.read_all()?;
        Ok(all
            .into_iter()
            .filter(|t| t.super_transaction_id == super_transaction_id)
            .collect())
    }

    fn get(&self, id: Uuid) -> AppResult<Option<Transaction>> {
        let all = self.read_all()?;
        Ok(all.into_iter().find(|t| t.id == id))
    }

    fn save(&self, transaction: &Transaction) -> AppResult<()> {
        let mut all = self.read_all()?;
        if let Some(existing) = all.iter_mut().find(|t| t.id == transaction.id) {
            *existing = transaction.clone();
        } else {
            all.push(transaction.clone());
        }
        self.write_all(&all)
    }

    fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut all = self.read_all()?;
        all.retain(|t| t.id != id);
        self.write_all(&all)
    }
}
