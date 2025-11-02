use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::IdGenerator;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Payee {
    pub id: Uuid,
    pub name: String,
    pub memo: String,

    pub budget_id: Uuid,
}

impl Payee {
    pub fn new<N: Into<String>, M: Into<String>>(name: N, memo: M, budget_id: Uuid) -> Self {
        Self {
            id: IdGenerator::new_id(),
            name: name.into(),
            memo: memo.into(),
            budget_id,
        }
    }

    pub fn rename<N: Into<String>>(&mut self, new_name: N) {
        self.name = new_name.into();
    }

    pub fn update_memo<M: Into<String>>(&mut self, new_memo: M) {
        self.memo = new_memo.into();
    }
}
