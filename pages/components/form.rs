#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn InputMonth<'a>(cx: Scope, month: String, onchange: EventHandler<'a, FormEvent>) -> Element<'a> {
    cx.render(rsx!(
        div {
            class: "relative z-0",
            input {
                r#type: "number",
                id: "input_month",
                class: "block py-2.5 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none out-of-range:border-red-500 dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                placeholder: " ",
                value: "{month}",
                min: 1,
                max: 12,
                required: true,
                onchange: move |evt| onchange.call(evt)
            }
            label {
                r#for: "input_month",
                class: "absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:left-0 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                "月"
            }
        }
    ))
}

#[inline_props]
pub fn InputEmployeeNum<'a>(
    cx: Scope,
    employee_num: String,
    onchange: EventHandler<'a, FormEvent>
) -> Element<'a> {
    cx.render(rsx!(
        div {
            class: "relative z-0",
            input {
                r#type: "number",
                id: "input_num",
                class: "block py-2.5 mb-5 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none out-of-range:border-red-500 dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                placeholder: " ",
                value: "{employee_num}",
                min: 2,
                max: 255,
                required: true,
                onchange: move |evt| onchange.call(evt)
            }
            label {
                r#for: "input_num",
                class: "absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:left-0 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                "人員"
            }
        }
    ))
}

#[inline_props]
pub fn InputEmployeeTimeframe<'a>(
    cx: Scope,
    required_people: Vec<String>,
    onchange_morning: EventHandler<'a, FormEvent>,
    onchange_afternoon: EventHandler<'a, FormEvent>,
    onchange_evening: EventHandler<'a, FormEvent>,
    onchange_f1: EventHandler<'a, FormEvent>,
    onchange_f2: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    let (
        morning,
        afternoon,
        evening,
        full1,
        full2,
    ) = (
        &required_people[0],
        &required_people[1],
        &required_people[2],
        &required_people[3],
        &required_people[4],
    );
    cx.render(rsx!(
        form {
            class: "space-y-3",
            h1 {
                class: "text-lg text-center font-medium pt-3 text-gray-900 dark:text-white",
                "時間帯ごとに必要な人数を入力してください。"
            }

            div {
                class: "relative z-0",
                input {
                    r#type: "number",
                    id: "input_morning",
                    class: "block py-2.5 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                    placeholder: " ",
                    value: "{morning}",
                    min: 0,
                    onchange: move |evt| onchange_morning.call(evt)
                }
                label {
                    r#for: "input_morning",
                    class: "absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:left-0 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                    "午前の必要人数"
                }
            }

            div {
                class: "relative z-0",
                input {
                    r#type: "number",
                    id: "input_afternoon",
                    class: "block py-2.5 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                    placeholder: " ",
                    value: "{afternoon}",
                    min: 0,
                    onchange: move |evt| onchange_afternoon.call(evt)
                }
                label {
                    r#for: "input_afternoon",
                    class: "absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:left-0 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                    "午後の必要人数"
                }
            }

            div {
                class: "relative z-0",
                input {
                    r#type: "number",
                    id: "input_evening",
                    class: "block py-2.5 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                    placeholder: " ",
                    value: "{evening}",
                    min: 0,
                    onchange: move |evt| onchange_evening.call(evt)
                }
                label {
                    r#for: "input_evening",
                    class: "absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:left-0 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                    "夜の必要人数"
                }
            }

            div {
                class: "relative z-0",
                input {
                    r#type: "number",
                    id: "input_full1",
                    class: "block py-2.5 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                    placeholder: " ",
                    value: "{full1}",
                    min: 0,
                    onchange: move |evt| onchange_f1.call(evt)
                }
                label {
                    r#for: "input_full1",
                    class: "absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:left-0 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                    "フルタイム(昼)の必要人数"
                }
            }

            div {
                class: "relative z-0",
                input {
                    r#type: "number",
                    id: "input_full2",
                    class: "block py-2.5 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer",
                    placeholder: " ",
                    value: "{full2}",
                    min: 0,
                    onchange: move |evt| onchange_f2.call(evt)
                }
                label {
                    r#for: "input_full2",
                    class: "absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:left-0 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6",
                    "フルタイム(夜)の必要人数"
                }
            }    
        }
    ))
}