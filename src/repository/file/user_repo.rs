use std::path::PathBuf;

use uuid::Uuid;

use crate::{
    User,
    repository::{file::file_helper::FileHelper, traits::UserRepository},
    util::error::AppResult,
};

pub struct FileUserRepo {
    path: PathBuf,
}

impl FileUserRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        if !path.exists() {
            FileHelper::save_to_file(&path, &Vec::<User>::new())?;
        }
        Ok(Self { path })
    }

    fn read_all(&self) -> AppResult<Vec<User>> {
        FileHelper::load_from_file(&self.path)
    }

    fn write_all(&self, users: &Vec<User>) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, users)
    }
}

impl UserRepository for FileUserRepo {
    fn list(&self) -> AppResult<Vec<User>> {
        let all = self.read_all()?;
        Ok(all)
    }

    fn get(&self, id: Uuid) -> AppResult<Option<User>> {
        let all = self.read_all()?;
        Ok(all.into_iter().find(|u| u.id == id))
    }

    fn save(&self, user: &User) -> AppResult<()> {
        let mut all = self.read_all()?;
        if let Some(existing) = all.iter_mut().find(|u| u.id == user.id) {
            *existing = user.clone();
        } else {
            all.push(user.clone());
        }
        self.write_all(&all)
    }

    fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut all = self.read_all()?;
        all.retain(|u| u.id != id);
        self.write_all(&all)
    }
}
