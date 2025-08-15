use crate::{
    Category,
    repository::{file::file_helper::FileHelper, traits::CategoryRepository},
};

pub struct FileCategoryRepo {
    path: String,
    data: Vec<Category>,
}

impl FileCategoryRepo {
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

impl CategoryRepository for FileCategoryRepo {
    fn list(&self) -> Vec<Category> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<Category> {
        self.data.iter().cloned().find(|c| c.id == id)
    }

    fn save(&mut self, category: Category) {
        if let Some(existing) = self.data.iter_mut().find(|c| c.id == category.id) {
            *existing = category;
        } else {
            self.data.push(category);
        }

        self.persist();
    }

    fn delete(&mut self, id: uuid::Uuid) {
        self.data.retain(|c| c.id != id);
        self.persist();
    }
}
