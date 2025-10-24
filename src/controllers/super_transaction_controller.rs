use std::{cell::RefCell, rc::Rc};

use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    SuperTransaction, app::app_state::AppState,
    services::super_transaction_service::SuperTransactionService,
};

pub struct SuperTransactionController {
    pub super_transaction_service: SuperTransactionService,
    pub app_state: Rc<RefCell<AppState>>,
}

impl SuperTransactionController {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            super_transaction_service: SuperTransactionService::new(),
            app_state,
        }
    }

    pub fn get_all(&self) -> Vec<SuperTransaction> {
        self.app_state
            .borrow()
            .super_transactions
            .list(self.app_state.borrow().current_budget.as_ref().unwrap())
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<SuperTransaction> {
        self.app_state
            .borrow()
            .super_transactions
            .get(self.app_state.borrow().current_budget.as_ref().unwrap(), id)
    }

    pub fn create(&self, date: NaiveDate, memo: &str) {
        let super_transaction = self.super_transaction_service.make_super_transaction(
            date,
            memo,
            self.app_state.borrow().current_budget.as_ref().unwrap(),
        );
        self.app_state
            .borrow_mut()
            .super_transactions
            .save(super_transaction);
    }

    pub fn update_memo(&self, id: Uuid, new_memo: &str) {
        let mut super_transaction = self.get_by_id(id).unwrap();
        self.super_transaction_service
            .update_memo(&mut super_transaction, new_memo);
        self.app_state
            .borrow_mut()
            .super_transactions
            .save(super_transaction);
    }

    pub fn update_date(&self, id: Uuid, new_date: NaiveDate) {
        let mut super_transaction = self.get_by_id(id).unwrap();
        self.super_transaction_service
            .update_date(&mut super_transaction, new_date);
        self.app_state
            .borrow_mut()
            .super_transactions
            .save(super_transaction);
    }

    pub fn delete(&self, id: Uuid) {
        self.app_state.borrow_mut().super_transactions.delete(id);
    }
}
