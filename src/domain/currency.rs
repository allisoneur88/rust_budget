use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::IdGenerator;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Currency {
    pub id: Uuid,
    pub code: String,
    pub symbol: String,
    pub name: String,

    pub user_id: Uuid,
}

impl Currency {
    pub fn new<C: Into<String>, S: Into<String>, N: Into<String>>(
        user_id: Uuid,
        code: C,
        symbol: S,
        name: N,
    ) -> Self {
        Self {
            id: IdGenerator::new_id(),
            user_id,
            code: code.into(),
            symbol: symbol.into(),
            name: name.into(),
        }
    }

    pub fn update_currency<C: Into<String>, S: Into<String>, N: Into<String>>(
        &mut self,
        new_code: C,
        new_symbol: S,
        new_name: N,
    ) {
        self.code = new_code.into();
        self.symbol = new_symbol.into();
        self.name = new_name.into();
    }
}
