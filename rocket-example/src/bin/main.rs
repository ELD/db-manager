#![feature(decl_macro, proc_macro_hygiene)]

use libexample::db::PrimaryDb;

fn main() {
    rocket::ignite()
        .attach(PrimaryDb::fairing())
        .mount("/todo", libexample::routes())
        .launch();
}
