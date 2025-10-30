use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::IdGenerator;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct SuperTransaction {
    pub id: Uuid,
    pub date: NaiveDate,
    pub memo: String,
    pub budget_id: Uuid,
}

impl SuperTransaction {
    pub fn new<M: Into<String>>(date: NaiveDate, memo: M, budget_id: Uuid) -> Self {
        Self {
            id: IdGenerator::new_id(),
            date,
            memo: memo.into(),
            budget_id,
        }
    }

    pub fn update_date(&mut self, new_date: NaiveDate) {
        self.date = new_date;
    }

    pub fn update_memo<M: Into<String>>(&mut self, new_memo: M) {
        self.memo = new_memo.into();
    }
}
