use std::cell::Ref;

use crate::{
    Account, Budget, User,
    app::app_state::AppState,
    util::error::{AppError, AppResult},
};

pub fn require_user<'a>(state: &'a Ref<'a, AppState>) -> AppResult<&'a User> {
    state
        .current_user
        .as_ref()
        .ok_or_else(|| AppError::Validation("No current user selected".into()))
}

pub fn require_budget<'a>(state: &'a Ref<'a, AppState>) -> AppResult<&'a Budget> {
    state
        .current_budget
        .as_ref()
        .ok_or_else(|| AppError::Validation("No current budget selected".into()))
}

pub fn require_account<'a>(state: &'a Ref<'a, AppState>) -> AppResult<&'a Account> {
    state
        .current_account
        .as_ref()
        .ok_or_else(|| AppError::Validation("No current account selected".into()))
}

pub fn username_is_unique<'a>(state: &'a Ref<'a, AppState>, name: &str) -> AppResult<()> {
    match state.users.list().iter().find(|u| u.name == name) {
        None => Ok(()),
        Some(..) => Err(AppError::UserExists(
            "User with this name already exists".into(),
        )),
    }
}
