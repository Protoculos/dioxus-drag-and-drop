#![allow(non_snake_case)]

use crate::models::item::ItemCard;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_signals::Signal;

#[derive(Clone, Copy, Default)]
pub struct ApplicationData {
    pub left_vec: Signal<Vec<ItemCard>>,
    pub right_vec: Signal<Vec<ItemCard>>,
}

pub fn use_app_data(cx: Scope) -> ApplicationData {
    *use_context(cx).unwrap()
}

pub fn App(cx: Scope) -> Element {
    use_context_provider(cx, ApplicationData::default);
    // let left_vec: Vec<ItemCard> = (0..=3).map(|_| ItemCard::TextCard("Card")).collect();
    // let right_vec: Vec<ItemCard> = (0..=3).map(|_| ItemCard::TextCard("Card")).collect();

    // use_context_provider(cx, || Signal::new(LeftList::new(left_vec)));

    render! { Router::<Route> {} }
}

// js solution
// https://www.youtube.com/watch?v=4AHot187Lj0
// <script>
// let lists = document.getElementsByClassName("list");
// let fightBox = document.getElementById("right");
// let leftBox = document.getElementById("left");

// for (list of lists){
//     list.addEventListener("dragstart", function(e)){
//         let selected = e.target;

//         rightBox.addEventListener("dragover", function(e){
//             e.preventDefault();
//         });
//         rightBox.addEventListener("drop", function(e){
//             rightBox.appendChild(selected);
//             selected = nill;
//         });
//         leftBox.addEventListener("dragover", function(e){
//             e.preventDefault();
//         });
//         leftBox.addEventListener("drop", function(e){
//             rightBox.appendChild(selected);
//             selected = nill;
//         })
//     }
// }

// </script>
