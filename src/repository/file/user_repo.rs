use std::path::PathBuf;

use crate::{
    User,
    repository::{file::file_helper::FileHelper, traits::UserRepository},
    util::error::AppResult,
};

pub struct FileUserRepo {
    path: PathBuf,
    data: Vec<User>,
}

impl FileUserRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let data = FileHelper::load_from_file(&path)?;
        Ok(Self { path, data })
    }

    pub fn persist(&self) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, &self.data)
    }
}

impl UserRepository for FileUserRepo {
    fn list(&self) -> Vec<User> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<User> {
        self.data.iter().find(|&u| u.id == id).cloned()
    }

    fn save(&mut self, user: User) -> AppResult<()> {
        if let Some(existing) = self.data.iter_mut().find(|u| u.id == user.id) {
            *existing = user;
        } else {
            self.data.push(user);
        }

        self.persist()
    }

    fn delete(&mut self, id: uuid::Uuid) -> AppResult<()> {
        self.data.retain(|u| u.id != id);
        self.persist()
    }
}
