use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::IdGenerator;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Assignment {
    pub id: Uuid,
    pub amount: f64,
    pub year_month: YearMonth,

    pub category_id: Uuid,
}

impl Assignment {
    pub fn new(amount: f64, year_month: YearMonth, category_id: Uuid) -> Self {
        Self {
            id: IdGenerator::new_id(),
            amount,
            year_month,
            category_id,
        }
    }

    pub fn update_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct YearMonth(NaiveDate);

impl YearMonth {
    pub fn new(year: i32, month: u32) -> Self {
        Self(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    }
}
