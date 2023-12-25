// #![allow(non_snake_case)]
// use crate::repo::card_repo_item::{get_vec, ITEM_CARD};
// use crate::ui::components::item_card::ItemCardUi;
// use crate::ui::components::nav_bar::NavBar;
// use dioxus::prelude::*;

// #[component]
// pub fn DelFromVec(cx: Scope) -> Element {
//     let mut left_vec = get_vec();
//     let mut right_vec = get_vec();
//     let left_list = use_ref(cx, || &mut left_vec);
//     let right_list = use_ref(cx, || &mut right_vec);

//     render! {
//         // Итерации вектора
//         section { class: "flex flex-row",
//             NavBar {}
//             div { class: "flex flex-col items-center w-full",
//                 div { class: "text-white", "Hello form lists" }
//                 div { class: "bg-[#0b0423] flex flex-row gap-2",
//                     div { class: "flex flex-row items-start gap-2",
//                         button {
//                             class: "p-2 text-white bg-red-600 hover:bg-red-500 rounded-md",
//                             onclick: move |_| {
//                                 left_list.with_mut(|left_list| left_list.insert(0, ITEM_CARD.clone()));
//                             },
//                             "Add above"
//                         }
//                         ul {
//                             class: "w-64 border-2 border-dashed text-white",
//                             // итерация одного объекта несколько раз
//                             id: "left",
//                             "box 1"
//                             left_list.read().iter().enumerate().map(|(id, card)| {
//                                 render!{
//                                     ItemCardUi  {
//                                         card: card,
//                                         id: id as i32,
//                                     }
//                                 }
//                             })
//                         }
//                     }
//                     // Создаем в репозитории вектор и итерируем через вектор
//                     div {
//                         div { class: "flex flex-row items-end gap-2",
//                             ul {
//                                 class: "w-64 border-2 border-dashed text-white",
//                                 ondragover: move |event| log::info!("Dragover! Event: {event:?}"),
//                                 ondrop: move |event| log::info!("Dragover! Event: {event:?}"),
//                                 id: "right",
//                                 "box 2"
//                                 right_list.read().iter().enumerate().map(|(id, card)| {
//                             render!{
//                                 ItemCardUi  {
//                                     card: card,
//                                     id: id as i32,
//                                 }
//                             }
//                         })
//                             }
//                             button {
//                                 class: "flex-0 p-2 text-white bg-red-600 hover:bg-red-500 rounded-md",
//                                 onclick: move |_| {
//                                     right_list.with_mut(|right_list| right_list.push(ITEM_CARD.clone()));
//                                 },
//                                 "Add bottom"
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
