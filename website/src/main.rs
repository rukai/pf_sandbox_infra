#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Template;
use rocket::response::NamedFile;

use std::path::{Path, PathBuf};
use std::env;
use std::ffi::OsStr;

#[get("/")]
fn index() -> Template {
    Template::render("index", 0)
}

#[get("/tutorial")]
fn tutorial() -> Template {
    Template::render("tutorial", 0)
}

#[get("/manual")]
fn manual() -> Template {
    Template::render("manual", 0)
}

#[get("/tas")]
fn tas() -> Template {
    Template::render("tas", 0)
}

#[get("/builds")]
fn builds() -> Template {
    let netplay_build = Commit {
        hash:      format!("1234"),
        message:   format!("Hello world"),
        os:        vec!(format!("Windows"), format!("Linux")),
        date_unix: 0,
    };

    let builds = vec!(Commit {
        hash:      format!("1234"),
        message:   format!("Hello world"),
        os:        vec!(format!("Windows"), format!("Linux")),
        date_unix: 0,
    });

    let page = BuildsPage {
        netplay_build,
        builds
    };

    Template::render("builds", &page)
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    if env::current_dir().unwrap().file_name().unwrap() != OsStr::new("website") {
        // templates are correctly located by finding the Rocket.toml
        // However static files are not handled, so if we aren't in the root directory, close immediately to avoid headaches.
        println!("Wrong directory, dummy!");
        return;
    }
    rocket::ignite().mount("/", routes![index, builds, tutorial, manual, tas, files]).attach(Template::fairing()).launch();
}

#[derive(Serialize)]
struct BuildsPage {
    netplay_build: Commit,
    builds:    Vec<Commit>
}

#[derive(Serialize)]
struct Commit {
    hash:      String,
    message:   String,
    os:        Vec<String>,
    date_unix: u64,
}
