use super::timeframe::TimeFrame;
use super::solve::solve;

#[derive(Debug, Clone, PartialEq)]
pub struct Employee {
    pub id: usize,
    pub timeframe: TimeFrame,
    pub day_off: Vec<u8>
}
impl Employee {
    pub fn new(id: usize, timeframe: TimeFrame, day_off: Vec<u8>) -> Self {
        Self { id, timeframe, day_off }
    }
    fn update(&self, timeframe: TimeFrame, day_off: Vec<u8>) -> Self {
        Self { timeframe, day_off, ..self.clone() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmployeeGroup {
    employee_num: usize,
    employees: Vec<Employee>,
    required_people: Vec<usize>,
    pub results: Option<Vec<Employee>>
}
impl EmployeeGroup {
    pub fn new(
        employee_num: &String,
        employees: Vec<Employee>,
        required_people: &Vec<String>
    ) -> Self {
        let employee_num = employee_num.parse::<usize>().unwrap();
        let required_people = required_people
            .iter()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self { employee_num, employees, required_people, results: None }
    }
    pub fn create(&mut self) -> bool {
        let g = assign_group(self.employees.clone());
        let chunk_size = self.employees[0].day_off.len();
        let people = g.iter().map(|e| e.len()).collect::<Vec<_>>();

        // test1
        let t1 = self.required_people.iter()
            .map(|r| (0..self.employee_num).contains(&r))
            .all(|b| b == true);
        if !t1 { return false; }

        // test2
        let full = self.required_people[3..].to_vec();
        let t2 = self.required_people[0..3].iter()
            .map(|r| {
                full.iter().map(|f| *r >= *f).all(|b| b == true)
            }).all(|b| b == true);
        if !t2 { return false; }

        // test3
        let t3 = full.iter().sum::<usize>() <= self.required_people[1];
        if !t3 { return false; }
        
        // test4
        let mut required_people = self.required_people.clone();

        required_people[0] -= full[0];
        required_people[1] -= full[0] + full[1];
        required_people[2] -= full[1];
        
        let t4 = required_people.iter()
            .zip(people.iter())
            .all(|(r, s)| *r <= *s);
        if !t4 { return false; }

        // main
        let mut results = Vec::with_capacity(self.employee_num);
        for (emps, r) in g.iter().zip(required_people) {
            if r == 0 || emps.is_empty() {
                continue;
            }
            let ind = solve(emps, r);
            let timeframe = &emps[0].timeframe;          
            let day_offs = ind
                .chunks(chunk_size)
                .map(|d| d.to_vec())
                .collect::<Vec<_>>();

            for (emp, day_off) in emps.iter().zip(day_offs) {
                let emp = emp.update(timeframe.clone(), day_off);
                results.push(emp);
            }
        }
        results.sort_by_key(|e| e.id);
        self.results = Some(results);

        true
    }
}

pub fn assign_group(employees: Vec<Employee>) -> Vec<Vec<Employee>> {
    // [morning, afternoon, evening, full1, full2]
    let mut results = vec![vec![]; 5];

    for emp in employees {
        match emp.timeframe {
            TimeFrame::Morning => results[0].push(emp),
            TimeFrame::Afternoon => results[1].push(emp),
            TimeFrame::Evening => results[2].push(emp),
            TimeFrame::Full1 => results[3].push(emp),
            TimeFrame::Full2 => results[4].push(emp)
        }
    }
    results
}
