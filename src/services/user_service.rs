use crate::Budget;

use super::super::IdGenerator;
use super::super::User;

#[derive(Debug)]
pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self
    }

    pub fn make_user_wo_password(&self, name: &str) -> User {
        User {
            id: IdGenerator::new_id(),
            name: name.to_string(),
            password: None,
        }
    }

    pub fn make_user(&self, name: &str, password: &str) -> User {
        User {
            id: IdGenerator::new_id(),
            name: name.to_string(),
            password: Some(password.to_string()),
        }
    }

    pub fn update_user_name(&self, user: &mut User, new_name: &str) {
        user.name = new_name.to_string();
    }

    pub fn update_user_password(&self, user: &mut User, new_password: &str) {
        user.password = Some(new_password.to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::services::user_service;

    use super::*;

    #[test]
    fn creates_user_wo_pw() {
        let user_sercice = UserService::new();

        let user = user_sercice.make_user_wo_password("Sasha");

        assert_eq!(user.name, "Sasha".to_string());
        assert_eq!(user.password, None);
    }

    #[test]
    fn creates_user_with_pw() {
        let user_service = UserService::new();

        let user = user_service.make_user("Alyonka", "password");

        assert_ne!(user.password, None);
        assert_eq!(user.password, Some("password".to_string()));
        assert_eq!(user.name, "Alyonka".to_string());
    }
}
