#![allow(non_snake_case)]
use crate::repo::card_repo_item::{ITEM_CARD, ITEM_CARDS};
use crate::ui::components::item_card::ItemCardUi;
use crate::ui::components::nav_bar::NavBar;
use dioxus::prelude::*;

#[component]
pub fn Simple(cx: Scope) -> Element {
    render! {
        // Итерации без вектора
        section { class: "flex flex-row",
            NavBar {}
            div { class: "container h-screen w-screen bg-[#0b0423] flex flex-row justify-center items-center",
                ul {
                    class: "w-64 h-96 m-5 border-2 border-dashed text-white",
                    // итерация одного объекта несколько раз
                    id: "left",
                    "box 1"
                    (0..=3).into_iter().map(|x| {
                        render!{
                            ItemCardUi  {
                                card: ITEM_CARD,
                                id: x,
                            }
                        }
                    })
                }
                // итерация слайса объектов
                ul {
                    class: " w-64 h-96 m-5 border-2 border-dashed text-white",
                    ondragover: move |event| log::info!("Dragover! Event: {event:?}"),
                    ondrop: move |event| log::info!("Dragover! Event: {event:?}"),
                    id: "right",
                    "box 2"
                    ITEM_CARDS.iter().enumerate().map(|(id, card)| {
                    render!{
                        ItemCardUi  {
                            card: card,
                            id: id as i32,
                        }
                    }
                })
                }
            }
        }
    }
}
