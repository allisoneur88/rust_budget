use super::transaction::Transaction;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug)]
pub struct SuperTransaction {
    pub id: Uuid,
    pub date: NaiveDate,
    pub memo: Option<String>,
    pub transactions: Vec<Transaction>,
}
