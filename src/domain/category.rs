use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::IdGenerator;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub super_category_id: Uuid,
}

impl Category {
    pub fn new<N: Into<String>>(name: N, super_category_id: Uuid) -> Self {
        Self {
            id: IdGenerator::new_id(),
            name: name.into(),
            super_category_id,
        }
    }

    pub fn rename<N: Into<String>>(&mut self, new_name: N) {
        self.name = new_name.into();
    }

    pub fn move_to_another_super_category(&mut self, new_super_category_id: Uuid) {
        self.super_category_id = new_super_category_id;
    }
}
