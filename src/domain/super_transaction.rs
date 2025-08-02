use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct SuperTransaction {
    pub id: Uuid,
    pub date: NaiveDate,
    pub memo: String,
    pub budget_id: Uuid,
}
