#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::crates::{
    employee::{Employee, EmployeeGroup},
    timeframe::TimeFrame,
    weekday::get_weekday_arr,
};

use super::button::{
    GenerateButton,
    ResetButton
};

#[inline_props]
pub fn DateTable(cx: Scope, month: String) -> Element {
    cx.render(rsx!(
        thead {
            class: "text-base text-gray-700 bg-gray-100 dark:bg-gray-700 dark:text-gray-400 uppercase",
            tr {
                th {
                    class: "sticky left-0 top-0 z-10 px-3 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", 
                    scope: "col", 
                    "ID"
                },
                th {
                    class: "sticky top-0 px-7 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", 
                    scope: "col", 
                    " "
                },
                (1..=31).map(|i| rsx!{ th { class: "sticky top-0 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", scope: "col", i.to_string() } } )
            },
            ArrangeWeekday {month: month.to_string()}
        }
    ))
}

#[inline_props]
fn ArrangeWeekday(cx: Scope, month: String) -> Element {
    let weekday_arr = get_weekday_arr(month);
    cx.render(rsx!(
        tr {
            th {
                class: "sticky left-0 bg-gray-100 dark:bg-gray-700 dark:text-gray-400",
                scope: "col",
                " "
            },
            th {
                scope: "col", 
                " "
            },
            weekday_arr.iter().map(|d| {
                rsx!( th {class: "p-2", scope: "col", d.clone()} )
            })
        }
    ))
}

#[inline_props]
pub fn EmployeeTable<'a>(
    cx: Scope,
    employee_num: String,
    month: String,
    required_people: Vec<String>,
    is_generated: bool,
    onclick: EventHandler<'a, MouseEvent>,
    onreset: EventHandler<'a, MouseEvent>,
) -> Element {
    if employee_num.is_empty() {
        return None;
    }

    let employees = create_default_table(employee_num.to_string());
    let employees_state = use_state(cx, || employees.clone());
    let emp = employees_state.get();

    let r = |n: u8| {
        match n {
            0 => "○".to_string(),
            1 => " ".to_string(),
            _ => unreachable!()
        }
    };

    if *is_generated {
        let eg = EmployeeGroup::new(employee_num, emp.clone(), required_people);
        let mut flag = true;

        let results = use_memo(cx, (&eg,), |(mut eg,)| {
            eg.create();
            eg.results
        });

        let results = if results.is_none() {
            flag = false;
            employees_state.get().clone()
        } else {results.clone().unwrap()};

        cx.render(rsx!(
            GeneratedEmployeeTable {
                employees: results.clone(),
                month: month.to_string(),
                flag: flag,
                onreset: move |evt| onreset.call(evt),
                r: r
            }
        ))
    } else {
        cx.render(rsx!(
            div {
                h1 {
                    class: "text-lg text-center font-medium pt-3 pb-3 pr-3 text-gray-900 dark:text-white",
                    "希望休を入力してください。"
                    GenerateButton {
                        is_empty: required_people.iter().any(|r| r.is_empty()),
                        onclick: move |evt| onclick.call(evt),
                    }
                }
                div {
                    class: "flex flex-col h-72 w-96 overflow-auto",
                    table {
                        class: "w-full text-center text-gray-500 border-collapse border dark:text-gray-400 dark:border-gray-700",
                        DateTable { month: month.to_string() }
                        tbody {
                            emp
                            .iter()
                            .enumerate()
                            .map(|(i, e)| rsx!(
                                tr {
                                    class: "bg-white border dark:bg-gray-800 dark:border-gray-700",
                                    th {
                                        class: "sticky left-0 bg-gray-100 dark:bg-gray-700 dark:text-gray-400",
                                        "{e.id + 1}"
                                    }
                                    th {
                                        class: "p-2",
                                        onclick: move |_| {
                                            let mut new = emp.clone();
                                            let mut new_t = new[i].timeframe.clone();
                                            new_t = new_t.shift();
                                            new[i].timeframe = new_t;
                                            employees_state.set(new);
                                        },
                                        "{e.timeframe.get()}"
                                    }
                                    e.day_off
                                        .iter()
                                        .enumerate()
                                        .map(|(j, d)| rsx!(
                                            td {
                                                class: "border dark:border-gray-700",
                                                onclick: move |_| {
                                                    let mut new = emp.clone();
                                                    let mut new_day_off = new[i].day_off.clone();
                                                    new_day_off[j] = 1 - new_day_off[j];
                                                    new[i].day_off = new_day_off;
                                                    employees_state.set(new);
                                                },
                                                "{r(*d)}"    
                                            }
                                        ))
                                }
                            ))
                        }    
                    }
                }
            }
        ))
    }
}

#[inline_props]
pub fn GeneratedEmployeeTable<'a, F>(
    cx: Scope,
    employees: Vec<Employee>,
    month: String,
    flag: bool,
    onreset: EventHandler<'a, MouseEvent>,
    r: F
) -> Element 
    where F: Fn(u8) -> String
{
    cx.render(rsx!(
        div {
            if *flag {
                rsx!(
                    h1 {
                        class: "text-lg text-center font-medium pt-3 pb-3 pr-3 text-gray-900 dark:text-white",
                        "シフト表が「ほぼ」仕上がりました！"
                        ResetButton {
                            onclick: move |evt| onreset.call(evt)
                        }
                    }
                )
            } else {
                rsx!(
                    h1 {
                        class: "text-lg text-center font-medium pt-3 pb-3 pr-3 text-gray-900 dark:text-white",
                        "希望休を入力してください。"
                        ResetButton {
                            onclick: move |evt| onreset.call(evt)
                        }
                    }        
                    div {
                        class: "text-center",
                        span {
                            class: "text-sm font-medium pt-3 pb-3 text-red-600 dark:text-red-400",
                            "設定(Settings)が間違っています。もう一度入力してください。"
                        }
                    }
                )
            }
            div {
                class: "flex flex-col h-72 w-96 overflow-auto",
                table {
                    class: "w-full text-center text-gray-500 border-collapse border dark:text-gray-400 dark:border-gray-700",
                    DateTable { month: month.to_string() }
                    tbody {
                        employees
                        .iter()
                        .map(|e| rsx!(
                            tr {
                                class: "bg-white border dark:bg-gray-800 dark:border-gray-700",
                                th {
                                    class: "sticky left-0 bg-gray-100 dark:bg-gray-700 dark:text-gray-400",
                                    "{e.id + 1}"
                                },
                                th { class: "p-2", "{e.timeframe.get()}" },
                                e.day_off
                                    .iter()
                                    .map(|d| rsx!(
                                        td {
                                            class: "border dark:border-gray-700",
                                            "{r(*d)}"
                                        }
                                    ))
                            }
                        ))
                    }    
                }
            }
        }
    ))
}

fn create_default_table(employee_num: String) -> Vec<Employee> {
    // 人数を入力すると、作成
    let employee_num = employee_num.parse::<usize>().unwrap();
    (0..employee_num).map(|i| {
        Employee::new(i, TimeFrame::Full1, vec![1u8; 31])
    }).collect::<Vec<_>>()
}
