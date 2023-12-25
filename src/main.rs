#![allow(non_snake_case)]
mod app;
mod models;
mod repo;
pub mod route;
mod ui;

use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(app::App);
}
