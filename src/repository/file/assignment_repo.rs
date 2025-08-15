use crate::{
    Assignment,
    repository::{file::file_helper::FileHelper, traits::AssignmentRepository},
};

pub struct FileAssignmentRepo {
    path: String,
    data: Vec<Assignment>,
}

impl FileAssignmentRepo {
    pub fn new(path: String) -> Self {
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

impl AssignmentRepository for FileAssignmentRepo {
    fn list(&self) -> Vec<Assignment> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<Assignment> {
        self.data.iter().cloned().find(|a| a.id == id)
    }

    fn save(&mut self, assignment: Assignment) {
        if let Some(existing) = self.data.iter_mut().find(|a| a.id == assignment.id) {
            *existing = assignment;
        } else {
            self.data.push(assignment);
        }

        self.persist();
    }

    fn delete(&mut self, id: uuid::Uuid) {
        self.data.retain(|a| a.id != id);

        self.persist();
    }
}
