#![feature(proc_macro_hygiene, decl_macro)]

// std

// external
use rocket::*;
use rocket_contrib::{serve::StaticFiles, templates::Template};

#[get("/")]
fn root() -> Result<Template, String> {
    Ok(Template::render("index", {}))
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/", routes![root])
        .attach(Template::fairing())
        .launch();
}
