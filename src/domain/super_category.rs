use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::IdGenerator;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct SuperCategory {
    pub id: Uuid,
    pub name: String,

    pub budget_id: Uuid,
}

impl SuperCategory {
    pub fn new<N: Into<String>>(name: N, budget_id: Uuid) -> Self {
        Self {
            id: IdGenerator::new_id(),
            name: name.into(),
            budget_id,
        }
    }

    pub fn rename<N: Into<String>>(&mut self, name: N) {
        self.name = name.into();
    }
}
