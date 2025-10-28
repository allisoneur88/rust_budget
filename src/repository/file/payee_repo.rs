use std::path::PathBuf;

use crate::{
    Payee,
    repository::{file::file_helper::FileHelper, traits::PayeeRepository},
    util::error::AppResult,
};

pub struct FilePayeeRepo {
    path: PathBuf,
    data: Vec<Payee>,
}

impl FilePayeeRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let data = FileHelper::load_from_file(&path)?;
        Ok(Self { path, data })
    }

    pub fn persist(&self) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, &self.data)
    }
}

impl PayeeRepository for FilePayeeRepo {
    fn list(&self) -> Vec<Payee> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<Payee> {
        self.data.iter().find(|&p| p.id == id).cloned()
    }

    fn save(&mut self, payee: Payee) -> AppResult<()> {
        if let Some(existing) = self.data.iter_mut().find(|p| p.id == payee.id) {
            *existing = payee;
        } else {
            self.data.push(payee);
        }

        self.persist()
    }

    fn delete(&mut self, id: uuid::Uuid) -> AppResult<()> {
        self.data.retain(|p| p.id != id);
        self.persist()
    }
}
