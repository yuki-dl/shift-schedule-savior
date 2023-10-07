#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::roster::{
    employee::{Employee, Roster},
    timeframe::TimeFrame,
};

use super::button::{
    GenerateButton,
    ResetButton
};

#[inline_props]
pub fn DateTable(cx: Scope, days: u32, weekday_arr: Vec<String>) -> Element {
    cx.render(rsx!(
        thead {
            class: "text-lg sm:text-base text-gray-700 bg-gray-100 dark:bg-gray-700 dark:text-gray-400 uppercase",
            tr {
                th {
                    class: "sticky left-0 top-0 z-20 px-4 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", 
                    scope: "col", 
                    "ID"
                },
                th {
                    class: "sticky top-0 left-12 z-10 px-7 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", 
                    scope: "col", 
                    " "
                },
                (1..=*days).map(|i| rsx!{ th { class: "sticky top-0 p-2 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", scope: "col", i.to_string() } } )
            },
            tr {
                th {
                    class: "sticky top-10 left-0 z-10 bg-gray-100 dark:bg-gray-700 dark:text-gray-400",
                    scope: "col",
                    " "
                },
                th {
                    class: "sticky top-10 left-12 z-10 bg-gray-100 dark:bg-gray-700 dark:text-gray-400",
                    scope: "col", 
                    " "
                },
                weekday_arr.iter().map(|d| { rsx!( th {class: "sticky top-10 p-2 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", scope: "col", d.clone()} ) })
            }    
        }
    ))
}

#[inline_props]
pub fn CountTable(
    cx: Scope,
    counts: Vec<Vec<usize>>
) -> Element {
    let morning = counts.iter().map(|c| c[0]).collect::<Vec<_>>();
    let afternoon = counts.iter().map(|c| c[1]).collect::<Vec<_>>();
    let evening = counts.iter().map(|c| c[2]).collect::<Vec<_>>();

    cx.render(rsx!(
        tfoot {
            class: "font-semibold dark:text-gray-400",
            tr {
                th {
                    class: "sticky left-0 p-2 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", 
                    scope: "col", 
                    " "
                },
                th {
                    class: "sticky left-12 p-2 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", 
                    scope: "col", 
                    "午前"
                },
                morning.iter().map(|m| rsx!(th {class: "bg-indigo-50 border dark:bg-indigo-950 dark:text-gray-400 dark:border-gray-700", "{m}"}))
            }
            tr {
                th {
                    class: "sticky left-0 p-2 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", 
                    scope: "col", 
                    " "
                },
                th {
                    class: "sticky left-12 p-2 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", 
                    scope: "col", 
                    "午後"
                },
                afternoon.iter().map(|a| rsx!(td {class: "bg-indigo-50 border dark:bg-indigo-950 dark:text-gray-400 dark:border-gray-700", "{a}"}))
            }
            tr {
                th {
                    class: "sticky left-0 p-2 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", 
                    scope: "col", 
                    " "
                },
                th {
                    class: "sticky left-12 p-2 bg-gray-100 dark:bg-gray-700 dark:text-gray-400", 
                    scope: "col", 
                    "夜"
                },
                evening.iter().map(|e| rsx!(td {class: "bg-indigo-50 border dark:bg-indigo-950 dark:text-gray-400 dark:border-gray-700", "{e}"}))
            }
        }
    ))
}

#[inline_props]
pub fn EmployeeTable<'a>(
    cx: Scope,
    employee_num: String,
    days: u32, 
    weekday_arr: Vec<String>,
    required_people: Vec<String>,
    is_generated: bool,
    onclick: EventHandler<'a, MouseEvent>,
    onreset: EventHandler<'a, MouseEvent>,
) -> Element {
    if employee_num.is_empty() {
        return None;
    }

    let employees = create_default_table(employee_num.to_string(), *days as usize);
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
        let eg = Roster::new(employee_num, emp.clone(), required_people);
        let mut flag = true;

        let (results, counts) = use_memo(cx, (&eg,), |(mut eg,)| {
            flag = eg.create();
            (eg.employees, eg.sum)
        });

        log::info!("{counts:?}");

        cx.render(rsx!(
            GeneratedEmployeeTable {
                employees: results.clone(),
                counts: counts.clone(),
                days: *days,
                weekday_arr: weekday_arr.to_vec(),
                flag: flag,
                onreset: move |evt| onreset.call(evt),
                r: r
            }
        ))
    } else {
        cx.render(rsx!(
            div {
                h1 {
                    class: "text-xl sm:text-lg text-center font-medium pt-3 pb-3 pr-3 text-gray-900 dark:text-gray-300",
                    "希望休を入力してください。"
                    GenerateButton {
                        is_empty: required_people.iter().any(|r| r.is_empty()),
                        onclick: move |evt| onclick.call(evt),
                    }
                }
                div {
                    class: "flex flex-col h-72 w-full overflow-auto",
                    table {
                        class: "w-full text-center text-gray-500 border-collapse border dark:text-gray-400 dark:border-gray-700",
                        DateTable { days: *days, weekday_arr: weekday_arr.to_vec() }
                        tbody {
                            emp
                            .iter()
                            .enumerate()
                            .map(|(i, e)| rsx!(
                                tr {
                                    class: "bg-white border dark:bg-gray-800 dark:border-gray-700",
                                    th {
                                        class: "sticky left-0 p-2 bg-gray-100 dark:bg-gray-700 dark:text-gray-400",
                                        "{e.id + 1}"
                                    }
                                    th {
                                        class: "sticky left-12 p-2 bg-gray-100 dark:bg-gray-700 dark:text-gray-400",
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
    counts: Vec<Vec<usize>>,
    days: u32, 
    weekday_arr: Vec<String>,
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
                        class: "text-xl sm:text-lg text-center font-medium pt-3 pb-3 pr-3 text-gray-900 dark:text-gray-300",
                        "シフト表が「ほぼ」仕上がりました！"
                        ResetButton {
                            onclick: move |evt| onreset.call(evt)
                        }
                    }
                )
            } else {
                rsx!(
                    h1 {
                        class: "text-xl sm:text-lg text-center font-medium pt-3 pb-3 pr-3 text-red-600 dark:text-red-400",
                        "設定(Settings)が間違えています。"
                        ResetButton {
                            onclick: move |evt| onreset.call(evt)
                        }
                    }        
                )
            }
            div {
                class: "flex flex-col h-72 w-full overflow-auto",
                table {
                    class: "w-full text-center text-gray-500 border-collapse border dark:text-gray-400 dark:border-gray-700",
                    DateTable { days: *days, weekday_arr: weekday_arr.to_vec() }
                    tbody {
                        employees
                        .iter()
                        .map(|e| rsx!(
                            tr {
                                class: "bg-white border dark:bg-gray-800 dark:border-gray-700",
                                th {
                                    class: "sticky left-0 p-2 bg-gray-100 dark:bg-gray-700 dark:text-gray-400",
                                    "{e.id + 1}"
                                },
                                th {
                                    class: "sticky left-12 p-2 bg-gray-100 dark:bg-gray-700 dark:text-gray-400",
                                    "{e.timeframe.get()}"
                                },
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
                    CountTable {counts: counts.to_vec()} 
                }
            }
        }
    ))
}

fn create_default_table(employee_num: String, days: usize) -> Vec<Employee> {
    // 人数を入力すると、作成
    let employee_num = employee_num.parse::<usize>().unwrap();
    (0..employee_num).map(|i| {
        Employee::new(i, TimeFrame::Full1, vec![1u8; days])
    }).collect::<Vec<_>>()
}
