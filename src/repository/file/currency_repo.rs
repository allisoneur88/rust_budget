use std::path::PathBuf;

use uuid::Uuid;

use crate::{
    Currency,
    repository::{file::file_helper::FileHelper, traits::CurrencyRepository},
    util::error::AppResult,
};

pub struct FileCurrencyRepo {
    path: PathBuf,
}

impl FileCurrencyRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        if !path.exists() {
            FileHelper::save_to_file(&path, &Vec::<Currency>::new())?;
        }
        Ok(Self { path })
    }

    fn read_all(&self) -> AppResult<Vec<Currency>> {
        FileHelper::load_from_file(&self.path)
    }

    fn write_all(&self, currencies: &Vec<Currency>) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, currencies)
    }
}

impl CurrencyRepository for FileCurrencyRepo {
    fn list(&self, user_id: Uuid) -> AppResult<Vec<Currency>> {
        let all = self.read_all()?;
        Ok(all.into_iter().filter(|c| c.user_id == user_id).collect())
    }

    fn get(&self, id: Uuid) -> AppResult<Option<Currency>> {
        let all = self.read_all()?;
        Ok(all.into_iter().find(|c| c.id == id))
    }

    fn save(&self, currency: &Currency) -> AppResult<()> {
        let mut all = self.read_all()?;
        if let Some(existing) = all.iter_mut().find(|c| c.id == currency.id) {
            *existing = currency.clone();
        } else {
            all.push(currency.clone());
        }
        self.write_all(&all)
    }

    fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut all = self.read_all()?;
        all.retain(|c| c.id != id);
        self.write_all(&all)
    }
}
