#![allow(dead_code, unused_variables)]

use budget::IdGenerator;
use chrono::NaiveDate;

mod domain;

use domain::{
    account::Account, account_type::AccountType, budget::Budget, category::Category,
    currency::Currency, payee::Payee, super_category::SuperCategory,
    super_transaction::SuperTransaction, transaction::Transaction, user::User,
};

mod services;

fn main() {
    test_manual();
}

fn test_manual() {}
