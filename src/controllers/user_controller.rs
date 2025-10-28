use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::{
    User, UserService,
    app::app_state::AppState,
    util::{
        error::{AppError, AppResult},
        validators::username_is_unique,
    },
};

pub struct UserController {
    user_service: UserService,
    app_state: Rc<RefCell<AppState>>,
}

impl UserController {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            user_service: UserService::new(),
            app_state,
        }
    }

    pub fn get_all(&self) -> AppResult<Vec<User>> {
        let state = self.app_state.borrow();
        Ok(state.users.list())
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<User> {
        let state = self.app_state.borrow();
        let user = state
            .users
            .get(id)
            .ok_or(AppError::NotFound { entity: "User", id })?;
        drop(state);
        Ok(user)
    }

    pub fn create(&mut self, name: &str, password: Option<&str>) -> AppResult<()> {
        let state = self.app_state.borrow();
        username_is_unique(&state, name)?;
        drop(state);
        let user = match password {
            Some(p) => self.user_service.make_user(name, p),
            None => self.user_service.make_user_wo_password(name),
        };
        self.app_state.borrow_mut().users.save(user)
    }
    pub fn create_with_password(&mut self, name: &str, password: &str) -> AppResult<()> {
        self.create(name, Some(password))
    }

    pub fn create_without_password(&mut self, name: &str) -> AppResult<()> {
        self.create(name, None)
    }

    pub fn update_name(&mut self, user: User, new_name: &str) -> AppResult<()> {
        let mut user = self.app_state.borrow().users.get(user.id).unwrap();
        self.user_service.update_user_name(&mut user, new_name);
        self.app_state.borrow_mut().users.save(user)
    }

    pub fn update_password(&mut self, user: User, new_password: &str) -> AppResult<()> {
        let mut user = self.app_state.borrow().users.get(user.id).unwrap();
        self.user_service
            .update_user_password(&mut user, new_password);
        self.app_state.borrow_mut().users.save(user)
    }

    pub fn delete(&mut self, user: User) -> AppResult<()> {
        let user = self.app_state.borrow().users.get(user.id).unwrap();
        self.app_state.borrow_mut().users.delete(user.id)
    }
}
