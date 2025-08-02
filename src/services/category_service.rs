use crate::{Category, IdGenerator, SuperCategory};

#[derive(Debug)]
pub struct CategoryService;

impl CategoryService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_category(&self, name: &str, super_category: &SuperCategory) -> Category {
        Category {
            id: IdGenerator::new_id(),
            name: name.to_string(),
            super_category_id: super_category.id,
        }
    }

    pub fn update_name(&self, category: &mut Category, new_name: &str) {
        category.name = new_name.to_string();
    }

    pub fn move_to_another_super_category(
        &self,
        category: &mut Category,
        new_super_category: &Category,
    ) {
        category.super_category_id = new_super_category.id;
    }
}
