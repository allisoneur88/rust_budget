use crate::{Account, AccountType, Currency, IdGenerator, domain::currency};

#[derive(Debug)]
pub struct AccountService;

impl AccountService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_account(
        &self,
        name: &str,
        is_off_budget: bool,
        acc_type: AccountType,
        currency: Currency,
    ) -> Account {
        Account {
            id: IdGenerator::new_id(),
            name: name.to_string(),
            is_off_budget: is_off_budget,
            acc_type,
            currency,
        }
    }

    pub fn update_name(&self, account: &mut Account, new_name: &str) {
        account.name = new_name.to_string();
    }

    pub fn toggle_is_off_budget(&self, account: &mut Account) {
        account.is_off_budget = !account.is_off_budget;
    }

    pub fn change_account_type(&self, account: &mut Account, acc_type: AccountType) {
        account.acc_type = acc_type;
    }
}
