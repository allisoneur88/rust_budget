use crate::Budget;
use crate::Currency;
use crate::IdGenerator;
use crate::User;

#[derive(Debug)]
pub struct BudgetService;

impl BudgetService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_budget(&self, name: &str, main_currency: Currency, user: &User) -> Budget {
        Budget {
            id: IdGenerator::new_id(),
            name: name.to_string(),
            main_currency,
            user_id: user.id,
        }
    }

    pub fn update_budget_name(&self, budget: &mut Budget, new_name: &str) {
        budget.name = new_name.to_string();
    }

    pub fn change_main_currency(&self, budget: &mut Budget, new_currency: Currency) {
        budget.main_currency = new_currency;
    }
}

#[cfg(test)]
mod tests {
    use crate::services::{
        budget_service::BudgetService, currency_service::CurrencyService, user_service::UserService,
    };

    #[test]
    pub fn creates_budget() {
        let us = UserService::new();
        let user = us.make_user_wo_password("Sasha");

        let bs = BudgetService::new();
        let cs = CurrencyService::new();
        let budget = bs.make_budget("main budet", cs.make_currency("RUB", "R", "Roubles"), &user);

        assert_eq!(budget.user_id, user.id);
    }
}
