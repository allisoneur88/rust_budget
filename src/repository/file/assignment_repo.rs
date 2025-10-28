use std::path::PathBuf;

use crate::{
    Assignment,
    repository::{file::file_helper::FileHelper, traits::AssignmentRepository},
    util::error::AppResult,
};

pub struct FileAssignmentRepo {
    path: PathBuf,
    data: Vec<Assignment>,
}

impl FileAssignmentRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let data = FileHelper::load_from_file(&path)?;
        Ok(Self { path, data })
    }

    pub fn persist(&self) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, &self.data)
    }
}

impl AssignmentRepository for FileAssignmentRepo {
    fn list(&self) -> Vec<Assignment> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<Assignment> {
        self.data.iter().find(|&a| a.id == id).cloned()
    }

    fn save(&mut self, assignment: Assignment) -> AppResult<()> {
        if let Some(existing) = self.data.iter_mut().find(|a| a.id == assignment.id) {
            *existing = assignment;
        } else {
            self.data.push(assignment);
        }

        self.persist()
    }

    fn delete(&mut self, id: uuid::Uuid) -> AppResult<()> {
        self.data.retain(|a| a.id != id);

        self.persist()
    }
}
