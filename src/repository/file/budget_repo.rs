use crate::{
    Budget,
    repository::{file::file_helper::FileHelper, traits::BudgetRepository},
};

pub struct FileBudgetRepo {
    path: String,
    data: Vec<Budget>,
}

impl FileBudgetRepo {
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

impl BudgetRepository for FileBudgetRepo {
    fn list(&self) -> Vec<Budget> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<Budget> {
        self.data.iter().cloned().find(|b| b.id == id)
    }

    fn save(&mut self, budget: Budget) {
        if let Some(existing) = self.data.iter_mut().find(|b| b.id == budget.id) {
            *existing = budget;
        } else {
            self.data.push(budget);
        }

        self.persist();
    }

    fn delete(&mut self, id: uuid::Uuid) {
        self.data.retain(|b| b.id != id);
        self.persist();
    }
}
