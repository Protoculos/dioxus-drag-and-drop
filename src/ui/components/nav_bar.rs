#![allow(non_snake_case)]
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn NavBar(cx: Scope) -> Element {
    render! {
        div { class: "w-64 h-screen bg-slate-400",
            Link { class: "w-64 h-screen bg-slate-400", to: Route::Home {}, "Home" }
            ul {
                "Simple lists"
                li { class: "flex flex-col",
                    Link { class: "", to: Route::Simple {}, "Simple List" }
                    Link { class: "", to: Route::SimpleVec {}, "Simple Vector" }
                    Link { class: "", to: Route::AddToVec {}, "Add to Vector" }
                }
            }
        }
    }
}
