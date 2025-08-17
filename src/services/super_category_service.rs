use crate::{Budget, IdGenerator, SuperCategory};

#[derive(Debug)]
pub struct SuperCategoryService;

impl SuperCategoryService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_super_category(&self, name: &str, budget: &Budget) -> SuperCategory {
        SuperCategory {
            id: IdGenerator::new_id(),
            name: name.to_string(),
            budget_id: budget.id,
        }
    }

    pub fn update_name(&self, super_category: &mut SuperCategory, name: &str) {
        super_category.name = name.to_string();
    }
}
