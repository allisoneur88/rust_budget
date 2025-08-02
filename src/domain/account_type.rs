use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub enum AccountType {
    Checking,
    Investment,
    CreditCard,
}
