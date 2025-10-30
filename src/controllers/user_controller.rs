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

    pub fn get_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        self.repos.users.get(id)
    }

    pub fn create<N: Into<String>, P: Into<String>>(
        &self,
        name: N,
        password: Option<P>,
    ) -> AppResult<()> {
        let user = User::new(name, password);
        self.repos.users.save(&user)
    }

    pub fn update_name<N: Into<String>>(&self, user_id: Uuid, new_name: N) -> AppResult<()> {
        match self.repos.users.get(user_id)? {
            Some(mut user) => {
                user.update_name(new_name);
                self.repos.users.save(&user)?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                entity: "User",
                id: user_id,
            }),
        }
    }

    pub fn update_password<P: Into<String>>(
        &mut self,
        user_id: Uuid,
        new_password: P,
    ) -> AppResult<()> {
        match self.repos.users.get(user_id)? {
            Some(mut user) => {
                user.update_password(new_password);
                self.repos.users.save(&user)?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                entity: "User",
                id: user_id,
            }),
        }
    }

    pub fn delete(&mut self, user_id: Uuid) -> AppResult<()> {
        match self.repos.users.get(user_id)? {
            Some(user) => {
                self.repos.users.delete(user.id)?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                entity: "User",
                id: user_id,
            }),
        }
    }
}
