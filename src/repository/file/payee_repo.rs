use std::path::PathBuf;

use uuid::Uuid;

use crate::{
    Payee,
    repository::{file::file_helper::FileHelper, traits::PayeeRepository},
    util::error::AppResult,
};

pub struct FilePayeeRepo {
    path: PathBuf,
}

impl FilePayeeRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        if !path.exists() {
            FileHelper::save_to_file(&path, &Vec::<Payee>::new())?;
        }
        Ok(Self { path })
    }

    fn read_all(&self) -> AppResult<Vec<Payee>> {
        FileHelper::load_from_file(&self.path)
    }

    fn write_all(&self, payees: &Vec<Payee>) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, payees)
    }
}

impl PayeeRepository for FilePayeeRepo {
    fn list(&self, budget_id: Uuid) -> AppResult<Vec<Payee>> {
        let all = self.read_all()?;
        Ok(all
            .into_iter()
            .filter(|p| p.budget_id == budget_id)
            .collect())
    }

    fn get(&self, id: Uuid) -> AppResult<Option<Payee>> {
        let all = self.read_all()?;
        Ok(all.into_iter().find(|p| p.id == id))
    }

    fn save(&self, payee: &Payee) -> AppResult<()> {
        let mut all = self.read_all()?;
        if let Some(existing) = all.iter_mut().find(|p| p.id == payee.id) {
            *existing = payee.clone();
        } else {
            all.push(payee.clone());
        }
        self.write_all(&all)
    }

    fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut all = self.read_all()?;
        all.retain(|p| p.id != id);
        self.write_all(&all)
    }
}
