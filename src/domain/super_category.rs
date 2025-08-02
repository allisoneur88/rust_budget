use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::category::Category;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct SuperCategory {
    pub id: Uuid,
    pub name: String,

    pub budget_id: Uuid,
}
