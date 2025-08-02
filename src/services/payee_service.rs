use crate::{IdGenerator, Payee};

#[derive(Debug)]
pub struct PayeeService;

impl PayeeService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_payee(&self, name: &str, memo: &str) -> Payee {
        Payee {
            id: IdGenerator::new_id(),
            name: name.to_string(),
            memo: memo.to_string(),
        }
    }

    pub fn update_payee_name(&self, payee: &mut Payee, new_name: &str) {
        payee.name = new_name.to_string();
    }
}
