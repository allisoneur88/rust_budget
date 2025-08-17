use uuid::Uuid;

use crate::{User, UserService, app::app_state::AppState};

pub struct UserController {
    user_service: UserService,
    app_state: AppState,
}

impl UserController {
    pub fn new() -> Self {
        Self {
            user_service: UserService::new(),
            app_state: AppState::new(),
        }
    }

    pub fn get_all(&self) -> Vec<User> {
        self.app_state.users.list()
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<User> {
        self.app_state.users.get(id)
    }

    pub fn create_with_password(&mut self, name: &str, password: &str) {
        let user = self.user_service.make_user(name, password);
        self.app_state.users.save(user);
    }

    pub fn create_without_password(&mut self, name: &str) {
        let user = self.user_service.make_user_wo_password(name);
        self.app_state.users.save(user);
    }

    pub fn update_name(&mut self, user: User, new_name: &str) {
        let mut user = self.app_state.users.get(user.id).unwrap();
        self.user_service.update_user_name(&mut user, new_name);
        self.app_state.users.save(user);
    }

    pub fn update_password(&mut self, user: User, new_password: &str) {
        let mut user = self.app_state.users.get(user.id).unwrap();
        self.user_service
            .update_user_password(&mut user, new_password);
        self.app_state.users.save(user);
    }

    pub fn delete(&mut self, user: User) {
        let user = self.app_state.users.get(user.id).unwrap();
        self.app_state.users.delete(user.id);
    }
}
