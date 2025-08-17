use serde::{Deserialize, Serialize};

use crate::{Currency, IdGenerator};

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CurrencyService;

impl CurrencyService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_currency(&self, code: &str, symbol: &str, name: &str) -> Currency {
        Currency {
            id: IdGenerator::new_id(),
            code: code.to_string(),
            symbol: symbol.to_string(),
            name: name.to_string(),
        }
    }

    pub fn update_currency(&self, currency: &mut Currency, code: &str, symbol: &str, name: &str) {
        currency.code = code.to_string();
        currency.symbol = symbol.to_string();
        currency.name = name.to_string();
    }
}
