use crate::{Category, IdGenerator};

#[derive(Debug)]
pub struct CategoryService;

impl CategoryService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_category(&self, name: &str) -> Category {
        Category {
            id: IdGenerator::new_id(),
            name: name.to_string(),
        }
    }

    pub fn update_name(&self, category: &mut Category, new_name: &str) {
        category.name = new_name.to_string();
    }
}
