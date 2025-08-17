use std::path::PathBuf;

use crate::{
    User,
    repository::{file::file_helper::FileHelper, traits::UserRepository},
};

pub struct FileUserRepo {
    path: PathBuf,
    data: Vec<User>,
}

impl FileUserRepo {
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

impl UserRepository for FileUserRepo {
    fn list(&self) -> Vec<User> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<User> {
        self.data.iter().cloned().find(|u| u.id == id)
    }

    fn save(&mut self, user: User) {
        if let Some(existing) = self.data.iter_mut().find(|u| u.id == user.id) {
            *existing = user;
        } else {
            self.data.push(user);
        }

        self.persist();
    }

    fn delete(&mut self, id: uuid::Uuid) {
        self.data.retain(|u| u.id != id);
        self.persist();
    }
}
