use std::path::PathBuf;

use uuid::Uuid;

use crate::{
    Budget, SuperTransaction,
    repository::{file::file_helper::FileHelper, traits::SuperTransactionRepository},
    util::error::AppResult,
};

pub struct FileSuperTransactionRepo {
    path: PathBuf,
}

impl FileSuperTransactionRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        if !path.exists() {
            FileHelper::save_to_file(&path, &Vec::<SuperTransaction>::new())?;
        }
        Ok(Self { path })
    }

    fn read_all(&self) -> AppResult<Vec<SuperTransaction>> {
        FileHelper::load_from_file(&self.path)
    }

    fn write_all(&self, super_transactions: &Vec<SuperTransaction>) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, super_transactions)
    }
}

impl SuperTransactionRepository for FileSuperTransactionRepo {
    fn list(&self, budget_id: Uuid) -> AppResult<Vec<SuperTransaction>> {
        let all = self.read_all()?;
        Ok(all
            .into_iter()
            .filter(|st| st.budget_id == budget_id)
            .collect())
    }

    fn get(&self, id: Uuid) -> AppResult<Option<SuperTransaction>> {
        let all = self.read_all()?;
        Ok(all.into_iter().find(|st| st.id == id))
    }

    fn save(&self, super_transaction: &SuperTransaction) -> AppResult<()> {
        let mut all = self.read_all()?;
        if let Some(existing) = all.iter_mut().find(|st| st.id == super_transaction.id) {
            *existing = super_transaction.clone();
        } else {
            all.push(super_transaction.clone());
        }
        self.write_all(&all)
    }

    fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut all = self.read_all()?;
        all.retain(|st| st.id != id);
        self.write_all(&all)
    }
}
