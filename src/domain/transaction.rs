use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{IdGenerator, repository::traits::CurrencyRepository};

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub flow: f64, // negative for outflow, positive for inflow
    pub memo: String,

    //Foreign keys
    pub account_id: Uuid,
    pub category_id: Uuid,
    pub super_transaction_id: Uuid,
    pub currency_id: Uuid,
    pub payee_id: Uuid,
}

impl Transaction {
    pub fn new<M: Into<String>>(
        &self,
        flow: f64,
        memo: M,
        account_id: Uuid,
        category_id: Uuid,
        super_transaction_id: Uuid,
        currency_id: Uuid,
        payee_id: Uuid,
    ) -> Self {
        Self {
            id: IdGenerator::new_id(),
            flow,
            memo: memo.into(),
            currency_id,
            account_id,
            category_id,
            super_transaction_id,
            payee_id,
        }
    }

    pub fn update_flow(&mut self, new_flow: f64) {
        self.flow = new_flow;
    }

    pub fn update_memo<M: Into<String>>(&mut self, new_memo: M) {
        self.memo = new_memo.into();
    }

    pub fn update_account(&mut self, new_account_id: Uuid, new_currency_id: Uuid) {
        self.account_id = new_account_id;
        self.currency_id = new_currency_id;
    }

    pub fn update_category(&mut self, new_category_id: Uuid) {
        self.category_id = new_category_id;
    }

    pub fn update_super_transaction(&mut self, new_super_transaction_id: Uuid) {
        self.super_transaction_id = new_super_transaction_id;
    }

    pub fn update_payee(&mut self, new_payee_id: Uuid) {
        self.payee_id = new_payee_id;
    }
}
