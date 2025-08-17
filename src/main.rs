#![allow(dead_code, unused_variables)]

use budget::app::app::App;

fn main() {
    let mut app = App::new();

    let users = app.users.get_all();
    let sasha = users
        .into_iter()
        .find(|u| u.name == String::from("Sasha"))
        .unwrap();

    app.users.update_password(sasha, "321");

    let users = app.users.get_all();

    println!("{:#?}", users);
}
