#![allow(non_snake_case)]
use crate::models::item::ItemCard;
use dioxus::prelude::*;

#[component]
pub fn ItemCardUi<'a>(cx: Scope, card: &'a ItemCard, id: i32) -> Element {
    render! {
        li {
            class: "list bg-[#e91e63] h-14 m-7 text-rose-100 cursor-grab flex flex-row items-center gap-3",
            draggable: "{card.draggable}",
            onclick: move |event| log::info!("Clicked! Event: {event:?}"),
            ondragstart: move |event| log::info!("DragStart! Event: {event:?}"),
            // ondragstart: move |event| {
            //     selected.with_mut(|selected| selected.push(event));
            // },
            svg { class: "fill-current text-white w-7 ml-2", xmlns: card.svgLeft.xmlns, view_box: card.svgLeft.view_box, path { d: card.svgLeft.path.d } }
            "{card.title} {id}"
        }
    }
}
