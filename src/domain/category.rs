use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub super_category_id: Uuid,
}
