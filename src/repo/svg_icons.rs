// use crate::models::svg::{Svg, SvgPath};

// pub static BULLER: Component<()> = |cx| {
//     cx.render(rsx!(
//         svg {
//             class: "w-6 h-6",
//             stroke_linecap: "round",
//             stroke_linejoin: "round",
//             fill: "none",
//             stroke_width: "2",
//             view_box: "0 0 24 24",
//             stroke: "currentColor",
//             path { d: "M22 12h-4l-3 9L9 3l-3 9H2" }
//         }
//     ))
// };

// pub static FORMAT_LIST_BULLER_SQUARE: &'static Svg = &Svg{
//         class: "",
//         xmlns: "http://www.w3.org/2000/svg",
//         view_box: "0 0 512 512",
//         path: &SvgPath{
//             d: "M40 48C26.7 48 16 58.7 16 72v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V72c0-13.3-10.7-24-24-24H40zM192 64c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zm0 160c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zm0 160c-17.7 0-32 14.3-32 32s14.3 32 32 32H480c17.7 0 32-14.3 32-32s-14.3-32-32-32H192zM16 232v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V232c0-13.3-10.7-24-24-24H40c-13.3 0-24 10.7-24 24zM40 368c-13.3 0-24 10.7-24 24v48c0 13.3 10.7 24 24 24H88c13.3 0 24-10.7 24-24V392c0-13.3-10.7-24-24-24H40z",
//         },
//     };

// pub static TRASH_OUTLINE: Svg = Svg{
//         class: "",
//         xmlns: "http://www.w3.org/2000/svg",
//         view_box: "0 0 24 24",
//         path: &SvgPath{
//             d: "M9,3V4H4V6H5V19A2,2 0 0,0 7,21H17A2,2 0 0,0 19,19V6H20V4H15V3H9M7,6H17V19H7V6M9,8V17H11V8H9M13,8V17H15V8H13Z",
//         },
//     };
