use crate::{UserService, app::app_state::AppState};

pub struct UserController {
    user_service: UserService,
    app_state: AppState,
}
