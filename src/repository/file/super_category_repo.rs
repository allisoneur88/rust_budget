use crate::{
    SuperCategory,
    repository::{file::file_helper::FileHelper, traits::SuperCategoryRepository},
};

pub struct FileSuperCategoryRepo {
    path: String,
    data: Vec<SuperCategory>,
}

impl FileSuperCategoryRepo {
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

impl SuperCategoryRepository for FileSuperCategoryRepo {
    fn list(&self) -> Vec<SuperCategory> {
        self.data.clone()
    }

    fn get(&self, id: uuid::Uuid) -> Option<SuperCategory> {
        self.data.iter().cloned().find(|sc| sc.id == id)
    }

    fn save(&mut self, super_category: SuperCategory) {
        if let Some(exisiting) = self.data.iter_mut().find(|sc| sc.id == super_category.id) {
            *exisiting = super_category;
        } else {
            self.data.push(super_category);
        }

        self.persist();
    }

    fn delete(&mut self, id: uuid::Uuid) {
        self.data.retain(|sc| sc.id != id);
        self.persist();
    }
}
