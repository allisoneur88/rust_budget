use super::super::IdGenerator;
use super::super::User;

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        UserService
    }

    pub fn make_user_wo_password(&self, name: &str) -> User {
        User {
            id: IdGenerator::new_id(),
            name: name.to_string(),
            password: None,
            budgets: None,
        }
    }

    pub fn make_user(&self, name: &str, password: &str) -> User {
        User {
            id: IdGenerator::new_id(),
            name: name.to_string(),
            password: Some(password.to_string()),
            budgets: None,
        }
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
