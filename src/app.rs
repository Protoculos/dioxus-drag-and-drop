#![allow(non_snake_case)]

use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn App(cx: Scope) -> Element {
    // let selected = use_ref(cx, Vec::new);
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
