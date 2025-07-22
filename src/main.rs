mod domain;
use chrono::NaiveDate;
use domain::account::Account;
use domain::account_type::AccountType;
use domain::budget::Budget;
use domain::category::Category;
use domain::currency::Currency;
use domain::payee::Payee;
use domain::super_category::SuperCategory;
use domain::super_transaction::SuperTransaction;
use domain::transaction::Transaction;
use domain::user::User;

fn main() {
    test_manual();
}

fn test_manual() {
    let transaction = Transaction {
        id: 1,
        flow: Some(50.2),
        currency: Currency::Euro,
        account_id: 1,
        category_id: 1,
        super_transaction_id: 1,
        payee_id: 1,
    };
    let payee = Payee {
        id: 1,
        name: "Spar".to_string(),
    };

    let category = Category {
        id: 1,
        name: "Groceries".to_string(),
    };

    let super_transaction = SuperTransaction {
        id: 1,
        date: NaiveDate::from_ymd_opt(2025, 7, 25).unwrap(),
        memo: None,
        transactions: vec![transaction],
    };

    let account = Account {
        id: 1,
        name: "ba".to_string(),
        is_off_budget: false,
        acc_type: AccountType::Checking,
    };

    let super_category = SuperCategory {
        id: 1,
        name: "Bills".to_string(),
        categories: vec![category],
    };

    let budget = Budget {
        id: 1,
        accounts: Some(vec![account]),
        super_categories: Some(vec![super_category]),
        super_transactions: Some(vec![super_transaction]),
        main_currency: Currency::Euro,
    };

    let user = User {
        id: 1,
        name: "Sasha".to_string(),
        password: None,
        budgets: Some(vec![budget]),
    };

    println!("{:#?}", user);
}
