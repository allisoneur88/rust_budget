use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Payee {
    pub id: Uuid,
    pub name: String,
    pub memo: String,
}
