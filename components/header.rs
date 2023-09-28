#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx!(
        header {
            nav {
                class: "w-full",
                div {
                    class: "flex flex-wrap justify-between items-center mx-auto max-w-screen-xl",
                    span {
                        class: "text-3xl font-black text-gray-900 self-center whitespace-nowrap dark:text-white",
                        "Shift Schedule Savior"
                    },
                }
            }
        }
    ))
}