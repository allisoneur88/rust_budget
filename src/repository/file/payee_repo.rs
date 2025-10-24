use std::path::PathBuf;

use crate::{
    Payee,
    repository::{file::file_helper::FileHelper, traits::PayeeRepository},
};

pub struct FilePayeeRepo {
    path: PathBuf,
    data: Vec<Payee>,
}

impl FilePayeeRepo {
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

impl PayeeRepository for FilePayeeRepo {
    fn list(&self) -> Vec<Payee> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<Payee> {
        self.data.iter().find(|&p| p.id == id).cloned()
    }

    fn save(&mut self, payee: Payee) {
        if let Some(existing) = self.data.iter_mut().find(|p| p.id == payee.id) {
            *existing = payee;
        } else {
            self.data.push(payee);
        }

        self.persist();
    }

    fn delete(&mut self, id: uuid::Uuid) {
        self.data.retain(|p| p.id != id);
        self.persist();
    }
}
