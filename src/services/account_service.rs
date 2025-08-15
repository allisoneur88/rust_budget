use crate::{Account, AccountType, Budget, Currency, IdGenerator, domain::currency};

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
        budget: &Budget,
    ) -> Account {
        Account {
            id: IdGenerator::new_id(),
            name: name.to_string(),
            is_off_budget: is_off_budget,
            acc_type,
            currency,
            budget_id: budget.id,
        }
    }

    pub fn update_name(&self, account: &mut Account, new_name: &str) {
        account.name = new_name.to_string();
    }

    pub fn set_is_off_budget(&self, account: &mut Account, value: bool) {
        account.is_off_budget = value;
    }

    pub fn change_account_type(&self, account: &mut Account, acc_type: AccountType) {
        account.acc_type = acc_type;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AccountType, Currency,
        services::{
            account_service::AccountService, budget_service::BudgetService,
            user_service::UserService,
        },
    };

    #[test]
    pub fn creates_account() {
        let us = UserService::new();
        let user = us.make_user_wo_password("sasha");

        let bs = BudgetService::new();
        let budget = bs.make_budget("main budget", Currency::RUB, &user);

        let accs = AccountService::new();
        let account =
            accs.make_account("sber", false, AccountType::Checking, Currency::RUB, &budget);

        assert_eq!(account.budget_id, budget.id)
    }
}
