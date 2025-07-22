use super::transaction::Transaction;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct SuperTransaction {
    pub id: u64,
    pub date: NaiveDate,
    pub memo: Option<String>,
    pub transactions: Vec<Transaction>,
}
