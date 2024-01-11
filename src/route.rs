use crate::ui::pages::del_sort::DelSort;
use crate::ui::pages::error_page::Err404;
use crate::ui::pages::home::Home;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    // The home page is at the / route
    #[route("/")]
    Home {},
    #[route("/del-sort")]
    DelSort {},
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}
