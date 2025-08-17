use crate::controllers::user_controller::UserController;

pub struct App {
    pub users: UserController,
}

impl App {
    pub fn new() -> Self {
        Self {
            users: UserController::new(),
        }
    }
}
