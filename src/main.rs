#![allow(non_snake_case)]
use dioxus::prelude::*;
use log::{info, LevelFilter};

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    // let selected = use_ref(cx, Vec::new);
    render! {
        div { class: "container w-full h-screen bg-[#0b0423] flex flex-row justify-center items-center",
            div {
                class: " w-64 h-96 m-5 border-2 border-dashed text-white",

                id: "left",
                "box 1"
                div {
                    class: "list bg-[#e91e63] h-14 m-7 text-rose-100 cursor-grab flex flex-row items-center gap-3",
                    draggable: "true",
                    onclick: move |event| log::info!("Clicked! Event: {event:?}"),
                    ondragstart: move |event| log::info!("DragStart! Event: {event:?}"),
                    // ondragstart: move |event| {
                    //     selected.with_mut(|selected| selected.push(event));
                    // },
                    svg {
                        class: " fill-current text-white w-7 ml-2",
                        xmlns: "http://www.w3.org/2000/svg",
                        // height: "16",
                        // width: "16",
                        view_box: "0 0 512 512",
                        path { d: "M40 48C26.7 48 16 58.7 16 72v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V72c0-13.3-10.7-24-24-24H40zM192 64c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zm0 160c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zm0 160c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zM16 232v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V232c0-13.3-10.7-24-24-24H40c-13.3 0-24 10.7-24 24zM40 368c-13.3 0-24 10.7-24 24v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V392c0-13.3-10.7-24-24-24H40z" }
                    }
                    "List item 1"
                }
                div {
                    class: "list bg-[#e91e63] h-14 m-7 text-rose-100 cursor-grab flex flex-row items-center gap-3",
                    draggable: "true",
                    svg {
                        class: " fill-current text-white w-7 ml-2",
                        xmlns: "http://www.w3.org/2000/svg",
                        // height: "16",
                        // width: "16",
                        view_box: "0 0 512 512",
                        path { d: "M40 48C26.7 48 16 58.7 16 72v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V72c0-13.3-10.7-24-24-24H40zM192 64c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zm0 160c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zm0 160c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zM16 232v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V232c0-13.3-10.7-24-24-24H40c-13.3 0-24 10.7-24 24zM40 368c-13.3 0-24 10.7-24 24v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V392c0-13.3-10.7-24-24-24H40z" }
                    }
                    "List item 2"
                }
                div {
                    class: "list bg-[#e91e63] h-14 m-7 text-rose-100 cursor-grab flex flex-row items-center gap-3",
                    draggable: "true",
                    svg {
                        class: " fill-current text-white w-7 ml-2",
                        xmlns: "http://www.w3.org/2000/svg",
                        // height: "16",
                        // width: "16",
                        view_box: "0 0 512 512",
                        path { d: "M40 48C26.7 48 16 58.7 16 72v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V72c0-13.3-10.7-24-24-24H40zM192 64c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zm0 160c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zm0 160c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zM16 232v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V232c0-13.3-10.7-24-24-24H40c-13.3 0-24 10.7-24 24zM40 368c-13.3 0-24 10.7-24 24v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V392c0-13.3-10.7-24-24-24H40z" }
                    }
                    "List item 3"
                }
                div {
                    class: "list bg-[#e91e63] h-14 m-7 text-rose-100 cursor-grab flex flex-row items-center gap-3",
                    draggable: "true",
                    svg {
                        class: " fill-current text-white w-7 ml-2",
                        xmlns: "http://www.w3.org/2000/svg",
                        // height: "16",
                        // width: "16",
                        view_box: "0 0 512 512",
                        path { d: "M40 48C26.7 48 16 58.7 16 72v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V72c0-13.3-10.7-24-24-24H40zM192 64c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zm0 160c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zm0 160c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zM16 232v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V232c0-13.3-10.7-24-24-24H40c-13.3 0-24 10.7-24 24zM40 368c-13.3 0-24 10.7-24 24v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V392c0-13.3-10.7-24-24-24H40z" }
                    }
                    "List item 4"
                }
            }
            div {
                class: " w-64 h-96 m-5 border-2 border-dashed text-white",
                ondragover: move |event| log::info!("Dragover! Event: {event:?}"),
                ondrop: move |event| log::info!("Dragover! Event: {event:?}"),
                id: "right",
                "box 2"
            }
        }
    }
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
