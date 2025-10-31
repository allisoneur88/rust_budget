use serde::{self, Deserialize, Serialize};
use uuid::Uuid;

use crate::IdGenerator;

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: Option<String>,
}

impl User {
    pub fn new<N: Into<String>, P: Into<String>>(name: N, password: Option<P>) -> Self {
        Self {
            id: IdGenerator::new_id(),
            name: name.into(),
            password: password.map(Into::into),
        }
    }

    pub fn new_with_password<N: Into<String>, P: Into<String>>(name: N, password: P) -> Self {
        Self::new(name, Some(password))
    }

    pub fn new_wo_password<N: Into<String>>(name: N) -> Self {
        Self::new::<N, String>(name, None)
    }

    pub fn rename<N: Into<String>>(&mut self, new_name: N) {
        self.name = new_name.into();
    }

    pub fn update_password<P: Into<String>>(&mut self, new_password: P) {
        self.password = Some(new_password.into());
    }
}
