use crate::{Category, IdGenerator, SuperCategory, domain::super_category};

#[derive(Debug)]
pub struct SuperCategoryService;

impl SuperCategoryService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_super_category(&self, name: &str) -> SuperCategory {
        SuperCategory {
            id: IdGenerator::new_id(),
            name: name.to_string(),
            categories: None,
        }
    }

    pub fn update_name(&self, super_category: &mut SuperCategory, name: &str) {
        super_category.name = name.to_string();
    }

    pub fn add_category(&self, super_category: &mut SuperCategory, category: Category) {
        match &mut super_category.categories {
            None => super_category.categories = Some(vec![category]),
            Some(categories) => categories.push(category),
        }
    }
}
