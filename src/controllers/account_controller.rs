use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::{
    Account, AccountType, Currency, app::app_state::AppState,
    services::account_service::AccountService,
};

pub struct AccountController {
    pub account_service: AccountService,
    pub app_state: Rc<RefCell<AppState>>,
}

impl AccountController {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            account_service: AccountService::new(),
            app_state,
        }
    }

    pub fn get_all(&self) -> Vec<Account> {
        self.app_state
            .borrow()
            .accounts
            .list(self.app_state.borrow().current_budget.as_ref().unwrap())
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<Account> {
        self.app_state
            .borrow()
            .accounts
            .get(self.app_state.borrow().current_budget.as_ref().unwrap(), id)
    }

    pub fn create(
        &self,
        name: &str,
        is_off_budget: bool,
        acc_type: AccountType,
        currency: Currency,
    ) {
        let account = self.account_service.make_account(
            name,
            is_off_budget,
            acc_type,
            currency,
            self.app_state.borrow().current_budget.as_ref().unwrap(),
        );

        self.app_state.borrow_mut().accounts.save(account);
    }

    pub fn rename(&self, id: Uuid, new_name: &str) {
        let mut account = self.get_by_id(id).unwrap();

        self.account_service.update_name(&mut account, new_name);

        self.app_state.borrow_mut().accounts.save(account);
    }

    pub fn change_type(&self, id: Uuid, acc_type: AccountType) {
        let mut account = self.get_by_id(id).unwrap();

        self.account_service
            .change_account_type(&mut account, acc_type);

        self.app_state.borrow_mut().accounts.save(account);
    }

    pub fn set_off_budget(&self, id: Uuid, value: bool) {
        let mut account = self.get_by_id(id).unwrap();

        self.account_service.set_is_off_budget(&mut account, value);

        self.app_state.borrow_mut().accounts.save(account);
    }

    pub fn delete(&self, id: Uuid) {
        self.app_state.borrow_mut().accounts.delete(id);
    }
}
