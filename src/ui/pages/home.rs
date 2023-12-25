#![allow(non_snake_case)]
use crate::ui::components::nav_bar::NavBar;
use dioxus::prelude::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    render! {
        // Итерации без вектора
        section { class: "flex flex-row",
            NavBar {}
            div { class: "container h-screen w-screen bg-[#0b0423] flex flex-row justify-center items-center text-white",
                "Hello from Home"
            }
        }
    }
}
