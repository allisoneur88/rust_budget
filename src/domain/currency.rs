use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Currency {
    pub id: Uuid,
    pub code: String,
    pub symbol: String,
    pub name: String,
}
