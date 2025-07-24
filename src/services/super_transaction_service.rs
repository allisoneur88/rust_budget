use chrono::NaiveDate;

use crate::{IdGenerator, SuperTransaction, Transaction, domain::super_transaction};

#[derive(Debug)]
pub struct SuperTransactionService;

impl SuperTransactionService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_super_transaction_wo_memo(&self, date: NaiveDate) -> SuperTransaction {
        SuperTransaction {
            id: IdGenerator::new_id(),
            date: date,
            memo: None,
            transactions: None,
        }
    }

    pub fn make_super_transaction(&self, date: NaiveDate, memo: &str) -> SuperTransaction {
        SuperTransaction {
            id: IdGenerator::new_id(),
            date: date,
            memo: Some(memo.to_string()),
            transactions: None,
        }
    }

    pub fn update_date(&self, super_transaction: &mut SuperTransaction, new_date: NaiveDate) {
        super_transaction.date = new_date;
    }

    pub fn update_memo(&self, super_transaction: &mut SuperTransaction, new_memo: &str) {
        super_transaction.memo = Some(new_memo.to_string());
    }

    pub fn add_transaction(
        &self,
        super_transaction: &mut SuperTransaction,
        transacton: Transaction,
    ) {
        match &mut super_transaction.transactions {
            None => super_transaction.transactions = Some(vec![transacton]),
            Some(transactions) => transactions.push(transacton),
        }
    }
}
