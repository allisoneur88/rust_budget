use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Currency {
    EUR,
    USD,
    RUB,
    GBP,
}
