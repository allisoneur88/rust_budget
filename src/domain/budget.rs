use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::currency::Currency;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub main_currency: Currency,

    pub user_id: Uuid,
}
