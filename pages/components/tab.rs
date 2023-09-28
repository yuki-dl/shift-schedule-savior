#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Tab<'a>(
    cx: Scope,
    class: &'a str,
    class_blue: &'a str,
    tab_index: i32, 
    open_index: i32,
    text: String,
    onclick: EventHandler<'a, MouseEvent>
) -> Element {
    cx.render(rsx!(
        li {
            class: "w-full",
            button {
                class: if open_index == tab_index {
                    *class_blue
                } else {
                    *class
                },
                onclick: move |evt| onclick.call(evt),
                "{text}"
            }
        }
    ))
}