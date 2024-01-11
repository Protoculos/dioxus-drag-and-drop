#![allow(non_snake_case)]
use crate::app::use_app_data;
use crate::models::item::ItemCard;
use crate::ui::components::item_card::ItemCardUi;
use crate::ui::components::nav_bar::NavBar;
use dioxus::prelude::*;
use dioxus_signals::Signal;
// use dioxus_signals::*;

#[component]
pub fn DelSort(cx: Scope) -> Element {
    let left_vec: Vec<ItemCard> = (0..=3).map(|_| ItemCard::TextCard("Card")).collect();
    let right_vec: Vec<ItemCard> = (0..=3).map(|_| ItemCard::TextCard("Card")).collect();

    let data = use_app_data(cx);

    *data.left_vec.write() = left_vec;
    *data.right_vec.write() = right_vec;

    render! {
        section { class: "flex flex-row",
            NavBar {}
            div { class: "flex flex-col items-center w-full",
                div { class: "text-white", "Hello form lists" }
                div { class: "bg-[#0b0423] flex flex-row gap-2",
                    div { class: "flex flex-row items-start gap-2",
                        button {
                            class: "p-2 text-white bg-red-600 hover:bg-red-500 rounded-md",
                            onclick: move |_| { data.left_vec.write().insert(0, ItemCard::TextCard("Above card")) },
                            "Add above"
                        }
                        ul {
                            class: "w-64 border-2 border-dashed text-white",
                            id: "left",
                            "box 1"
                            data.left_vec.read().iter().enumerate().map(|(id, card)| {
                                render!{
                                    ItemCardUi  {
                                        card: *card,
                                        id: id,
                                        signal: data.left_vec,
                                    }
                                }
                            })
                        }
                    }
                    div {
                        div { class: "flex flex-row items-end gap-2",
                            ul {
                                class: "w-64 border-2 border-dashed text-white",
                                ondragover: move |event| log::info!("Dragover! Event: {event:?}"),
                                ondrop: move |event| log::info!("Dragover! Event: {event:?}"),
                                id: "right",
                                "box 2"
                                data.right_vec.read().iter().enumerate().map(|(id, card)| {
                            render!{
                                ItemCardUi  {
                                    card: *card,
                                    id: id,
                                    signal: data.right_vec,
                                }
                            }
                        })
                            }
                            button {
                                class: "flex-0 p-2 text-white bg-red-600 hover:bg-red-500 rounded-md",
                                onclick: move |_| {
                                    data.right_vec.write().push(ItemCard::TextCard("Bottom Card"));
                                },
                                "Add bottom"
                            }
                        }
                    }
                }
            }
        }
    }
}
