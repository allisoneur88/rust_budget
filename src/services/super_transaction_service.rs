use chrono::NaiveDate;

use crate::{Budget, IdGenerator, SuperTransaction};

#[derive(Debug)]
pub struct SuperTransactionService;

impl SuperTransactionService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_super_transaction(
        &self,
        date: NaiveDate,
        memo: &str,
        budget: &Budget,
    ) -> SuperTransaction {
        SuperTransaction {
            id: IdGenerator::new_id(),
            date,
            memo: memo.to_string(),
            budget_id: budget.id,
        }
    }

    pub fn update_date(&self, super_transaction: &mut SuperTransaction, new_date: NaiveDate) {
        super_transaction.date = new_date;
    }

    pub fn update_memo(&self, super_transaction: &mut SuperTransaction, new_memo: &str) {
        super_transaction.memo = new_memo.to_string();
    }
}
