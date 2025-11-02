#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::result_large_err,
    clippy::match_wild_err_arm
)]
#![allow(dead_code, unused_variables)]

use budget::{app::application::Application, util::error::AppResult};

fn main() -> AppResult<()> {
    let app = Application::new()?;
    let users = app.users.get_all();
    println!("users: {:#?}", users);
    Ok(())
}
