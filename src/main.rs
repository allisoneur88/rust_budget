#![warn(clippy::all, clippy::pedantic)]
#![allow(dead_code, unused_variables)]

use budget::app::app::App;

fn main() {
    let mut app = App::new();

    let users = app.users.get_all();

    println!("users: {:#?}", users);
}
