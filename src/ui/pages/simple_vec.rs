#![allow(non_snake_case)]
use crate::models::item::ItemCard;
use crate::repo::card_repo_item::ITEM_CARD;
use crate::ui::components::item_card::ItemCardUi;
use crate::ui::components::nav_bar::NavBar;
use dioxus::prelude::*;

#[component]
pub fn SimpleVec(cx: Scope) -> Element {
    // Создаем диапазон (0..=3) и применяем функцию к каждому элементу
    let iter = (0..=3).map(|_| ITEM_CARD);
    // Собираем элементы итератора в вектор
    let v: Vec<&ItemCard> = iter.collect();

    render! {
        // Итерации вектора
        section { class: "flex flex-row",
            NavBar {}
            div { class: "container h-screen w-screen bg-[#0b0423] flex flex-row justify-center items-center",
                ul {
                    class: "w-64 h-96 m-5 border-2 border-dashed text-white",
                    // итерация одного объекта несколько раз
                    id: "left",
                    "box 1"
                    v.iter().enumerate().map(|(id, card)| {
                            render!{
                                ItemCardUi  {
                                    card: card,
                                    id: id as i32,
                                }
                            }
                        })
                }
                // Создаем в репозитории вектор и итерируем через вектор
                ul {
                    class: " w-64 h-96 m-5 border-2 border-dashed text-white",
                    ondragover: move |event| log::info!("Dragover! Event: {event:?}"),
                    ondrop: move |event| log::info!("Dragover! Event: {event:?}"),
                    id: "right",
                    "box 2"
                    v.iter().enumerate().map(|(id, card)| {
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
