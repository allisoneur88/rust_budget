use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::IdGenerator;

use super::account_type::AccountType;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub is_off_budget: bool,
    pub acc_type: AccountType,

    pub currency_id: Uuid,

    pub budget_id: Uuid,
}

impl Account {
    pub fn new<N: Into<String>>(
        name: N,
        is_off_budget: bool,
        acc_type: AccountType,
        currency_id: Uuid,
        budget_id: Uuid,
    ) -> Self {
        Self {
            id: IdGenerator::new_id(),
            name: name.into(),
            is_off_budget,
            acc_type,
            currency_id,
            budget_id,
        }
    }

    pub fn rename<N: Into<String>>(&mut self, new_name: N) {
        self.name = new_name.into();
    }

    pub fn set_is_off_budget(&mut self, value: bool) {
        self.is_off_budget = value;
    }

    pub fn change_account_type(&mut self, acc_type: AccountType) {
        self.acc_type = acc_type;
    }
}
