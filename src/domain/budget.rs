use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub main_currency_id: Uuid,
    pub user_id: Uuid,
}
