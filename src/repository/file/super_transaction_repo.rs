use crate::{
    SuperTransaction,
    repository::{file::file_helper::FileHelper, traits::SuperTransactionRepository},
};

pub struct FileSuperTransactionRepo {
    path: String,
    data: Vec<SuperTransaction>,
}

impl FileSuperTransactionRepo {
    pub fn new(path: String) -> Self {
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

impl SuperTransactionRepository for FileSuperTransactionRepo {
    fn list(&self) -> Vec<SuperTransaction> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<SuperTransaction> {
        self.data.iter().cloned().find(|st| st.id == id)
    }

    fn save(&mut self, super_transaction: SuperTransaction) {
        if let Some(existing) = self
            .data
            .iter_mut()
            .find(|st| st.id == super_transaction.id)
        {
            *existing = super_transaction;
        } else {
            self.data.push(super_transaction);
        }

        self.persist();
    }

    fn delete(&mut self, id: uuid::Uuid) {
        self.data.retain(|st| st.id != id);
        self.persist();
    }
}
