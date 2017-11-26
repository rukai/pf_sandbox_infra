#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

             extern crate rocket_contrib;
             extern crate rocket;
#[macro_use] extern crate serde_derive;
             extern crate git2;
             extern crate chrono;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::State;

use std::path::{Path, PathBuf};
use std::env;
use std::ffi::OsStr;
use std::sync::{RwLock, Arc};

pub mod builds;
pub mod files;

use builds::{Commits};

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
fn builds(builds_rw: State<Arc<RwLock<Commits>>>) -> Template {
    let builds: &Commits = &builds_rw.read().unwrap();
    Template::render("builds", builds)
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
    let builds = builds::build_reader();
    rocket::ignite()
        .manage(builds)
        .mount("/", routes![index, builds, tutorial, manual, tas, files])
        .attach(Template::fairing())
        .launch();
}
