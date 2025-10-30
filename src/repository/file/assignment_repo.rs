use std::path::PathBuf;

use uuid::Uuid;

use crate::{
    Assignment,
    repository::{file::file_helper::FileHelper, traits::AssignmentRepository},
    util::error::AppResult,
};

pub struct FileAssignmentRepo {
    path: PathBuf,
}

impl FileAssignmentRepo {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        if !path.exists() {
            FileHelper::save_to_file(&path, &Vec::<Assignment>::new())?;
        }
        Ok(Self { path })
    }

    fn read_all(&self) -> AppResult<Vec<Assignment>> {
        FileHelper::load_from_file(&self.path)
    }

    fn write_all(&self, assignments: &Vec<Assignment>) -> AppResult<()> {
        FileHelper::save_to_file(&self.path, assignments)
    }
}

impl AssignmentRepository for FileAssignmentRepo {
    fn list(&self, category_id: Uuid) -> AppResult<Vec<Assignment>> {
        let all = self.read_all()?;
        Ok(all
            .into_iter()
            .filter(|a| a.category_id == category_id)
            .collect())
    }

    fn get(&self, id: Uuid) -> AppResult<Option<Assignment>> {
        let all = self.read_all()?;
        Ok(all.into_iter().find(|a| a.id == id))
    }

    fn save(&self, assignment: &Assignment) -> AppResult<()> {
        let mut all = self.read_all()?;
        if let Some(existing) = all.iter_mut().find(|a| a.id == assignment.id) {
            *existing = assignment.clone();
        } else {
            all.push(assignment.clone());
        }

        self.write_all(&all)
    }

    fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut all = self.read_all()?;
        all.retain(|a| a.id != id);
        self.write_all(&all)
    }
}
