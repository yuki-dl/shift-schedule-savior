#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn NextButton<'a>(
    cx: Scope,
    onrotate: EventHandler<'a, MouseEvent>,
    flag: bool
) -> Element {
    if *flag {
        cx.render(rsx!(
            button {
                class: "text-lg font-semibold text-white py-2 px-4 mt-5 rounded bg-indigo-700 btn hover:bg-indigo-400",
                onclick: move |evt| onrotate.call(evt),
                "Next"
            }
        ))
    } else {
        cx.render(rsx!(
            button {
                class: "text-lg font-semibold text-white py-2 px-4 mt-5 rounded bg-indigo-700 opacity-30",
                disabled: true,
                "Next"
            }
        ))
    }
}

#[inline_props]
pub fn GenerateButton<'a>(
    cx: Scope,
    onclick: EventHandler<'a, MouseEvent>,
    is_empty: bool
) -> Element {
    if *is_empty {
        cx.render(rsx!(
            button {
                class: "text-base font-semibold text-white py-2 px-4 ml-4 rounded bg-indigo-700 opacity-30",
                disabled: true,
                "Generate"
            }
        ))
    } else {
        cx.render(rsx!(
            button {
                class: "text-base font-semibold text-white py-2 px-4 ml-4 rounded bg-indigo-700 btn hover:bg-indigo-400 active:bg-rose-500",
                onclick: move |evt| onclick.call(evt),
                "Generate"
            }
        ))
    }

}

#[inline_props]
pub fn RegenerateButton<'a>(
    cx: Scope,
    onclick: EventHandler<'a, MouseEvent>
) -> Element {
    cx.render(rsx!(
        button {
            class: "text-base font-semibold text-white py-2 px-4 ml-4 rounded bg-indigo-700 btn hover:bg-indigo-400 active:bg-rose-500",
            onclick: move |evt| onclick.call(evt),
            "Regenerate"
        }
    ))
}

#[inline_props]
pub fn ResetButton<'a>(
    cx: Scope,
    onclick: EventHandler<'a, MouseEvent>
) -> Element {
    cx.render(rsx!(
        button {
            class: "text-base font-semibold text-white py-2 px-4 ml-4 rounded bg-indigo-700 btn hover:bg-indigo-400",
            onclick: move |evt| onclick.call(evt),
            "Reset"
        }
    ))
}