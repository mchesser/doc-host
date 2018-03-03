#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate dotenv;
extern crate fs_extra;
extern crate git2;
extern crate mdbook;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod github;
mod git;
mod doc_builder;
mod config;

use std::error::Error;
use std::io;
use std::path::PathBuf;

use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::Json;

use github::{PushEvent, PingEvent, PushData, PingData};
use config::Config;

#[get("/")]
fn index(config: State<Config>) -> io::Result<NamedFile> {
    NamedFile::open(config.tmp_dir.join("src").join("book").join("index.html"))
}

#[get("/<file..>")]
fn files(config: State<Config>, file: PathBuf) -> Option<NamedFile> {
    let path = config.tmp_dir.join("src").join("book").join(file);

    if path.exists() {
        NamedFile::open(path).ok()
    }
    else {
        NamedFile::open(config.tmp_dir.join("src").join("book").join("index.html")).ok()
    }
}

#[post("/ambigem-docs-update", data = "<payload>")]
fn github_push(config: State<Config>, _event: PushEvent, payload: Json<PushData>)
    -> Result<(), Box<Error>>
{
    println!("{:?}", payload);
    let target_ref = format!("refs/heads/{}", config.src.branch);
    if payload.git_ref == target_ref {
        if let Err(e) = perform_update(&config) {
            println!("Error updating docs: {:?}", e);
        }
    }

    Ok(())
}

#[post("/ambigem-docs-update", data = "<payload>", rank = 1)]
fn github_ping(_event: PingEvent, payload: Json<PingData>) -> Result<(), Box<Error>> {
    println!("{:?}", payload);
    Ok(())
}

fn perform_update(config: &Config) -> Result<(), Box<Error>> {
    git::get_latest(&config)?;
    doc_builder::update_book(&config)?;
    git::commit_changes(&config)?;
    git::push_update(&config)?;
    Ok(())
}

fn main() {
    dotenv::dotenv().ok();

    let config = Config::from_env();

    perform_update(&config).unwrap();

    rocket::ignite()
        .mount("/", routes![index, files, github_push, github_ping])
        .manage(config)
        .launch();
}
