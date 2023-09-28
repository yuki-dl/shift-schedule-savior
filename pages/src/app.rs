use dioxus::prelude::*;

// use super::components::header::Header;
// use super::components::footer::Footer;
use super::components::card::{
    Front,
    Back
};
use crate::crates::weekday::get_year_month;

pub fn app(cx: Scope) -> Element {
    // state
    let is_rotate = use_state(&cx, || false);
    let employee_num = use_state(&cx, || "".to_string());

    let (year, _month) = get_year_month();
    let month = use_state(&cx, || _month);
    
    cx.render(rsx!(
        // Header {}
        main {
            class: "flex min-h-screen items-center justify-center dark:bg-gray-800",
            div {
                class: "group h-96 w-96 [perspective:1000px]",
                div {
                    class: if *is_rotate.get() {
                        "relative h-full w-full rounded-xl shadow-xl transition-all duration-500 [transform-style:preserve-3d] [transform:rotateY(180deg)]"
                    } else {
                        "relative h-full w-full rounded-xl shadow-xl transition-all duration-500 [transform-style:preserve-3d]"
                    },
                    Front {
                        employee_num: employee_num.get().clone(),
                        year: year,
                        month: month.get().clone(),
                        onchange_employee_num: move |evt: FormEvent| employee_num.set(evt.value.clone()),
                        onchange_month: move |evt: FormEvent| month.set(evt.value.clone()),
                        onrotate: move |_| is_rotate.set(true),
                    }
                    Back {
                        employee_num: employee_num.get().clone(),
                        month: month.get().clone(),
                    }
                }
            }
        }
    ))
}

