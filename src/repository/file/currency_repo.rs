use std::path::PathBuf;

use crate::{
    Currency,
    repository::{file::file_helper::FileHelper, traits::CurrencyRepository},
};

pub struct FileCurrencyRepo {
    path: PathBuf,
    data: Vec<Currency>,
}

impl FileCurrencyRepo {
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

impl CurrencyRepository for FileCurrencyRepo {
    fn list(&self) -> Vec<Currency> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<Currency> {
        self.data.iter().cloned().find(|c| c.id == id)
    }

    fn save(&mut self, currency: Currency) {
        if let Some(existing) = self.data.iter_mut().find(|c| c.id == currency.id) {
            *existing = currency;
        } else {
            self.data.push(currency);
        }

        self.persist();
    }

    fn delete(&mut self, id: uuid::Uuid) {
        self.data.retain(|c| c.id != id);
        self.persist();
    }
}
