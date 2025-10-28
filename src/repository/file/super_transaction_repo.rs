use std::path::PathBuf;

use crate::{
    Budget, SuperTransaction,
    repository::{file::file_helper::FileHelper, traits::SuperTransactionRepository},
    util::error::AppResult,
};

pub struct FileSuperTransactionRepo {
    path: PathBuf,
    data: Vec<SuperTransaction>,
}

impl FileSuperTransactionRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let data = FileHelper::load_from_file(&path)?;
        Ok(Self { path, data })
    }

    pub fn persist(&self) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, &self.data)
    }
}

impl SuperTransactionRepository for FileSuperTransactionRepo {
    fn list(&self, budget: &Budget) -> Vec<SuperTransaction> {
        let mut data = self.data.clone();
        data.retain(|st| st.budget_id == budget.id);
        data
    }

    fn get(&self, budget: &Budget, id: uuid::Uuid) -> Option<SuperTransaction> {
        let mut data = self.data.clone();
        data.retain(|st| st.budget_id == budget.id);
        data.iter().find(|&st| st.id == id).cloned()
    }

    fn save(&mut self, super_transaction: SuperTransaction) -> AppResult<()> {
        if let Some(existing) = self
            .data
            .iter_mut()
            .find(|st| st.id == super_transaction.id)
        {
            *existing = super_transaction;
        } else {
            self.data.push(super_transaction);
        }

        self.persist()
    }

    fn delete(&mut self, id: uuid::Uuid) -> AppResult<()> {
        self.data.retain(|st| st.id != id);
        self.persist()
    }
}
