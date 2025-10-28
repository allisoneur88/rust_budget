use std::path::PathBuf;

use crate::{
    Currency,
    repository::{file::file_helper::FileHelper, traits::CurrencyRepository},
    util::error::AppResult,
};

pub struct FileCurrencyRepo {
    path: PathBuf,
    data: Vec<Currency>,
}

impl FileCurrencyRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let data = FileHelper::load_from_file(&path)?;
        Ok(Self { path, data })
    }

    pub fn persist(&self) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, &self.data)
    }
}

impl CurrencyRepository for FileCurrencyRepo {
    fn list(&self) -> Vec<Currency> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<Currency> {
        self.data.iter().find(|&c| c.id == id).cloned()
    }

    fn save(&mut self, currency: Currency) -> AppResult<()> {
        if let Some(existing) = self.data.iter_mut().find(|c| c.id == currency.id) {
            *existing = currency;
        } else {
            self.data.push(currency);
        }

        self.persist()
    }

    fn delete(&mut self, id: uuid::Uuid) -> AppResult<()> {
        self.data.retain(|c| c.id != id);
        self.persist()
    }
}
