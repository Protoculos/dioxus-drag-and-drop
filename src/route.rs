use crate::ui::pages::error_page::Err404;
use crate::ui::pages::home::Home;
use crate::ui::pages::simple::Simple;
use crate::ui::pages::simple_vec::SimpleVec;
use crate::ui::pages::vec_add::AddToVec;
// use crate::ui::pages::vec_del::DelFromVec;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    // The home page is at the / route
    #[route("/")]
    Home {},
    #[route("/simple")]
    Simple {},
    #[route("/simple-vec")]
    SimpleVec {},
    #[route("/add-to-vec")]
    AddToVec {},
    // #[route("/del-from-vec")]
    // DelFromVec {},
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}
