use std::sync::Arc;

use uuid::Uuid;

use crate::{
    User,
    app::repositories::Repositories,
    util::error::{AppError, AppResult},
};

pub struct UserController {
    repos: Arc<Repositories>,
}

impl UserController {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub fn get_all(&self) -> AppResult<Vec<User>> {
        self.repos.users.list()
    }

    pub fn get_by_id(&self, id: Uuid) -> AppResult<User> {
        self.repos
            .users
            .get(id)?
            .ok_or_else(|| AppError::NotFound { entity: "User", id })
    }

    pub fn create<N: Into<String>, P: Into<String>>(
        &self,
        name: N,
        password: Option<P>,
    ) -> AppResult<User> {
        let user = User::new(name, password);
        self.repos.users.save(&user)?;
        Ok(user)
    }

    pub fn rename<N: Into<String>>(&self, id: Uuid, new_name: N) -> AppResult<()> {
        let mut user = self.get_by_id(id)?;
        user.rename(new_name);
        self.repos.users.save(&user)
    }

    pub fn update_password<P: Into<String>>(&mut self, id: Uuid, new_password: P) -> AppResult<()> {
        let mut user = self.get_by_id(id)?;
        user.update_password(new_password);
        self.repos.users.save(&user)
    }

    pub fn delete(&mut self, id: Uuid) -> AppResult<()> {
        self.repos.users.delete(id)
    }
}
