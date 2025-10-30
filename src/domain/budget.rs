use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::IdGenerator;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub main_currency_id: Uuid,
    pub user_id: Uuid,
}

impl Budget {
    pub fn new<N: Into<String>>(name: N, main_currency_id: Uuid, user_id: Uuid) -> Budget {
        Budget {
            id: IdGenerator::new_id(),
            name: name.into(),
            main_currency_id,
            user_id,
        }
    }

    pub fn rename<N: Into<String>>(&mut self, new_name: N) {
        self.name = new_name.into();
    }

    pub fn change_currency(&mut self, new_currency_id: Uuid) {
        self.main_currency_id = new_currency_id;
    }

    pub fn belongs_to_user(&self, user_id: Uuid) -> bool {
        self.user_id == user_id
    }
}
