pub mod app;
pub mod components {
    pub mod card;
    pub mod form;
    pub mod tab;
    pub mod table;
    pub mod button;
    pub mod header;
    pub mod footer;
}
pub mod ga {
    pub mod build;
    pub mod solve;
}
pub mod roster {
    pub mod employee;
    pub mod timeframe;
    pub mod weekday;
}

// debug
pub fn run() {
    use roster::employee::{Employee, Roster};
    use roster::timeframe::TimeFrame;

    let employees = vec![
        Employee::new(1, TimeFrame::Full1, vec![1u8; 31]),
        Employee::new(2, TimeFrame::Full1, vec![1u8; 31]),
        Employee::new(3, TimeFrame::Full1, vec![1u8; 31]),
        Employee::new(4, TimeFrame::Full1, vec![1u8; 31]),
        Employee::new(5, TimeFrame::Full1, vec![1u8; 31]),
        Employee::new(6, TimeFrame::Full1, vec![1u8; 31]),
        Employee::new(7, TimeFrame::Morning, vec![1u8; 31]),
        Employee::new(8, TimeFrame::Morning, vec![1u8; 31]),
        Employee::new(9, TimeFrame::Morning, vec![1u8; 31]),
        Employee::new(10, TimeFrame::Morning, vec![1u8; 31]),
        Employee::new(11, TimeFrame::Morning, vec![1u8; 31]),
        Employee::new(12, TimeFrame::Morning, vec![1u8; 31]),
        Employee::new(13, TimeFrame::Morning, vec![1u8; 31]),
        Employee::new(14, TimeFrame::Full2, vec![1u8; 31]),
        Employee::new(15, TimeFrame::Full2, vec![1u8; 31]),
        Employee::new(16, TimeFrame::Evening, vec![1u8; 31]),
        Employee::new(17, TimeFrame::Evening, vec![1u8; 31]),
        Employee::new(18, TimeFrame::Evening, vec![1u8; 31]),
        Employee::new(19, TimeFrame::Evening, vec![1u8; 31]),
        Employee::new(20, TimeFrame::Evening, vec![1u8; 31]),
        Employee::new(21, TimeFrame::Evening, vec![1u8; 31]),
        Employee::new(22, TimeFrame::Evening, vec![1u8; 31]),
    ];
    let employee_num = employees.len().to_string();

    let required_people = vec![
        8.to_string(),
        5.to_string(),
        4.to_string(),
    ];
    
    let start = std::time::Instant::now();

    let mut eg = Roster::new(&employee_num, employees, &required_people);
    eg.create();
    eg.employees.iter().for_each(|e| println!("{e:?}"));
    println!("{:?}", eg.sum);

    let end = start.elapsed();
    println!("processing time: {} s {} ms", end.as_secs(), end.subsec_millis());

}
