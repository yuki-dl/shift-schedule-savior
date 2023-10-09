#![allow(non_snake_case)]

use dioxus::prelude::*;

use super::button::NextButton;
use super::form::{
    InputEmployeeNum,
    InputMonth,
    InputEmployeeTimeframe
};
use super::tab::Tab;
use super::table::EmployeeTable;

#[inline_props]
pub fn Front<'a>(
    cx: Scope,
    employee_num: &'a String,
    year: String,
    month: &'a String,
    onchange_employee_num: EventHandler<'a, FormEvent>,
    onchange_month: EventHandler<'a, FormEvent>,
    onrotate: EventHandler<'a, MouseEvent>,
) -> Element {
    let is_validate = |s: &String, range: std::ops::Range<u8>| {
        let Ok(numbers) = s.parse::<u8>() else { return false };
        if range.contains(&numbers) {
            return true;
        }
        false
    };

    let flag = is_validate(employee_num, 2..255) && is_validate(month, 1..12);
    cx.render(rsx!(
        div {
            class: "absolute inset-0 h-full w-full rounded-xl object-cover shadow-xl shadow-black/40",
            div {
                class: "flex min-h-full flex-col items-center justify-center",
                h1 {
                    class: "text-2xl mb-10 text-center font-medium text-gray-900 dark:text-gray-300",
                    "{year}年{month}月度 人員計画表"
                }
                div {
                    class: "flex flex-col items-center justify-center",
                    InputEmployeeNum { employee_num: employee_num, onchange: move |evt| onchange_employee_num.call(evt) }
                    InputMonth { month: month, onchange: move |evt| onchange_month.call(evt) }
                    NextButton {
                        onrotate: move |evt| onrotate.call(evt),
                        flag: flag                    
                    }
                }
            }
        }
    ))
}

#[inline_props]
pub fn Back<'a>(
    cx: Scope,
    employee_num: &'a String,
    days: u32, 
    weekday_arr: Vec<String>
) -> Element {
    let is_generated = use_state(&cx, || false);
    let open_tab = use_state(&cx, || 1);

    // [morning, afternoon, evening]
    let input_vec = use_ref(&cx, || vec!["".to_string(); 3]);
    let input_vec_refcell = input_vec.read();
    
    cx.render(rsx!(
        div {
            class: "absolute inset-0 h-full w-full bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 [transform:rotateY(180deg)] backface-hidden",
            ul {
                class: "text-sm font-medium text-center text-gray-500 grid grid-cols-2 divide-x divide-gray-200 rounded-lg sm:flex dark:divide-gray-600 dark:text-gray-400",
                Tab {
                    class: "inline-block w-full p-4 rounded-tl-lg bg-gray-50 hover:bg-gray-100 focus:outline-none dark:bg-gray-700 dark:hover:bg-gray-600",
                    class_blue: "inline-block w-full p-4 rounded-tl-lg bg-gray-50 hover:bg-gray-100 focus:outline-none dark:bg-gray-700 dark:hover:bg-gray-600 text-blue-600 dark:text-blue-500",
                    tab_index: 1,
                    open_index: open_tab.get(),
                    text: "Settings",
                    onclick: move |_| open_tab.set(1)                
                }
                Tab {
                    class: "inline-block w-full p-4 rounded-tr-lg bg-gray-50 hover:bg-gray-100 focus:outline-none dark:bg-gray-700 dark:hover:bg-gray-600",
                    class_blue: "inline-block w-full p-4 rounded-tr-lg bg-gray-50 hover:bg-gray-100 focus:outline-none dark:bg-gray-700 dark:hover:bg-gray-600 text-blue-600 dark:text-blue-500",
                    tab_index: 2,
                    open_index: open_tab.get(),
                    text: "Table",
                    onclick: move |_| open_tab.set(2)
                }
            }
            div {
                class: if open_tab.get().clone() == 1 {"block"} else {"hidden"},
                div {
                    class: "flex min-h-full flex-col items-center justify-center",
                    div {
                        InputEmployeeTimeframe {
                            required_people: input_vec_refcell.to_vec(),
                            onchange_morning: move |evt: FormEvent| input_vec.with_mut(|i| i[0] = evt.value.clone()),
                            onchange_afternoon: move |evt: FormEvent| input_vec.with_mut(|i| i[1] = evt.value.clone()),
                            onchange_evening: move |evt: FormEvent| input_vec.with_mut(|i| i[2] = evt.value.clone()),
                        }
                    }
                }    
            }
            div {
                class: if open_tab.get().clone() == 2 {"block"} else {"hidden"},
                EmployeeTable {
                    employee_num: employee_num,
                    days: *days,
                    weekday_arr: weekday_arr,
                    required_people: input_vec_refcell.to_vec(),
                    is_generated: *is_generated.get(),
                    onclick: move |_| {
                        is_generated.set(true);
                    },
                    onreset: move |_| {
                        is_generated.set(false);
                        input_vec.with_mut(|i| *i = vec!["".to_string(); 3]);
                    },
                }
            }
        }
    ))
}